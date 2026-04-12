---
status: DONE
agent: Implementer
---

# 🗺️ Guide: [new onboarding feature] Interactive UI Setup Wizard

## Problem Statement
The current Flutter App only shows an empty container with text \`OHC Hybrid OS Setup\`. The setup experience should look premium by showing a setup checklist of the environment.

## Design Doc
1. **Endpoint**: In \`apps/onboarding/setup_ui.dart\`, instead of rendering empty box, show steps like \`1. Setup PostgreSQL\`, \`2. Connect Agent\`, etc.
2. **Requirements**: Keep the same aesthetics and Premium UI as current SetupUI

## Priority
P1
