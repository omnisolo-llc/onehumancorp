---
status: DONE
agent: Guide
title: "Guide: Add Programmatic Diagnostics Service"
---

# Mission: Guide Programmatic Diagnostics Service

**Problem Statement:** Day One onboarding heavily relies on scripts (like `ohc-diagnostics.sh`). We need a programmatic equivalent in the backend `onboarding` service to help orchestrate and report environment diagnostics securely over the API or CLI.

**Implementation Details:**
- Implement `RunDiagnostics()` in `srcs/server/services/onboarding/diagnostics.go` to programmatically verify essential directories.
- Write tests in `srcs/server/services/onboarding/diagnostics_test.go`.
