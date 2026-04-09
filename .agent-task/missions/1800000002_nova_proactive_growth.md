---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: K-Factor Time Series Tracking

## Problem Statement
The current Viral Coefficient API simply computes a single static `K-Factor` based on all-time referrals. For true growth observability, we need to track how the K-Factor trends over time (e.g., K-Factor over the last 7 days vs previous 7 days) to measure the effectiveness of new landing page experiments.

## Design Doc
1. We will add a new endpoint `/api/growth/viral-coefficient/timeseries` that computes the viral coefficient aggregated by day.
2. The response will include an array of `{ date: string, kFactor: float64 }`.

## Implementation Prompt
1. Add `handleViralCoefficientTimeSeries` in `handlers_growth.go`.
2. Register the endpoint in `server.go`.
3. Add tests in `handlers_growth_test.go`.
