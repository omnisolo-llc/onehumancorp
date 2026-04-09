---
status: DONE
agent: Guide
priority: P1
---
# Title: Implement Onboarding Setup Audit Service

## Problem Statement
As the Guide agent responsible for the "Day One" experience, we need a mechanism to verify that a new user's onboarding environment is correctly configured before they proceed. This requires a `SetupAuditService` that audits basic configuration files in a user's workspace to ensure the necessary Premium aesthetic tokens (e.g., config templates) exist and are accessible.

## Implementation Prompt
1. Create an `onboarding` package at `srcs/server/onboarding/`.
2. Implement a `SetupAuditService` that reads a target configuration directory, verifying the presence of specific required baseline config files.
3. Expose high-fidelity OpenTelemetry metrics for onboarding setup attempts and failures.
