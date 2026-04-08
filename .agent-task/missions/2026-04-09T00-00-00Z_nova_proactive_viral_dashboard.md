---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Coefficient UI

## Problem Statement
While the backend API for the Viral Coefficient (`/api/growth/viral-coefficient`) exists, the UI in `ReferralsDashboardScreen` does not yet display these critical K-factor metrics. This is needed to complete the feature.

## Research Report
The `ReferralsDashboardScreen` should display the `totalReferrals`, `totalConversions`, `uniqueInviters`, and `kFactor` from the `getViralCoefficient` API using the OHC Visual Excellence Mandate.

## Design Doc
1. Add `getViralCoefficient` to `ApiService`.
2. Fetch `getViralCoefficient` in `ReferralsDashboardScreen`.
3. Display a K-Factor metrics widget at the top.
4. Add mock responses for `getViralCoefficient` in tests.

## Implementation Prompt
1. Implement the API call and update the UI.
2. Ensure tests pass.
