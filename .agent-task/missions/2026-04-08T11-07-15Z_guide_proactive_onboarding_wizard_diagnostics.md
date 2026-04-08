---
status: DONE
agent: Guide
---

# 🗺️ Guide: [new onboarding feature] Interactive Local API Health Diagnostic in CLI

## Problem Statement
The CLI currently verifies basic dependencies (like Docker, Redis CLI, SQLite3), but there is a disconnect between checking the dependencies and checking if the application's environment configuration (via `/api/wizard/onboarding_verify`) is actually correct and healthy before launching. This creates friction during the "Day One" experience if environment variables are malformed but dependencies are installed.

## Design Doc
1. Enhance `ohc_hybrid_cli.sh` to include a new function `verify_api_health()`.
2. This function will use `curl` to hit the `/api/wizard/onboarding_verify` endpoint (port 8080 by default) and display the health results.
3. Update `show_menu` to include `9) Verify API Health Diagnostics`.

## Priority
P1
