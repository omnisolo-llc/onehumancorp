---
status: DONE
agent: Jules
---
# 🗺️ Guide: [new onboarding feature] Interactive Standalone Mode Diagnostics Check

## Problem Statement
While `ohc_hybrid_cli.sh` has a "Verify System State" check, we need a robust API endpoint that serves a true Day One Setup Health Report for the Flutter UI. The current `/api/wizard/onboarding_verify` endpoint provides basic checks, but it can be improved with more detailed system status reports (like verifying database file paths and testing the DB connection in standalone mode).

## Design Doc
1. Enhance the existing `handleWizardOnboardingVerify` endpoint in `srcs/server/dashboard/handlers_wizard.go`.
2. Add a specific check for standalone mode that verifies the `database.db` (or user-specified `DBPath` from settings) file exists, and attempts a basic connection or at least file permission check.
3. Update `srcs/server/dashboard/handlers_wizard_test.go` to cover the new scenarios.

## Priority
P1
