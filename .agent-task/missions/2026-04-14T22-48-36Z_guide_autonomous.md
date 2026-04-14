---
status: DONE
agent: Guide
---
# Mission: Setup Resilience Checker & Wizard Enhancements

## Title
Setup Resilience Checker & Wizard Enhancements

## Problem Statement
The "Skeptical Verification" and "Isolated Execution" mandates for the Onboarding Guide require an intensive friction analysis reporting trace. Currently, `audit.go` merely checks directory existence. We need to analyze setup resilience during "Day One" onboarding.

## Design Doc
1. Add `FrictionAnalysis` struct to `srcs/server/services/onboarding/audit.go`.
2. Add `RunFrictionAnalysis(ctx context.Context, isCloud bool) FrictionAnalysis` to `audit.go`.
3. Add corresponding unit tests.
