mod backend;
mod frontend;
mod ir;
mod orchestrator;
mod semantic;

use crate::ir::compiler::{compile_to_ir, compile_to_ir_json};

use std::env;
use std::fs;

fn read_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|error| format!("Failed to read source file '{}': {}", path, error,))
}

fn build_ir(path: &str) -> Result<ir::nodes::IRRoot, String> {
    let source = read_source(path)?;

    compile_to_ir(&source).map_err(|error| format!("Failed to compile source to IR: {}", error,))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
     * Semantic diagnostics:
     *
     * cargo run -- --semantic-diagnostics input.nxd
     */
    if args.len() == 3 && args[1] == "--semantic-diagnostics" {
        let ir_root = match build_ir(&args[2]) {
            Ok(ir_root) => ir_root,

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        let response = orchestrator::semantic_diagnostics_from_ir(&ir_root);

        match serde_json::to_string(&response) {
            Ok(json) => {
                println!("{}", json);
            }

            Err(error) => {
                eprintln!("Failed to serialize semantic diagnostics: {}", error,);

                std::process::exit(1);
            }
        }

        return;
    }

    /*
     * Definition lookup:
     *
     * cargo run -- --definition input.nxd SYMBOL
     */
    if args.len() == 4 && args[1] == "--definition" {
        let ir_root = match build_ir(&args[2]) {
            Ok(ir_root) => ir_root,

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        let definition = orchestrator::definition_from_ir(&ir_root, &args[3]);

        match definition {
            Ok(result) => {
                println!("{}", result);
            }

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }

        return;
    }

    /*
     * Optional IR JSON debug output:
     *
     * cargo run -- --emit-ir-json input.nxd output.json
     */
    if args.len() == 4 && args[1] == "--emit-ir-json" {
        let source = match read_source(&args[2]) {
            Ok(source) => source,

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        let json = match compile_to_ir_json(&source) {
            Ok(json) => json,

            Err(error) => {
                eprintln!("Failed to generate IR JSON: {}", error,);

                std::process::exit(1);
            }
        };

        if let Err(error) = fs::write(&args[3], json) {
            eprintln!("Failed to write IR JSON file '{}': {}", args[3], error,);

            std::process::exit(1);
        }

        println!("IR JSON generation successful");

        return;
    }

    /*
     * Normal compilation:
     *
     * cargo run -- input.nxd output.nim
     *
     * Compilation with semantic validation:
     *
     * cargo run -- --semantics input.nxd output.nim
     */
    let use_semantics = args.len() == 4 && args[1] == "--semantics";

    let valid_normal_command = args.len() == 3;

    let valid_semantic_command = args.len() == 4 && use_semantics;

    if !valid_normal_command && !valid_semantic_command {
        eprintln!("Usage:");
        eprintln!("  cargo run -- <input.nxd> <output.nim>");
        eprintln!("  cargo run -- --semantics <input.nxd> <output.nim>");
        eprintln!("  cargo run -- --semantic-diagnostics <input.nxd>");
        eprintln!("  cargo run -- --definition <input.nxd> <symbol>");
        eprintln!("  cargo run -- --emit-ir-json <input.nxd> <output.json>");

        std::process::exit(1);
    }

    let (input_path, output_path) = if use_semantics {
        (&args[2], &args[3])
    } else {
        (&args[1], &args[2])
    };

    let ir_root = match build_ir(input_path) {
        Ok(ir_root) => ir_root,

        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    let result = if use_semantics {
        orchestrator::compile_from_ir_with_semantics(&ir_root)
    } else {
        orchestrator::compile_from_ir(&ir_root)
    };

    match result {
        Ok(nim_code) => {
            if let Err(error) = fs::write(output_path, nim_code) {
                eprintln!(
                    "Failed to write Nim output file '{}': {}",
                    output_path, error,
                );

                std::process::exit(1);
            }

            if use_semantics {
                println!("Semantic validation successful");
            }

            println!("Compilation successful");
        }

        Err(error) => {
            eprintln!("Compilation failed: {}", error,);

            std::process::exit(1);
        }
    }
}
