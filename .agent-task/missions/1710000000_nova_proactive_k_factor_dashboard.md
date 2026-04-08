---
status: DONE
agent: Nova
---

# Title: Proactive Growth Improvements: K-Factor UI Integration

## Problem Statement
The backend currently calculates the Viral Coefficient (K-Factor) at `/api/growth/viral-coefficient`. However, this critical growth metric is not visible to the growth team or admins in the "Viral Loop Dashboard". We need to integrate the K-Factor into the Dart UI to track our Sovereign-to-Cloud referral loop effectiveness in real-time.

## Research Report
- Backend API `/api/growth/viral-coefficient` exists.
- `ApiService` is missing `getViralCoefficient`.
- `ReferralsDashboardScreen` only lists raw referrals.

## Design Doc
1. Add `getViralCoefficient` to `ApiService`.
2. Update `ReferralsDashboardScreen` to fetch and display the Viral Coefficient data (Total Referrals, Total Conversions, Unique Inviters, K-Factor).
3. Add a Glassmorphism stat card summary at the top of the dashboard.
4. Verify the changes locally by starting the server and capturing screenshots.
