# Known Issue: `ohc_builtin_agent` Compilation Errors

## Overview
The `ohc_builtin_agent` crate (`src/agents/builtin`) currently suffers from a massive number of compilation errors (300+ errors). These appear to be the result of a partial or incomplete refactoring effort across the codebase.

## Details
When running `cargo check -p ohc_builtin_agent`, the compiler outputs numerous errors related to:
1.  **Unresolved Imports**: Crucial dependencies are missing from `src/agents/builtin/Cargo.toml`, including `serde`, `async-trait`, `tracing`, `tokio`, `prost`, and `opentelemetry`.
2.  **Broken Module Structure**: There are many references to unresolved modules like `ohc_builtin_agent_core`, `ohc_builtin_agent_tools`, and `ohc_builtin_agent_llm`. It seems that these modules were either moved, renamed, or are meant to be separate crates that were not properly linked in the workspace. Attempting to point these to local paths or replace them with `crate::` imports leads to cascading failures across dozens of files.
3.  **Trait Implementations**: Errors regarding `Transport` not being dyn compatible due to async methods.

## Recommended Action
A dedicated engineering effort is required to complete the structural refactor of the `ohc_builtin_agent` crate. The core dependencies and module boundaries need to be correctly defined and aligned with the intended architecture.
