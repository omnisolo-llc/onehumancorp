The task requires implementing mobile navigation clarity: "Audit the mobile nav. Ensure the 5 most-used actions (Add Product, View Orders, Check Messages, See Analytics, Share Store) are reachable in ≤2 taps from any screen."

Currently, `AppShell` in `src/app/lib/router.dart` uses a fixed `Row(children: [_Sidebar(), Expanded(child: child)])` for the layout, with `_Sidebar` always displaying a `NavigationDrawer`. This isn't responsive and doesn't handle mobile navigation well, failing the standard mobile-first non-negotiables: "Every screen must be fully functional on a 375px-wide phone without horizontal scroll." The `NavigationDrawer` in a Row takes up significant screen real estate on a 375px screen.

Plan:

1. **Investigate Route Actions**: I will use `grep` and `cat` on `src/app/lib/router.dart` to discover and confirm the correct route paths for "Share Store" and "Add Product". Based on previous `grep` operations, I confirmed:
   - "See Analytics": `/dashboard`
   - "View Orders": `/orchestration/tasks`
   - "Check Messages": `/chat`
   - "Share Store": `/referrals`
   - "Add Product": `/wizard` (Setup Wizard).

2. **Update AppShell Layout in `src/app/lib/router.dart`**: Use `MediaQuery.of(context).size.width` to dynamically switch layout based on screen width (breakpoint 768px).
   - If `width >= 768`: Use the existing `Row(children: [_Sidebar(), Expanded(child: child)])` layout inside a `Scaffold`.
   - If `width < 768`: Return a `Scaffold(body: child, bottomNavigationBar: ...)` with a `NavigationBar` containing 4 destination items mapping to: Analytics (`/dashboard`), Orders (`/orchestration/tasks`), Messages (`/chat`), Share (`/referrals`).
   - For "Add Product", add a `FloatingActionButton` to the mobile `Scaffold` that triggers the setup wizard `/wizard` to add new entities.

3. **Verify the Layout Code**: Run `cat src/app/lib/router.dart` to verify the modified `AppShell` correctly uses `MediaQuery` and defines the `NavigationBar` and `FloatingActionButton`.

4. **Add Mobile E2E Test**: Create `src/app/e2e/mobile_navigation.spec.ts` using Playwright.
   - Set the viewport to 375x812.
   - Login to the app.
   - Verify that the bottom navigation items and floating action button for the 5 most-used actions are visible and clickable, ensuring they are accessible within ≤2 taps.

5. **Verify the E2E Test**: Run `cat src/app/e2e/mobile_navigation.spec.ts` to confirm its contents were created correctly.

6. **Pre-Commit Steps**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Run Full Test Suite**: Execute `export PATH=$PATH:$HOME/go/bin && bazelisk test //...` to ensure all tests pass and no regressions were introduced.
