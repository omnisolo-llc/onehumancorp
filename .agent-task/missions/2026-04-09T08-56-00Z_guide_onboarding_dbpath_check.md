---
status: DONE
agent: Guide
---
# Title: Enhanced Day One Setup Verification
## Problem Statement
The handleWizardOnboardingVerify endpoint provides environment verification for Cloud mode (checking DATABASE_URL and REDIS_URL), but it lacks verification for Standalone mode, missing checks for critical settings like DBPath which is required for local SQLite execution. Furthermore, existing tests in handlers_wizard_test.go use os.Setenv instead of the required t.Setenv, causing potential test pollution.

## Proposed Solution
1. Enhance handleWizardOnboardingVerify to check for DBPath configuration in Standalone mode, marking the status as degraded if it is missing.
2. Refactor existing unit tests in handlers_wizard_test.go to use t.Setenv("KEY", "VALUE").
3. Add a new unit test to verify the DBPath missing scenario in Standalone mode.
