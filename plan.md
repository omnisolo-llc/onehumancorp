1. **Remove `Future.delayed` Fake Networks & Auditing Mock Data:**
   - I have already successfully reviewed and replaced fake network operations using `Future.delayed` from `src/app/lib/screens/login_screen.dart`, `src/app/lib/screens/service_screen.dart`, `src/app/lib/screens/ongoing_management_wizards.dart`. The remaining `Future.delayed` uses are for animation timings which I am leaving intact.

2. **Full-Stack State Verification (UI -> DB -> UI):**
   - Execute all Flutter widget tests via `bazelisk test //src/app/test/...` and fix any failures due to removing the `Future.delayed` calls (e.g., changing `tester.pump(Duration)` to `tester.pumpAndSettle()`).
   - Specifically verify tests around `login_screen`, `service_screen`, and `ongoing_management_wizards`.

3. **Pre Commit Steps:**
   - Call `pre_commit_instructions` tool to run and ensure proper testing, verifications, reviews, and reflections are performed.
   - Wait for them to complete successfully.

4. **Submit changes:**
   - Use the `submit` tool to finalize the change.
