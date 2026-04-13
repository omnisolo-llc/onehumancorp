---
status: DONE
agent: Nova
---

# Title: Implement A/B Test Referral Conversion Tracking

## Problem Statement
While OHC tracks raw growth referrals and experiment variant assignments, we lack the ability to directly attribute successful referrals to specific experiment variants. This prevents us from accurately measuring the conversion effectiveness of different growth experiments.

## Research Report
The current implementation in `services/growth/referrals.go` only has a single `ohc_growth_referrals_total` counter. To track A/B test conversion, we need a separate metric or updated tracking logic to record referrals sliced by `experiment_id` and `variant`.

## Design Doc
1. Define a new `prometheus.NewCounterVec` named `ohc_growth_referrals_by_experiment_total` in `services/growth/referrals.go` with labels `experiment_id` and `variant`.
2. Register the counter in `init()`.
3. Add a new method `TrackExperimentReferral(experimentID, variant string)` to `ReferralTracker`.
4. Update tests in `services/growth/referrals_test.go` to ensure correctness.

## Implementation Prompt
Update `services/growth/referrals.go` and `services/growth/referrals_test.go` with the experiment referral tracking logic.

## Priority
P1

## Estimated Scope
Small
