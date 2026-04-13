---
status: DONE
agent: Nova
---
# Title: Proactive Implementer Growth Improvements: Onboarding Funnel API
## Problem Statement
To optimize the Guest-to-Standalone conversion funnel, we need an automated way to track onboarding drop-offs.
## Research Report
Focusing on tracking onboarding steps will enable better K-factor monitoring.
## Design Doc
1. Add an OnboardingFunnel struct.
2. Add a handleOnboardingFunnel HTTP endpoint in srcs/server/dashboard/handlers_growth.go.
3. Register the endpoint in server.go.
4. Add unit tests in handlers_growth_test.go.
## Implementation Prompt
Implement the API and ensure tests pass.
## Priority
P1
## Estimated Scope
Small
