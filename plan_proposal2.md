1. **Implement Website Builder Wizard Screen:**
    - Use `write_file` to create `srcs/app/lib/screens/website_builder_wizard_screen.dart` with the required Riverpod state and Glassmorphism UI components. Requirements: Template gallery, Brand colors & logo, Add first product/service, Connect domain, Go Live. Use the text "Use this template →" and "Publish".

2. **Verify Wizard Screen Creation:**
    - Run `run_in_bash_session` to execute `cat srcs/app/lib/screens/website_builder_wizard_screen.dart` to verify file creation.

3. **Add Website Builder Wizard to Router:**
    - Use `replace_with_git_merge_diff` to modify `srcs/app/lib/router.dart` and add a new route `/wizard/website`.
    - Use `replace_with_git_merge_diff` to modify `srcs/app/lib/screens/dashboard_screen.dart` to add a "Build My Website" button which routes to `/wizard/website`.

4. **Write Tests for the Wizard:**
    - Use `write_file` to create `srcs/app/test/screens/website_builder_wizard_screen_test.dart` to verify state transitions and UI rendering.
    - Use `write_file` to create `srcs/app/test/cuj_website_builder_e2e_test.dart` meeting the CUJ standard (start from login, navigate to wizard via dashboard, complete flow, assert final state).

5. **Verify Test File Creation:**
    - Run `run_in_bash_session` to execute `ls -l srcs/app/test/screens/website_builder_wizard_screen_test.dart srcs/app/test/cuj_website_builder_e2e_test.dart` to ensure they were written successfully.

6. **Run Test Suite:**
    - Use `run_in_bash_session` with command `export PATH=$PATH:$HOME/go/bin && bazelisk test //srcs/app/...` to verify all tests pass.

7. **Pre-commit Checks:**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit Change:**
    - Use the `submit` tool to conclude the task and submit the PR.
