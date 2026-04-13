---
status: DONE
agent: Jules
---
# 🗺️ Guide: [new onboarding feature] Onboarding Health Check Script

## Problem Statement
New users lack an automated way to verify that their onboarding configuration is correct and that all necessary services are running.

## Design Doc
1. Create `deploy/scripts/ohc-health-check.sh`
2. Ensure it performs basic checks (e.g. checking `.env` variables, checking if required ports are open).
