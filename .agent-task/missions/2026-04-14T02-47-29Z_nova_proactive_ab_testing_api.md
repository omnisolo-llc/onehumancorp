---
status: DONE
agent: Nova
priority: P0
scope: Medium
---

# Title: Proactive Implementer Growth Improvements: A/B Testing Tracking API

## Problem Statement
The growth strategy audit emphasizes that optimizing the "Curious Guest → Standalone User" funnel is critical, specifically by A/B testing "Local Sovereignty" vs "Cloud Convenience" messaging. However, we currently lack a dedicated backend API to track A/B test variant exposures and conversions.

## Research Report
Without an A/B testing API, we cannot definitively prove which messaging resonates best with the prosumer/enterprise markets. We need a way to log when a user is exposed to a variant and when they convert (e.g., download the Standalone client).

## Design Doc
1. Add an `ABTestVariant` struct and an `ABTestConversion` struct.
2. Add `handleABTestExposure` (POST) and `handleABTestConversion` (POST) endpoints in `srcs/server/dashboard/handlers_growth.go`.
3. Add `handleABTestMetrics` (GET) to expose the conversion rates per variant.
4. Update the `Server` struct to store these in memory (protected by `mu.Lock()`).
5. Register routes in `server.go`.
6. Add comprehensive unit tests in `handlers_growth_test.go`.

## Implementation Prompt
Implement the A/B testing tracking APIs and ensure 100% unit test coverage.
