1. **Update Frontend UI & State**
   - Refactor `src/app/lib/screens/business_setup_wizard_screen.dart` to match the 7 new steps (Welcome, Business type, Business name & description, What do you sell, Payment preference, Administrator account, Review & Launch).
   - The steps will be exactly as required by the problem description, using `AnimatedSwitcher` to transition between them, and making them mobile responsive.

2. **Update Testing & Build configuration**
   - Ensure the new E2E test `e2e_business_setup_test.go` is actually run. We will add it to `src/tests/e2e/BUILD.bazel` to be compiled and executed in one of the e2e test targets (e.g. `e2e_wizard_test`).
   - Fix `src/tests/e2e/e2e_business_setup_test.go` to match the newly implemented wizard UI flow exactly.
   - Fix `src/app/test/screens/business_setup_wizard_screen_test.dart` to match the new UI.

3. **Backend API updates**
   - The current UI calls `/api/wizard/configure` on finish to save data. Currently `handleWizardConfigure` expects fields in `req.Extras`. Our UI already sends this in the payload under `extras`, so the backend implementation `src/server/dashboard/handlers_wizard.go` might already be sufficient to persist this to `settings.Extras`. We just need to make sure `admin_name` and `admin_email` are properly accepted, but `req.Extras` acts as a map of strings so it will accept them without backend changes.
   - We will verify if we need to adjust the backend handler, but since `Extras map[string]string` is dynamically populated, it should store the wizard data perfectly fine.

4. **Testing validation**
   - `bazelisk test //... > plan_test.log 2>&1 &` to verify everything works and E2E passes.

5. **Pre-commit checks**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
