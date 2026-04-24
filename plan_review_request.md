# Problem Description

The task is to build documentation features for the OneHumanCorp app:
1. In-App Help Center (searchable, organized by topic).
2. Contextual Tooltips (with a tooltip registry).
3. Interactive Walkthroughs.
4. AI-Powered Help Chat (floating button).
5. Video Tutorials (embedded, metadata in backend).
6. API Documentation (for advanced users).
7. Release Notes & Changelog (What's New in app).

# Proposed Plan

1. **Tooltip Registry**: Create `src/app/lib/widgets/tooltip_registry.dart` with a central repository of plain-language tooltips. Create a `ContextualTooltip` widget wrapper.
2. **Help Center Screen**: Create `src/app/lib/screens/help_center_screen.dart` with sections (Getting Started, etc.). Add a `HelpCenterArticle` view.
3. **Floating AI Help Chat**: Create `src/app/lib/widgets/ai_help_chat_button.dart` using a floating action button on screens like `DashboardScreen`. It opens a specialized help chat overlay.
4. **Interactive Walkthroughs**: Create `src/app/lib/widgets/interactive_walkthrough.dart` for the overlay highlight + speech bubble system.
5. **Video Tutorials & Release Notes**: Add `src/app/lib/screens/video_tutorials_screen.dart` and `release_notes_screen.dart`.
6. **API Documentation**: Create `src/app/lib/screens/api_docs_screen.dart` linking to or displaying an OpenAPI spec interface for advanced users.
7. **E2E Testing**: Add E2E tests for these screens in `src/tests/e2e/e2e_help_center_test.go` or Flutter E2E tests, starting from home page to UI.
8. **Pre-commit**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
9. **Verification**: Verify the implementation compiles and tests pass (`bazelisk test //...`). Use `read_file` to confirm changes.
