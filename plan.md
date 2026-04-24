1. **Implement In-App Help Center**:
   - Create a `HelpCenterScreen` in `src/app/lib/screens/help_center_screen.dart` with searchable topics (Getting Started, My Store, Payments, etc.) and quick links to API docs and Release Notes.

2. **Implement AI Help Chat**:
   - Add a floating action button on the `DashboardScreen` for "AI Help Chat".
   - The button should open a chat interface `HelpChatOverlay` that answers questions.

3. **Contextual Tooltips System**:
   - Implement a tooltip registry `src/app/lib/services/tooltip_service.dart` to manage text content.
   - Add a `TooltipWrapper` widget that displays tooltips (hover on desktop, long-press on mobile) for key non-obvious UI elements.

4. **Interactive Walkthroughs**:
   - Create step-by-step guides using `src/app/lib/widgets/walkthrough_overlay.dart`. This should be an overlay highlight + speech bubble system.

5. **Release Notes & Changelog**:
   - Implement a Release Notes screen parsing standard changelogs to show recent updates.

6. **Update `router.dart`**:
   - Add routes for `/help`, `/help/api`, `/help/release-notes`.
   - Update sidebar navigation to include "Help Center".

7. **Ensure E2E tests pass**:
   - Once implemented, verify the existing E2E tests (`help_center_test.go`) pass properly.

8. **Pre Commit Steps**:
   - Run `pre_commit_instructions` tool to complete pre-commit checks.
