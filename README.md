# nxd-rust-frontend

**Status: Archived**

This repository previously served as the development and validation environment for migrating the NXD compiler frontend from Python to Rust.

The migration is now complete.

The completed Rust frontend and Rust compiler backend have been transferred into the primary `nxd_language` repository. Active Rust Language Server Protocol and editor tooling development is maintained separately in `nxd-extension`.

This repository now serves as an archival and architectural reference for the NXD compiler migration.

## Branches

This repository preserves two important stages of NXD's compiler architecture:

- **`main`** preserves the migration-era architecture, including the original Python frontend, Rust backend, and Python-to-JSON-to-Rust compiler handoff.
- **`current`** provides a later architecture reference containing the Rust frontend, Rust backend, and Rust Language Server Protocol implementation.

The `current` branch exists so that the completed Rust architecture can be studied directly alongside the historical implementation without requiring multiple repositories to reconstruct the migration.

Neither branch is the authoritative development location for the current NXD compiler or editor tooling.

For active development:

- `nxd_language` contains the current compiler and language implementation.
- `nxd-extension` contains the current language server and editor/IDE tooling.

---

## Migration Result

The NXD compiler migration successfully completed the following:

- Rust scanner
- Rust parser
- Rust AST
- Rust IR lowering
- Native Rust IR construction
- Rust semantic integration
- Rust backend integration
- Rust diagnostic integration
- Rust Language Server Protocol implementation
- Frontend behavioral parity
- Validation-suite parity
- Removal of the active Python-to-JSON-to-Rust handoff

The active NXD compiler is now implemented as a unified Rust pipeline:

`NXD Source \u2192 Rust Scanner \u2192 Rust Parser \u2192 Rust AST \u2192 Rust Lowering \u2192 IR Root \u2192 NXD Semantics \u2192 Backend Transpilation`

JSON output may still be generated for debugging, inspection, testing, and compiler analysis, but JSON serialization and rehydration are no longer required as the internal handoff between the frontend and semantic backend.

---

## Repository Purpose

This repository is no longer the active development location for the NXD Rust frontend.

It now preserves multiple stages of the NXD compiler's architectural evolution.

The `main` branch provides a historical record of the migration-era implementation, including:

- The former Python frontend
- The previous Python-to-JSON-to-Rust compiler pipeline
- Earlier Rust compiler and backend components
- Previous AST, IR, semantic, and diagnostic structures
- A reference point for regression analysis
- A comparison point for future compiler validation
- Historical context for the completed Rust migration

The `current` branch provides a reference snapshot of the later Rust architecture, including:

- The Rust frontend
- The Rust backend
- The Rust Language Server Protocol implementation
- Native Rust IR construction
- Unified compiler and diagnostic infrastructure

Together, the branches allow the earlier and later compiler architectures to be inspected and compared within a single repository.

The contents of this repository should not be assumed to represent the current active NXD compiler architecture.

---

## Migration Background

Before the migration, NXD used a Python frontend and a Rust compiler backend connected through a JSON IR handoff:

`Python Scanner \u2192 Python Parser \u2192 Python AST \u2192 Python Lowering \u2192 IR JSON \u2192 Rust Loader \u2192 IR Root \u2192 NXD Semantics \u2192 Backend Transpilation`

This architecture enabled the initial development of the NXD language, but it also required compiler structures to be maintained across Python and Rust.

The serialized handoff introduced additional:

- Data structure duplication
- Serialization and deserialization
- Frontend and backend synchronization
- Cross-language debugging
- IR schema maintenance
- Diagnostic propagation complexity

The `nxd-rust-frontend` repository was created as an isolated migration environment where the frontend could be rewritten and validated without disrupting the working implementation in `nxd_language`.

---

## Migration Goals

The migration was developed around the following goals:

- Reimplement the NXD scanner in Rust
- Reimplement the NXD parser in Rust
- Reimplement the NXD AST in Rust
- Reimplement AST-to-IR lowering in Rust
- Preserve existing NXD language behavior
- Preserve compiler semantics
- Preserve diagnostic behavior
- Maintain compatibility with the existing semantic analyzer
- Maintain compatibility with the existing backend
- Validate the Rust implementation against the NXD test corpus
- Remove the active JSON rehydration boundary
- Preserve JSON as an optional debugging artifact
- Migrate the NXD Language Server Protocol implementation to Rust
- Establish behavioral parity with the Python frontend
- Transfer the completed implementation into `nxd_language`

These goals have been completed.

---

## Validation Strategy

During migration, the Rust implementation was tested against the existing NXD validation corpus.

Validation focused on preserving:

- Scanner behavior
- Token production
- Parser behavior
- AST construction
- IR lowering
- IR structure
- Source span propagation
- Warning generation
- Parser diagnostics
- Semantic diagnostics
- Backend integration
- Positive test behavior
- Negative test behavior
- Specialty backend behavior
- Language Server Protocol behavior

The former Python implementation served as the behavioral reference while each compiler stage was migrated.

After parity was achieved, the completed Rust implementation became the active compiler in the primary `nxd_language` repository.

---

## Language Server Migration

The NXD Language Server Protocol implementation was also migrated to Rust.

The resulting editor diagnostic pipeline follows the unified Rust architecture:

`Editor \u2192 Rust Language Server \u2192 Rust Compiler Frontend \u2192 IR Root \u2192 NXD Semantics \u2192 Diagnostics \u2192 Editor Response`

This allows the command-line compiler and editor tooling to use the same frontend, semantic analyzer, source spans, and diagnostic infrastructure.

The LSP originally lived alongside compiler development during the migration. It is included in the `current` branch to preserve that architectural stage as a self-contained reference.

Active LSP and editor integration development has since moved to the dedicated `nxd-extension` repository.

---

## Current Development

Active NXD development no longer occurs in this archived repository.

The authoritative compiler implementation is maintained in `nxd_language`, including:

- Rust scanner
- Rust parser
- Rust AST
- Rust lowering pipeline
- Rust IR
- Semantic analyzer
- Diagnostic infrastructure
- Compiler orchestration
- Backend implementations
- Validation suites

Active editor, IDE, and Language Server Protocol development is maintained in `nxd-extension`.

This archived repository should not receive active compiler or language-server features unless required for historical preservation, archival corrections, or documentation maintenance.

The `current` branch is a reference branch, not a replacement for the active development repositories.

---

## Comparing the Architectures

The two preserved branches can be inspected directly:

```bash
git switch main
```
