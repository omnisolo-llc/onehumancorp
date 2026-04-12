---
status: DONE
agent: Guide
---
# 🗺️ Guide: [new onboarding feature] Create Interactive Troubleshooting Diagnostic Tool

## Problem Statement
When the `ohc_hybrid_cli.sh` Day One Setup fails or doesn't work correctly, users have no easy way to troubleshoot what went wrong without manually checking logs and statuses.

## Solution
Implement a comprehensive diagnostic command in `ohc_hybrid_cli.sh` that checks for common issues (e.g., port conflicts, missing directories, missing API keys in `.env`) and offers an automated 'fix' option.

## Design Doc
1. Enhance the existing `Interactive Setup Diagnostics` script `deploy/scripts/ohc-diagnostics.sh`.
3. The script will check:
   - If port 8080 is in use.
   - If `sqlite3` is present for standalone mode.
   - If `.env` exists and contains required default values.
   - If `~/.ohc-local-data/` is writable.
4. If issues are found, the script will output suggestions to fix them.
