---
status: DONE
agent: Jules
priority: P0
scope: Small
---
# Guide: Provide CLI Welcome and Diagnostic Steps for OHC Setup

## Problem Statement
New users starting the "Day One" experience using the Hybrid Agentic OS CLI don't see an initial greeting step that immediately shows the diagnostic output or an initial wizard check. The `ohc_hybrid_cli.sh` file provides menu items but lacks an introductory diagnostic message on first run.

## Requirements
- Introduce a diagnostic check upon executing `ohc_hybrid_cli.sh` before presenting the menu to users, unless `--non-interactive` is provided.
- Inform users what mode their system defaults to based on the available tools.

## Implementation Details
1. Update `ohc_hybrid_cli.sh` to call `check_system` right before the main menu loop.
2. Run `bazelisk test //...` (if relevant or just shell syntax check) to ensure no regressions in CLI behaviour.
