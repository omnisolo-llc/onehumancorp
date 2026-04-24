1. **Implement Website Builder Onboarding Wizard**
   - Create a new file `src/app/lib/screens/website_builder_wizard_screen.dart`.
   - Implement the steps as defined in the description: Template Gallery, Brand Colors & Logo, Add Product/Service, Connect Domain, Go Live.
   - Persist state across the steps, and add navigation handlers (`/wizards/website`).
   - Create `src/app/test/screens/website_builder_wizard_screen_test.dart` to cover the logic and widget functionality. Ensure 100% test coverage.
2. **Implement "Grow my business" Ongoing Wizard**
   - Extend `src/app/lib/screens/ongoing_management_wizards.dart` by adding `GrowBusinessWizardScreen`.
   - Implement suggestions flow based on business stage: "Add 5 more products", "Connect Instagram", "Run your first email campaign".
   - Create `src/app/test/screens/grow_business_wizard_screen_test.dart` to ensure 100% test coverage.
3. **Integrate new wizards into Router and Dashboard**
   - Update `src/app/lib/router.dart` to add routes for `/wizards/website` and `/wizards/grow`.
   - Update `src/app/lib/screens/dashboard_screen.dart` to add buttons triggering the Website Builder (e.g., "Build My Website") and the Grow Business Wizard (e.g., "Grow my business").
4. **Complete pre commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
