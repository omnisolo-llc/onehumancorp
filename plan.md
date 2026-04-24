1. **Refactor Loading States to use ShimmerLoading:**
   - I will use `run_in_bash_session` to run a Python script that replaces `loading: () => const Center(child: CircularProgressIndicator())` and similar constructs with `loading: () => const Center(child: ShimmerLoading())`.
   - The script will modify the following exact files: `src/app/lib/screens/security_screen.dart`, `src/app/lib/screens/logs_screen.dart`, `src/app/lib/screens/dashboard_screen.dart`, `src/app/lib/screens/ai_config_screen.dart`, `src/app/lib/screens/settings_screen.dart`, `src/app/lib/screens/agents_screen.dart`, `src/app/lib/screens/meetings_screen.dart`, `src/app/lib/screens/ongoing_management_wizards.dart`, `src/app/lib/screens/skills_screen.dart`, `src/app/lib/screens/orchestration/task_list_screen.dart`, `src/app/lib/screens/channels_screen.dart`, `src/app/lib/screens/diagnostics_screen.dart`.
   - The script will also inject `import 'package:ohc_app/widgets/shimmer_loading.dart';` into these files. I have already executed a version of this script, but I will write and run it again to catch any edge cases with `loading: () => Center(child: CircularProgressIndicator())` missing the `const`.

2. **Verify Refactoring:**
   - I will use `run_in_bash_session` to run `git diff` to verify the modified files now contain `ShimmerLoading`.

3. **Add Playwright E2E Test:**
   - I will use `write_file` to create a Playwright E2E test `src/app/e2e/cuj_loading_state_e2e.spec.ts` that logs into the dashboard and verifies navigation without crashing.

4. **Verify Add Playwright E2E Test:**
   - I will use `run_in_bash_session` to run `cat src/app/e2e/cuj_loading_state_e2e.spec.ts` to verify its content was written correctly.

5. **Verify App UI and Tests:**
   - I will use `run_in_bash_session` to run `cd src/app && flutter test test/dashboard_screen_loading_test.dart` to ensure the widget test passes.
   - I will use `run_in_bash_session` to run `cd src/app/e2e && pnpm install && npx playwright test cuj_loading_state_e2e.spec.ts` to verify the E2E test runs successfully. I will also make sure to use `bazelisk test //src/server/...` to test backend and `bazelisk test //src/app/...` to test flutter UI.

6. **Complete Pre Commit Steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit:**
   - I will stage my files using `git add` in `run_in_bash_session`.
   - I will use `request_code_review`.
   - I will submit the changes using `submit`.
