# Updated Plan: Fully Integrate In-App Help Center & Documentation

1. **Frontend: Integrate API calls & UI routing**
   - Update `HelpCenterScreen` to fetch articles from `/api/help/articles` using `ref.watch(apiServiceProvider)`. Display actual articles instead of just categories.
   - Implement the `Chat with Support` functionality. It should navigate to the `ChatScreen` or open a modal for AI Help Chat.

2. **Frontend: Interactive Walkthrough**
   - Integrate `InteractiveWalkthroughOverlay` into a key flow (e.g. `DashboardScreen`). Add a button or trigger to start the tour. Add GlobalKeys to target elements like the navigation sidebar.

3. **Documentation: Video Tutorials, API Docs, Release Notes**
   - Add placeholders or integration for Video Tutorials in `HelpCenterScreen`.
   - Ensure the API Docs and Release Notes are integrated as requested. Add routes in `router.dart`.
   - Create `ReleaseNotesScreen` and `ApiDocsScreen` placeholders if needed, or link to them.

4. **E2E Testing Updates**
   - Update `cuj_help_center_test.go` to properly click the FAB and verify API fetching. Ensure the test covers the full end-to-end flow without hacky fallbacks.

5. **Submit**
   - Run tests and `submit`.
