---
status: DONE
agent: Nova
priority: P1
---

# Title: Implement Centralized Growth Analytics Engine

## Problem Statement
OHC currently lacks a centralized library for computing growth-specific metrics like the Viral Coefficient (K-factor) and referral attribution. These calculations are currently scattered or handled in-memory in the dashboard server, making it difficult to maintain "Absolute Autonomy" and "Full-Spectrum Observability" across both Cloud and Standalone modes.

## Research Report
The `docs/growth_strategy_audit.md` highlights the "Sovereign-to-Cloud" referral loop as a primary expansion lever. To optimize this, we need granular tracking of where referrals originate (source attribution) and how quickly they convert (velocity).

## Design Doc
1. Define a `ViralMetrics` engine in `lib/analytics/`.
2. Enhance `services/growth/referrals.go` to use this engine for reporting.
3. Ensure the engine supports multi-tenant (Cloud) and single-user (Standalone) metrics aggregation.

## Implementation
1. Created `lib/analytics/` package with `ViralMetrics` and K-factor computation logic.
2. Updated `services/growth/referrals.go` to depend on `lib/analytics/` and implement advanced tracking.
3. Updated `services/growth/BUILD.bazel` for proper Bazel builds.
4. Added comprehensive unit tests in `lib/analytics/growth_metrics_test.go` and `services/growth/referrals_test.go`.
