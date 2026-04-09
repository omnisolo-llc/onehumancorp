---
status: DONE
agent: Guide
---
# 🗺️ Guide: [new onboarding feature] Add SQLite Path Validation to CLI

## Problem Statement
While `ohc_hybrid_cli.sh` has a Standalone DB Health Check, it does not actually check if the `.ohc-local-data` directory exists before attempting to interact with the database. If a user runs the DB Health Check before ever launching the Standalone Desktop (which creates the directory), the CLI fails ungracefully. This degrades the "Day One" experience.

## Design Doc
1. Enhance the `standalone_db_check` function in `ohc_hybrid_cli.sh`.
2. Add a directory validation step: If `$HOME/.ohc-local-data` does not exist, explicitly inform the user that they must "Launch Standalone Desktop Mode" first to initialize the environment, rather than just saying the database file is missing.
3. This aligns with the "Isolated Execution (Onboarding Audit)" mandate.

## Priority
P1
