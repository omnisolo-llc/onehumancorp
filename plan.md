# Problem Description
The goal is to implement documentation-critical features for the OneHumanCorp Small Business App, creating help content, interactive guides, and in-app assistance for non-technical small business owners.

Requirements from the prompt:
1. **In-App Help Center**: Searchable help portal accessed via "?" button in every major screen. Articles by topic. Mobile friendly.
2. **Contextual Tooltips**: Tooltips for non-obvious UI elements. Max 2 sentences, plain language. Implement a `tooltip_registry.dart` so agents can add/update tooltips without touching UI code.
3. **Interactive Walkthroughs**: Step-by-step in-app tours. Overlay highlight + speech bubble system (no popups/modals).
4. **AI-Powered Help Chat**: Floating "Ask anything" chat button on every page. Routes to a Help Agent using help center content as context. "Read the full article →" links.
5. **Video Tutorials**: Embed short video tutorials for top 10 tasks. Metadata in backend, mobile portrait-optimized player.
6. **API Documentation**: Interactive API reference for Advanced users (e.g., Swagger UI).
7. **Release Notes & Changelog**: "What's New" section in the app showing recent updates with screenshots. Link to full changelog.

# Proposed Plan

1. **Tooltip Registry**:
   - Create `src/app/lib/widgets/tooltip_registry.dart` with a map of tooltip keys to text.
   - Use `read_file` to verify `src/app/lib/widgets/tooltip_registry.dart` is correctly created.

2. **OhcTooltip Component**:
   - Create `src/app/lib/widgets/ohc_tooltip.dart` for the `OhcTooltip` widget.
   - Use `read_file` to verify `src/app/lib/widgets/ohc_tooltip.dart` is correctly created.

3. **Apply Tooltips**:
   - Modify `src/app/lib/screens/dashboard_screen.dart` to apply `OhcTooltip` to the 'AI Helpers' section.
   - Modify `src/app/lib/screens/agents_screen.dart` to apply `OhcTooltip` to the 'Help me fix this' button.
   - Verify changes using `cat` or `read_file`.

4. **Global In-App Help Overlay**:
   - Create `src/app/lib/widgets/in_app_help_overlay.dart` to wrap `AppShell` with a floating action button linking to the Help Center and Chat.
   - Use `read_file` to verify `src/app/lib/widgets/in_app_help_overlay.dart` is created.
   - Modify `src/app/lib/router.dart` to wrap `AppShell(child: child)` with `InAppHelpOverlay(child: AppShell(child: child))`. Add the `/help`, `/help/chat`, `/api-docs` and `/whats-new` routes to the router.
   - Use `read_file` on `src/app/lib/router.dart` to confirm the new routes and `InAppHelpOverlay` wrapper were added correctly.

5. **Help Center Screen**:
   - Create `src/app/lib/screens/help_center_screen.dart` to serve as the main searchable portal.
   - Use `read_file` to verify `src/app/lib/screens/help_center_screen.dart`.

6. **Help Chat Screen**:
   - Create `src/app/lib/screens/help_chat_screen.dart` linking to `/api/help/chat`.
   - Use `read_file` to verify `src/app/lib/screens/help_chat_screen.dart`.

7. **Interactive Walkthroughs**:
   - Create `src/app/lib/widgets/walkthrough_overlay.dart`. Implement a `WalkthroughOverlay` widget that accepts a target `GlobalKey` and displays a text bubble with navigation controls.
   - Use `read_file` to verify `src/app/lib/widgets/walkthrough_overlay.dart`.

8. **Video Tutorials & Help Widget**:
   - Create `src/app/lib/widgets/video_tutorial_list.dart` integrated into `HelpCenterScreen` reading from `/api/tutorials/videos`.
   - Use `read_file` to verify `src/app/lib/widgets/video_tutorial_list.dart`.

9. **Release Notes UI**:
   - Create `src/app/lib/screens/whats_new_screen.dart` calling the `/api/changelog` endpoint to display recent updates.
   - Use `read_file` to verify `src/app/lib/screens/whats_new_screen.dart`.

10. **API Documentation UI**:
    - Create `src/app/lib/screens/api_docs_screen.dart` using a `ListView` of `ExpansionTile` widgets to display mocked API endpoint details.
    - Use `read_file` to verify `src/app/lib/screens/api_docs_screen.dart`.

11. **Backend Handlers (Help)**:
    - Create `src/server/dashboard/handlers_help.go` implementing `/api/help/chat`, `/api/changelog`, and `/api/tutorials/videos` handlers.
    - Use `read_file` to verify `src/server/dashboard/handlers_help.go`.

12. **Register Backend Handlers**:
    - Modify `src/server/dashboard/server.go` to add `mux.HandleFunc` registrations for `/api/help/chat`, `/api/changelog`, and `/api/tutorials/videos` right next to the wizard endpoints.
    - Verify with `cat src/server/dashboard/server.go`.

13. **E2E Test File**:
    - Create `src/app/e2e/help_center.spec.ts`. Write tests to: Navigate to the homepage, click the '?' button, and assert that the Help Center modal/screen renders successfully. Interact with the chat.
    - Use `read_file` to verify `src/app/e2e/help_center.spec.ts`.

14. **Run Tests**:
    - Run the test suite using `bazelisk test //...` to verify all changes and ensure no regressions were introduced.

15. **Pre-commit**:
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
