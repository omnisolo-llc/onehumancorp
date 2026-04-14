---
status: PENDING
agent: Guide
---

# 🗺️ Guide: [new onboarding feature] Interactive Standalone SQLite Onboarding

## Problem Statement
While the CLI wizard provides a good starting point, many users struggle with initializing the SQLite SIPDB for Standalone Desktop mode when `DATABASE_URL` is omitted. The fallback behavior works, but lacks a guided "Day One" onboarding confirmation that their local DB is correctly provisioned, leading to silent failures when RAG vector search attempts to run.

## Research Report
The OHC Hybrid Architecture seamlessly falls back to SQLite. However, if the `db_path` is misconfigured or missing permissions, agents will silently error on memory writes. We need to add an interactive check in the CLI setup.

## Design Doc
1.  **Enhance `ohc_hybrid_cli.sh`**: Add a function `check_sqlite()` that runs during the "Run Initial Setup" option.
2.  **Implementation Details**:
    - The function should check if `sqlite3` is installed.
    - It should attempt to create a dummy database in the configured `db_path` (defaulting to `.sipdb/` in the local directory) to verify write permissions.
    - Output premium-styled confirmation messages (e.g., using `${GREEN}` and `${BOLD}`).
3.  **Update README.md**: Add a note about SQLite Standalone requirement.

## Implementation Prompt
Hello Implementer agent! Please add the `check_sqlite()` functionality to `ohc_hybrid_cli.sh` and update `README.md`. Verify the script runs without syntax errors.

## Priority
P1

## Estimated Scope
Small
