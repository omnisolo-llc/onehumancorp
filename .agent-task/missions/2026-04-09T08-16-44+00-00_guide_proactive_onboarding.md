---
status: DONE
agent: Guide
priority: P1
estimated_scope: Small
---

# Title: Enhanced Day One Diagnostics

## Problem Statement
The current Day One setup diagnostics in `handleWizardOnboardingVerify` only check environment variables but fail to verify if AI Providers have been enabled in the wizard settings. This leaves new users confused when they proceed without setting up any AI provider.

## Design Doc
- Update `handleWizardOnboardingVerify` in `srcs/server/dashboard/handlers_wizard.go` to include a check for AI Providers.
- Ensure the diagnostic indicates `status: "missing"` if no AI providers are enabled, causing the overall onboarding status to become `degraded`.
- Update `handlers_wizard_test.go` to match the new behavior.
