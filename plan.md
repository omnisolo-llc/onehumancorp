1. **Understand Task and Specific Implementation Details**:
   - The task is to create the "Grow my business" wizard which is triggered from the home dashboard.
   - It should use progressive disclosure style, Glassmorphism UI (`GlassCard`), Outfit/Inter typography, and be responsive for 375px screens.
   - The wizard needs the following steps: Add products, Connect Socials, Email Campaign, etc.

2. **Grow My Business Wizard Implementation**:
   - Modify `src/app/lib/screens/ongoing_management_wizards.dart`.
   - Create a `ConsumerStatefulWidget` named `GrowMyBusinessWizardScreen`.
   - Define state variable `int _step = 0;` to track the current step.
   - Add a `build` method returning a `Scaffold` with an `AppBar` ("Grow Your Business").
   - Wrap the body in a `Center`, `SingleChildScrollView` and `ConstrainedBox(maxWidth: 600)`.
   - Inside the container, add a `GlassCard` (from `../widgets/glass_card.dart`).
   - The UI will have an introduction page (`_step == 0`) with text "Next steps to grow your business". A button "Start" to advance.
   - Step 1: "Add 5 more products". Text explaining why it helps. A button "I'll do it later" (`_step++`) and "Do it now" (which can navigate or just `_step++`).
   - Step 2: "Connect Instagram". Text explaining social reach. Buttons to advance.
   - Step 3: "Run your first email campaign". Final step, "Finish" button that navigates back to `/dashboard`.
   - Verify changes with `git diff src/app/lib/screens/ongoing_management_wizards.dart`.

3. **Dashboard Integration**:
   - Modify `src/app/lib/screens/dashboard_screen.dart`.
   - In `_DashboardContent`, inside the `Wrap` containing "Build My Website" and "Billing & Credits", add:
     `OutlinedButton.icon(onPressed: () => context.go('/wizards/grow_my_business'), icon: const Icon(Icons.trending_up), label: const Text('Grow My Business')),`
   - Verify changes with `git diff src/app/lib/screens/dashboard_screen.dart`.

4. **Routing Integration**:
   - Modify `src/app/lib/router.dart`.
   - Add the new route in the `routes` list of `ShellRoute`:
     `GoRoute(path: '/wizards/grow_my_business', builder: (context, state) => const GrowMyBusinessWizardScreen()),`
   - Import `GrowMyBusinessWizardScreen` from `ongoing_management_wizards.dart`.
   - Verify changes with `git diff src/app/lib/router.dart`.

5. **Unit Testing**:
   - Create `src/app/lib/screens/ongoing_management_wizards_test.dart`. (If it does not exist, wait, the `BUILD.bazel` file indicates we can just add `*_test.dart` files and Bazel handles it, but since `src/app/lib/screens/wizard_screen_test.dart` exists, we can add it there or in a new file `grow_my_business_wizard_test.dart`). Let's create `src/app/lib/screens/grow_my_business_wizard_test.dart`.
   - Inside, use `testWidgets('GrowMyBusinessWizardScreen transitions', ...)` to verify rendering, verify `find.text('Next steps to grow your business')` is found. Tap the "Start" button, verify `find.text('Add 5 more products')` is found, etc.

6. **E2E Testing**:
   - Modify `src/tests/e2e/dashboard_test.go` or create `src/tests/e2e/cuj_grow_business_test.go` and add it to `src/tests/e2e/BUILD.bazel`. Let's just modify `src/tests/e2e/dashboard_test.go`.
   - Add `TestGrowMyBusinessWizardFromDashboard(t *testing.T)`.
   - `loginAsAdmin(t, page)`
   - `page.Locator("text=Grow My Business").Click()`
   - Assert page contains text "Next steps to grow your business" (`Expect(page.Locator("text=Next steps to grow your business")).ToBeVisible()`).
   - Click "Start". Assert page contains text "Add 5 more products".
   - Click "I'll do it later". Assert page contains "Connect Instagram".
   - Click "I'll do it later". Assert page contains "Run your first email campaign".
   - Click "Finish". Assert page URL is `/dashboard` or dashboard title is visible.

7. **Execute Tests**:
   - Run `bazelisk test //...` to ensure all tests pass and no regressions are introduced.

8. **Pre-commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
