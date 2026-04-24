1. **Set Up Website Builder Base Screen & State Manager:**
   - I will create `src/app/lib/screens/website_builder_wizard_screen.dart`.
   - Implement the `WebsiteBuilderState` class containing the required state variables (selected template, brand colors, product name, domain choice).
   - Implement the `WebsiteBuilderNotifier` (Riverpod) to handle state transitions between the 5 wizard steps and updates to state variables.
   - Use the `read_file` tool to verify the structure and syntax.

2. **Implement Wizard Steps 1-3 (Template, Brand, Product):**
   - In `website_builder_wizard_screen.dart`, implement the `WebsiteBuilderWizardScreen` widget using `Stepper` or `AnimatedSwitcher` for step navigation.
   - Step 1: Implement Template Gallery with full-bleed cards and mini-previews.
   - Step 2: Implement Brand & Logo with color palette picker and "Generate a logo for me" button.
   - Step 3: Implement Add Product/Service with an inline form and AI description generator toggle.
   - Use the `read_file` tool to verify the syntax.

3. **Implement Wizard Steps 4-5 (Domain, Go Live):**
   - Step 4: Implement Domain connection options (free subdomain, use own domain, buy domain).
   - Step 5: Implement Go Live preview with a Publish button that copies the link to the clipboard and redirects to the dashboard.
   - Implement Progressive Disclosure standard mode and advanced mode toggle for domain config.
   - Use the `read_file` tool to verify the syntax.

4. **Update App Router:**
   - I will modify `src/app/lib/router.dart` to add a new route: `/wizards/website-builder` pointing to `WebsiteBuilderWizardScreen`.
   - Use `read_file` to verify the modified `src/app/lib/router.dart`.

5. **Add Entry Point to Dashboard:**
   - I will modify `src/app/lib/screens/dashboard_screen.dart` to include an `OutlinedButton` labeled "Build My Website" linking to `/wizards/website-builder`, next to the "Billing & Credits" button.
   - Use `read_file` to verify the modified `src/app/lib/screens/dashboard_screen.dart`.

6. **Add Unit Tests for State and UI:**
   - I will create `src/app/test/screens/website_builder_wizard_screen_test.dart`.
   - Write tests to verify state transitions (nextStep, prevStep).
   - Write tests to validate the initial layout (renders Step 1).
   - Write tests to verify that the final submission triggers router navigation.
   - Use `read_file` to verify the structure and syntax.

7. **Add E2E Widget Test:**
   - I will create `src/app/test/cuj_website_builder_e2e_test.dart`.
   - Write an E2E test that starts by logging in via the home page, clicks 'Build My Website' on the dashboard, navigates through all 5 wizard steps via UI interactions (clicking 'Next', selecting domain options), and asserts the final publish state is reached.
   - Use `read_file` to verify the structure and syntax.

8. **Test the changes:**
   - Run all relevant tests via `bazelisk test //...` to ensure there are no regressions.

9. **Pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

10. **Completion:**
    - Finalize and submit the task.
