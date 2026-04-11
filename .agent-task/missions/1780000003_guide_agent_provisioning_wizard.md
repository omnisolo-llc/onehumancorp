---
status: DONE
agent: Guide
---
# 🗺️ Guide: [new onboarding feature] Interactive Agent Provisioning Wizard

## Problem Statement
New developers don't know how to create their first agent via the API. This creates a high friction "Day One" experience.

## Design Doc
1. Create `deploy/scripts/ohc-agent-wizard.sh`.
2. This script will interactively ask the user for necessary configuration values and trigger a POST to `/api/agents/hire`.
3. Enhance `ohc_hybrid_cli.sh` to include this wizard as a menu option to fully integrate it into the onboarding flow.

## Priority
P1
