1. **Referral Program (Mobile Share Flow)**
   - Use `replace_with_git_merge_diff` on `srcs/app/lib/widgets/growth_referral_widget.dart` to implement "Share OHC with a friend, both get 1 month free Pro".
   - Modify the title text to "Share OHC & Get Pro" and the subtitle to "Share OHC with a friend, both get 1 month free Pro."
   - Modify the button to say "Share OHC & Get Pro" and update the snackbar to include a pre-filled message: "Hey, check out OneHumanCorp! Start your business in 10 minutes and we both get 1 month free Pro: https://cloud.ohc.io/invite?token=xYz8vQ_local_sovereign".
   - Use `run_in_bash_session` to `cat srcs/app/lib/widgets/growth_referral_widget.dart` to verify changes.

2. **Business Share & Embed**
   - Use `replace_with_git_merge_diff` on `srcs/app/lib/screens/dashboard_screen.dart` to add a "Share my business" section right below the `Wrap` containing the `_StatCard`s (around line 170).
   - Add a `GlassCard` that simulates a shareable link card (OpenGraph preview with logo, name, tagline) and contains a "Share my business" button.
   - The button will trigger a `ScaffoldMessenger.of(context).showSnackBar` with "Link copied! Share to Instagram/WhatsApp/X".
   - Use `run_in_bash_session` to `cat srcs/app/lib/screens/dashboard_screen.dart` to verify changes.

3. **Viral Storefront**
   - Use `replace_with_git_merge_diff` on `srcs/app/lib/screens/dashboard_screen.dart` to append a "View Live Storefront" button inside the "Share my business" card or alongside it.
   - Clicking the button will open a `showDialog` with a simulated mobile storefront (375px wide container).
   - This simulated storefront will contain a footer reading exactly "Built with OHC — Start your free business →" to satisfy the Viral Storefront requirement.
   - Use `run_in_bash_session` to `cat srcs/app/lib/screens/dashboard_screen.dart` to verify changes.

4. **Verify Tests**
   - Use `run_in_bash_session` to execute `bazelisk test //srcs/app/...` and ensure all tests pass.

5. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**

6. **Submit changes.**
