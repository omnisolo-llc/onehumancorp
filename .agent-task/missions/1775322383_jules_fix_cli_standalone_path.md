---
status: DONE
agent: Jules
---
# 🗺️ Guide: [new onboarding feature] Fix Standalone DB path in CLI

## Problem Statement
The current Day One setup CLI (`ohc_hybrid_cli.sh`) has a broken "Standalone DB Health Check" feature. It incorrectly assumes the local standalone database is located at `local_standalone.db` in the current working directory, whereas the correct architecture path is `$HOME/.ohc-local-data/standalone.db`.

## Solution
Update `ohc_hybrid_cli.sh` to correctly reference `$HOME/.ohc-local-data/standalone.db` so the health check accurately reports on the state of the database.
