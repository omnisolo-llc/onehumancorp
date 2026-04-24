**Plan: Implement Growth Features - Viral Invite Loop**

1.  **Understand Current State**: The codebase has some foundational referral tracking elements in `src/server/dashboard/handlers_growth.go`. However, the current referral flow isn't a complete viral loop from mobile/UI to backend yet.
2.  **Referral Program Backend (Go API)**:
    *   Using python scripts or `replace_with_git_merge_diff`, edit `handleReferrals` and related handlers in `src/server/dashboard/handlers_growth.go`.
    *   Add `Credits` tracking to the `Referral` struct and grant 1 credit when a conversion is recorded (simulating "1 month free Pro" attribution). (Already applied)
3.  **Referral Program Frontend (Flutter UI)**:
    *   Edit `src/app/lib/widgets/growth_referral_widget.dart` to change the "Invite Team to Expand Quota" button to a "Share OHC (1 Month Free Pro)" button. Update the `Clipboard` copy functionality to include the pre-filled message: `"Hey! I'm running my business on OHC. Use my link to get 1 month free Pro, and I get one too! [Link]"` (Already applied)
    *   Edit `src/app/lib/screens/referrals_dashboard_screen.dart` and its test `src/app/lib/screens/referrals_dashboard_screen_test.dart` to show the earned credits on the referral tracking dashboard. (Already applied)
4.  **Frontend Tests**:
    *   Update `src/app/test/growth_referral_widget_test.dart` to match the new button text. (Already applied)
    *   Update `src/app/test/cuj_diagnostics_referrals_e2e_test.dart` to fix pumpAndSettle timeouts by changing to `.pump()`. (Already applied)
5.  **E2E Tests**:
    *   Create a new file `src/tests/e2e/cuj_referral_e2e_test.go` that simulates the viral loop workflow by clicking the "Share OHC (1 Month Free Pro)" button, verifying the snackbar, and visiting the referral dashboard. (Already applied)
    *   Add this new file to `src/tests/e2e/BUILD.bazel`. (Already applied)
6.  **Verify**:
    *   Run frontend tests to ensure changes are correct. (Already passing)
7.  **Pre commit step**:
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
