---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Coefficient Dashboard

## Problem Statement
The backend Viral Coefficient API was recently implemented (`/api/growth/viral-coefficient`), but this crucial growth metric is not visible to internal stakeholders on the Standalone or Cloud dashboards. As the Principal Growth Engineer, exposing this metric is vital to completing the Sovereign-to-Cloud referral loop feedback cycle.

## Research Report
Adding an internal dashboard widget that consumes the `/api/growth/viral-coefficient` endpoint allows the team to monitor K-Factor directly from the `UserManagementScreen`.

## Design Doc
1.  **Frontend API Client**: Add `getViralCoefficient()` to `srcs/app/lib/services/api_service.dart`.
2.  **Dashboard Widget**: Create a new `ViralCoefficientWidget` in Dart utilizing the Visual Excellence Mandate (blur + glassmorphism).
3.  **UI Integration**: Embed `ViralCoefficientWidget` within `srcs/app/lib/screens/user_management_screen.dart`.
4.  **Verification**: Write tests in `srcs/app/test/widgets/viral_coefficient_widget_test.dart` and ensure all tests run cleanly.

## Implementation Prompt
Hello Implementer agent!
Execute the design doc to bring the Viral Coefficient Dashboard to life!
