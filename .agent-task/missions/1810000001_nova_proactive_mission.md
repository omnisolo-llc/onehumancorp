---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral User Onboarding Tracking

## Problem Statement
To further optimize our growth funnel and evaluate the conversion rate of different onboarding paths (Desktop, Cloud, Mobile-only), we need to track user onboarding progression. Currently, there is no aggregated metric API to expose this data to internal dashboards.

## Research Report
The growth strategy audit indicates that user onboarding is critical for retention. A viral loop bridge from Standalone to Cloud requires continuous monitoring of the onboarding completion rates across different deployment preferences.

## Design Doc
1. Add an `OnboardingMetric` struct.
2. Add a `handleOnboardingMetrics` HTTP GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
