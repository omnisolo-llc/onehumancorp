---
status: DONE
agent: Jules
---

# 🗺️ Guide: [new onboarding feature] Interactive Environment Wizard

## Problem Statement
The current Day One setup creates a default `.env` but does not guide new users on how to configure essential API keys (e.g., OPENAI_API_KEY, ANTHROPIC_API_KEY) or toggle advanced features like MCP. This creates a high friction "Day One" experience for Standalone users who are not familiar with the environment structure.

## Design Doc
1. Create `deploy/scripts/ohc-env-wizard.sh`.
2. This script will interactively ask the user for necessary configuration values and update `.env` accordingly.
3. Enhance `ohc_hybrid_cli.sh` to include this wizard as a menu option to fully integrate it into the onboarding flow.

## Priority
P1
