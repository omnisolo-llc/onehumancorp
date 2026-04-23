1. **Create `website_builder_wizard_screen.dart`**
   - Use `run_in_bash_session` to write `srcs/app/lib/screens/website_builder_wizard_screen.dart` containing a 5-step `ConsumerStatefulWidget` for the website builder wizard with OHC styling (`GlassCard`).

2. **Update `router.dart`**
   - Use `run_in_bash_session` to execute a python script that accurately patches `srcs/app/lib/router.dart` with a regex to insert the import, route, and sidebar navigation item.

3. **Add Flutter Unit Test**
   - Use `run_in_bash_session` to write `srcs/app/lib/screens/website_builder_wizard_screen_test.dart` containing full coverage tests for the widget. (Note: placing it in `srcs/app/lib/screens` will make it automatically picked up by the `//srcs/app/lib/screens:all_tests` glob used in `srcs/app/BUILD.bazel` for `flutter_unit_tests`).

4. **Add E2E Playwright Test in Go**
   - Use `run_in_bash_session` to write the new E2E test into `srcs/tests/e2e/website_builder_wizard_test.go` implementing the UI test using `loginAsAdmin` and `newPage`.
   - Update `srcs/tests/e2e/BUILD.bazel` using `run_in_bash_session` to inject the new test file `website_builder_wizard_test.go` into the existing `e2e_wizard_test` target using a python patch script.

5. **Run tests**
   - Use `run_in_bash_session` to run tests using `bazelisk test //... > test_output.log 2>&1 &` and monitor progress. Wait for it to finish and then verify results with `tail -n 50 test_output.log`.

6. **Pre-commit Steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit Change**
   - Use the `submit` tool to finalize the changes with a PR title starting with "🧙 Wizard: Website Builder Onboarding".
