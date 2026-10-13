# nxd-rust-frontend

Development repository for the NXD Rust frontend.

This project reimplements the existing Python lexer, parser, and AST pipeline from the main NXD compiler in Rust.

The current Python frontend serves as the reference implementation while functionality is ported and validated through parity testing.

## Goals

- Rust lexer
- Rust parser
- Rust AST
- JSON AST compatibility
- Feature parity with the Python frontend
- Preservation of current compiler semantics and diagnostics

## Status

Experimental.

This repository is not intended for production use. Once feature parity and stability are achieved, the Rust frontend will be merged into the main `nxd_language` repository.

## Validation Strategy

The Rust frontend is validated against the existing NXD test corpus to ensure matching parser output and compiler behavior.
