# nxd-rust-frontend

Status: Active Migration

This repository contains the ongoing Rust frontend rewrite of the NXD compiler.

The Python frontend (in the nxd_language repository) remains the reference implementation during migration and parity testing.

Current status:

- Lexer complete
- Parser complete
- AST complete
- IR migration in progress
- Builds successfully
- Tests passing

Development repository for the NXD Rust frontend.

This project reimplements the existing Python lexer, parser, IR, and AST pipeline from the main NXD compiler in Rust.

The current Python frontend serves as the reference implementation while functionality is ported and validated through parity testing.

## Background

NXD currently uses a Python frontend and a Rust compiler core connected through
a JSON IR handoff. While this architecture enabled rapid language development,
it also requires maintaining compiler structures across language boundaries and
introduces additional serialization, debugging, and synchronization overhead.

The goal of this project is to evaluate a fully Rust-based frontend while
preserving existing language semantics and compiler behavior.

## Goals

- Rust lexer
- Rust parser
- Rust AST
- Rust IR
- JSON IR compatibility
- Feature parity with the Python frontend
- Preservation of current compiler semantics and diagnostics

## Status

Experimental.

This repository is a frontend migration and validation environment.
The Python frontend in the `nxd_language` repository remains the reference implementation until feature parity and stability have been demonstrated. Once stability has been demonstrated, the new code will be migrated into the `nxd_language` repository to be a replacement implementation for the current Python frontend. 

## Validation Strategy

The Rust frontend is validated against the existing NXD test corpus to ensure matching parser output and compiler behavior.
