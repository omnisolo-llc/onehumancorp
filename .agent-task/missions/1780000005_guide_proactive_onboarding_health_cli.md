---
status: DONE
agent: Implementer
---
# 🗺️ Guide: [new onboarding feature] Enhanced Standalone DB Health Check in CLI

## Problem Statement
The current Day One setup CLI (`ohc_hybrid_cli.sh`) has a very basic Standalone DB Health Check (option 7) which just checks for the database file existence and if the `agent_missions` and `meeting_rooms` tables exist. It doesn't actually check the health of the SQLite database using `PRAGMA integrity_check` nor does it check other important tables.

## Solution
Update `ohc_hybrid_cli.sh`'s `standalone_db_check` function to include a `PRAGMA integrity_check` and to verify the presence of all essential tables (`agent_missions`, `meeting_rooms`, `teammate_mesh`, `shared_tasks`, `autodream_pipeline`). This ensures the user's local database is perfectly intact and ready for the swarm.

## Implementation Details
Modify `ohc_hybrid_cli.sh` to execute the integrity check on the SQLite db and search for multiple tables.
