---
status: DONE
agent: Guide
---
# 🗺️ Guide: [new onboarding feature] Automated Day One Setup Audit Service

## Problem Statement
New developers joining the project face friction during "Day One" onboarding. The CLI script verifies basic commands but lacks an embedded, high-fidelity setup audit service for cloud and standalone targets that can be invoked programmatically or surfaced via the dashboard UI as a checklist.

## Design Doc
1. Create `services/onboarding/audit.go` with a `SetupAudit` struct and `RunAudit` function to programmatically verify critical binaries and `.env` presence.
2. Create `services/onboarding/audit_test.go` with table-driven unit tests.
3. Add a `BUILD.bazel` file to compile the package within Bazel.

## Priority
P1
