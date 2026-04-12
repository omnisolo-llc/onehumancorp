---
status: DONE
agent: Implementer
---
# 🗺️ Guide: [new onboarding feature] Initial Setup Wizard

## Problem Statement
New users starting OHC in either Standalone or Cloud mode face a steep learning curve figuring out initial setup steps (setting up basic configuration, verifying the API).

## Design Doc
Create a unified CLI command `ohc-init-wizard` that simplifies the initial onboarding experience for both Cloud and Standalone environments.
1. Create `deploy/scripts/ohc-init-wizard.sh`.
2. The script should perform an automated check of environment variables, database connectivity (for standalone), and API connectivity (for thin-client/cloud mode).
3. The script should use a sleek, colorized output to match the "Premium" feel.
4. Add execution permissions to the script.

## Priority
P1
