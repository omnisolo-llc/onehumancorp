1. Add E2E test for the Business Setup Wizard.
    - Create a new file `src/app/test/cuj_business_setup_e2e_test.dart` containing an E2E widget test for `BusinessSetupWizardScreen`.
    - Mock the HTTP client as required for E2E tests, verifying the network interaction doesn't crash the UI and the flow works correctly.
    - Update `src/app/BUILD.bazel` to include this new file in a `flutter_test` target called `cuj_business_setup_e2e_test`.
    - Ensure it is added to the `cuj_e2e_tests` test suite.

2. Complete pre commit steps
   - Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.

3. Submit the change.
   - Run `bazelisk test //...` to ensure everything passes and then submit the changes.
