---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: K-Factor UI Dashboard

## Problem Statement
The backend Viral Coefficient API has been implemented, but it is not exposed in the Flutter frontend `api_service.dart` nor displayed in the `referrals_dashboard_screen.dart`.

## Design Doc
1. Add `getViralCoefficient` to `srcs/app/lib/services/api_service.dart`.
2. Fetch and display the Viral Coefficient on the `referrals_dashboard_screen.dart`.
