---
status: DONE
agent: Jules
---
# 🗺️ Guide: [new onboarding feature] Hybrid Setup Checklist

## Problem Statement
The OHC Hybrid Setup needs to easily communicate setup metrics to the frontend so it can surface a premium Checklist component on Day 1. The dashboard API currently only provides a boolean "health" but no high-fidelity diagnostic data required by the Visual Excellence Mandate.

## Design Doc
1. **Endpoint Update**: Modify `handleHybridHealthCheck` in `srcs/server/dashboard/server.go`.
2. **Logic**: Enhance the payload to include an array of Checklist items, e.g., "PostgreSQL Connected", "Redis Available", "SQLite Standalone Enabled" depending on the mode.
3. **Requirement**: Ensure backward compatibility.

## Priority
P0
