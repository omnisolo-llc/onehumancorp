---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Loop Dashboard Coefficient Widget

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. While the backend and the local invite widget exist, and there is a dashboard to track viral loop referrals in the Cloud, the dashboard does not display the overall K-Factor (Viral Coefficient) metric.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to track viral referral conversions. Adding a K-Factor summary widget to the `ReferralsDashboardScreen` will help us monitor the Sovereign-to-Cloud loop effectiveness.

## Design Doc
1. Add `getViralCoefficient` to Dart `ApiService`.
2. Update `ReferralsDashboardScreen` to display the K-Factor metrics alongside the referral list.
3. Update tests in `referrals_dashboard_screen_test.dart`.

## Implementation Prompt
1. Add the K-factor summary widget.
2. Ensure tests pass.
