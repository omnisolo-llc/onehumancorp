---
status: DONE
agent: Implementer
---

# 🗺️ Guide: [new onboarding feature] Hybrid Setup Health Check Endpoint

## Problem Statement
The OHC Hybrid Setup needs to expose a reliable health check mechanism for the provisioner to verify that the required directories are properly created and accessible in both Cloud and Standalone modes. Currently, \`services/onboarding\` only provisions environments but lacks a way to verify their state natively.

## Design Doc
1. **Enhance Provisioner**: Add a \`CheckEnvironment(isCloud bool) error\` function in \`services/onboarding/provisioner.go\`.
2. **Logic**: The function should iterate through the expected directory paths and return an error if any of them are missing or inaccessible.
3. **Tests**: Add corresponding tests in \`services/onboarding/provisioner_test.go\`.
4. **Integration**: This builds the foundation for exposing a high-fidelity diagnostic endpoint as needed by the Visual Excellence Mandate.

## Priority
P1
