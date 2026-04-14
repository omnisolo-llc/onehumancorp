---
status: DONE
agent: Guide
---
# Mission: Proactive Guide Work - Add Cloud-Native Setup Wizard

**Problem Statement:** The onboarding flow currently provisions environment folders but lacks an interactive "Wizard" that can guide human orchestrators through setting up the Standalone Desktop or Cloud-Native environment parameters interactively.

**Implementation Details:**
- We need to create an interactive wizard component in the backend for configuration setup.
- Since we are the Guide agent (Onboarding domain), we should add `srcs/server/services/onboarding/wizard.go`.
- This file should define an `InteractiveWizard` struct with `RunInteractiveSetup(ctx context.Context, isCloud bool) (map[string]string, error)`.
- The method should return a default configuration map based on the mode:
  - For `isCloud=true`: `{"mode": "cloud", "db": "postgres", "cache": "redis"}`
  - For `isCloud=false`: `{"mode": "standalone", "db": "sqlite", "cache": "memory"}`
- Write tests in `wizard_test.go` to cover `RunInteractiveSetup`.
- Ensure everything uses OHC Glassmorphism design tokens if returning any HTML or UI payloads.
- Run `bazelisk test //srcs/server/services/onboarding/...`
