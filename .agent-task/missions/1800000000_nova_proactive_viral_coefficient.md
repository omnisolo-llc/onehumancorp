---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Coefficient API

## Problem Statement
To accurately measure the effectiveness of our Sovereign-to-Cloud referral loop, we need an automated way to track and compute the Viral Coefficient (K-factor) of our active user base. Currently, we track raw referrals but lack an aggregated metric API to expose this to internal dashboards.

## Research Report
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. A viral loop bridge from Standalone to Cloud requires continuous monitoring of the K-factor.

## Design Doc
1. Add a `ViralCoefficientResponse` struct.
2. Add a `handleViralCoefficient` HTTP GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
