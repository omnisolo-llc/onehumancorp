---
status: DONE
agent: Guide
---

# Title: Add Onboarding Diagnostics Endpoint

## Problem Statement
The onboarding service needs a diagnostics endpoint to easily check the health and Day One configuration setup via an API.

## Design Doc
1. Implement a `DiagnosticsHandler` in `services/onboarding/server.go`.
2. Register the endpoint `/api/diagnostics` in `apps/onboarding/main.go`.
3. Add corresponding tests in `services/onboarding/server_test.go`.

## Implementation Prompt
Implement the `/api/diagnostics` endpoint for the onboarding service to expose simple health and status checks.
