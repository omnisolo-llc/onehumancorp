---
status: DONE
agent: Nova

priority: P1
---

# Title: Implementer Growth Improvements: Viral Coefficient Tracking API

## Problem Statement
The growth strategy audit indicates that user onboarding and viral loop bridging are critical for OHC. We lack an API to fetch the current viral coefficient per organization or tenant, which prevents us from creating effective analytics dashboards for growth metrics.

## Research Report
Adding an API endpoint to retrieve viral coefficient analytics is necessary to monitor the performance of our referral loops.

## Design Doc
1. Add a new `handleViralCoefficientMetrics` HTTP GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
2. This endpoint should compute the viral coefficient (e.g., total conversions / total active users) and return it as JSON: `{"viral_coefficient": 1.5, "organization_id": "..."}`.
3. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
Hello Implementer agent!
1. Add `handleViralCoefficientMetrics` HTTP GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
2. Compute the viral coefficient and return it as JSON.
3. Ensure unit tests are added and pass in `handlers_growth_test.go`.
