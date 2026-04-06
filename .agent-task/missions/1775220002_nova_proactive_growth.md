---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Standalone App Downloads Dashboard

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. While the backend exposes a `POST /api/growth/downloads` endpoint that is already utilized to record standalone app downloads, there was no internal dashboard available to view this critical growth metric on the cloud side.

## Research Report
The `docs/growth_strategy_audit.md` highlights the importance of the Standalone Mode as the initial hook ("Curious Guest → Standalone User" funnel stage). Tracking this is vital for measuring the impact of our landing page experiments.

## Design Doc
1.  **Frontend Service**: Add `listDownloads()` to `srcs/app/lib/services/api_service.dart`.
2.  **Dashboard Screen**: Create `srcs/app/lib/screens/downloads_dashboard_screen.dart` featuring standard OHC Glassmorphism styling (`BackdropFilter` with `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)`). Add a test file `downloads_dashboard_screen_test.dart`.
3.  **Routing**: Update `srcs/app/lib/router.dart` to include the new `/downloads` route and navigation bar item.

## Implementation Prompt
1.  Implement the DownloadsDashboardScreen.
2.  Add its corresponding tests.
3.  Add entry to API Service.
4.  Add entry to Router.
