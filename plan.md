1. **Implement Welcome Checklist (Post-Onboarding) UI**
   - Create a new widget `WelcomeChecklistWidget` in `srcs/app/lib/widgets/welcome_checklist_widget.dart` that uses `GlassCard`.
   - The checklist will contain items: ✅ Business live (checked), ⬜ Add 3 more products (links to `/service` or product screen), ⬜ Connect Instagram (links to `/integrations`), ⬜ Share your link with a friend (triggers a share or copies the URL).
   - Each item should be clickable and navigate to the respective route using `context.go()`.
2. **Integrate Checklist into Dashboard**
   - Import `WelcomeChecklistWidget` into `srcs/app/lib/screens/dashboard_screen.dart`.
   - Add the `WelcomeChecklistWidget` at the top of the `_DashboardContent`'s `ListView` just below the Upgrade Banner or at a prominent position to fulfill the requirement: "After going live, show a 'You're set up! Here's what to do next' checklist".
3. **Write/Update E2E Tests**
   - Update E2E tests or write a new test `srcs/app/test/widgets/welcome_checklist_widget_test.dart` to test the new widget and its navigation links.
   - Run existing E2E tests `bazelisk test //srcs/app/...` to verify the dashboard layout and ensure everything passes.
4. **Complete Pre-Commit Steps**
   - Call `pre_commit_instructions` to satisfy checks. Ensure proper testing, verification, review, and reflection are done.
5. **Submit**
   - Submit the changes using the `submit` tool with a descriptive commit message and PR title.
