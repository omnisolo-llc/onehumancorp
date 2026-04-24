1. **Create Business Share Widget**:
   - Write the `BusinessShareWidget` code to a new file `src/app/lib/widgets/business_share_widget.dart`.
   - The widget will display an OpenGraph-style preview (link card) with a logo, business name, and tagline.
   - Include a "Share my business" button that copies the public storefront link to the clipboard and shows a `SnackBar`.
2. **Verify Widget Creation**:
   - Use `read_file` to verify the file `src/app/lib/widgets/business_share_widget.dart` was created correctly.
3. **Integrate into Dashboard**:
   - Modify `src/app/lib/screens/dashboard_screen.dart` to add the required import and inject the `BusinessShareWidget` directly into the `_DashboardContent` widget.
4. **Verify Dashboard Modification**:
   - Use `git diff src/app/lib/screens/dashboard_screen.dart` to confirm the modification.
5. **Explore Tests Build File**:
   - Use `read_file` to inspect the contents of `src/tests/e2e/BUILD.bazel` to understand the existing targets.
6. **Write E2E Test**:
   - Write the Playwright E2E test code to a new file `src/tests/e2e/viral_storefront_test.go`.
   - Update `src/tests/e2e/BUILD.bazel` to include the new test file in the `srcs` array.
7. **Verify Test File Creation**:
   - Use `read_file` to verify the test file `src/tests/e2e/viral_storefront_test.go` and `git diff src/tests/e2e/BUILD.bazel` to verify the BUILD.bazel modification.
8. **Run Tests**:
   - Run `bazelisk test //...` to ensure all tests pass and no regressions were introduced.
9. **Pre-commit Steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
10. **Submit Change**:
   - Submit the new feature with an appropriate descriptive commit message.
