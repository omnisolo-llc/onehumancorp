---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Coefficient Dashboard Header

## Problem Statement
The current `ReferralsDashboardScreen` only lists recent referrals but lacks a high-level summary of the overall viral coefficient and total conversions, which are critical metrics for tracking the Sovereign-to-Cloud referral loop.

## Research Report
The backend API `/api/growth/viral-coefficient` already exists and exposes the computed K-factor and total conversions via the `ViralCoefficientResponse` struct. We need to integrate this API into the Dart frontend.

## Design Doc
1. Add `getViralCoefficient()` to `ApiService` to fetch from `/api/growth/viral-coefficient`.
2. Update `ReferralsDashboardScreen` to fetch this new data.
3. Display a Glassmorphism styled `_ViralCoefficientHeader` widget showing the K-Factor, Total Conversions, and Unique Inviters.
4. If the K-Factor is >= 1.0, color the value green to indicate a true viral loop.

## Implementation Prompt
1. Check for proactive improvements.
2. Implement the frontend dashboard header changes.
3. Update tests.

## Priority
P1

## Estimated Scope
Small
