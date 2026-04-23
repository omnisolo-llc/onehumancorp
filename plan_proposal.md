1. **Implement Website Builder Wizard Screen:**
    - Create a new file `srcs/app/lib/screens/website_builder_wizard_screen.dart`.
    - Implement a progressive wizard based on the "Website Builder Onboarding" requirements in the task description.
    - Requirements:
        - Steps: Template gallery, Brand colors & logo, Add first product/service, Connect domain, Go Live.
        - UI constraints: Glassmorphism (`GlassCard`), progressive disclosure (Expert mode toggle), mobile-first.
        - State management using Riverpod.
    - Make sure to use the exact specified text copies like "Use this template →" and "Publish".

2. **Add Website Builder Wizard to Router:**
    - Update `srcs/app/lib/router.dart` to add a new route for the website builder (e.g. `/wizard/website`).
    - Add a button in the dashboard or wherever it should be accessible, or just make it accessible via route for testing. Since the task says "Triggered after the setup wizard, or when the user taps 'Build My Website'", I'll add a trigger button on the Dashboard.

3. **Write E2E Test for the Wizard:**
    - Create `srcs/app/test/screens/website_builder_wizard_screen_test.dart` to verify state transitions and UI rendering.
    - Create an E2E test `srcs/app/test/cuj_website_builder_e2e_test.dart` meeting the CUJ standard (start from login, navigate to wizard, complete flow, assert final state).

4. **Pre-commit Checks:**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

5. **Submit Change:**
    - Use `bazelisk test //srcs/app/...` to verify all tests pass.
    - Submit PR.
