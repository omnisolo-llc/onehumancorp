1. **Implement `UpgradeBottomSheet` Widget**:
    - Use `write_file` to create `src/app/lib/widgets/upgrade_bottom_sheet.dart`.
    - Build a 375px optimized widget that accepts a limit reason (e.g., 'max products' or 'storage').
    - Include friendly non-technical text explaining the limit and a native mobile payment style one-tap upgrade button using `ohc_app/widgets/glass_card.dart` components.

2. **Verify `UpgradeBottomSheet`**:
    - Use `run_in_bash_session` to `cat src/app/lib/widgets/upgrade_bottom_sheet.dart` to verify its creation and contents.

3. **Add Multi-Tenant SaaS Tier Frontend State Management**:
    - Use `write_file` to create a provider `tierProvider` and model `TierStatus` in `src/app/lib/models/tier.dart`.
    - `TierStatus` will hold `tierName`, `productsCount`, `maxProducts`, `aiActionsUsed`, `aiActionsLimit`.
    - The provider should hold default Free tier limits.

4. **Verify `tier.dart`**:
    - Use `run_in_bash_session` to `cat src/app/lib/models/tier.dart` to verify its creation and contents.

5. **Graceful Degradation for AI Customer Success Agent Chat**:
    - Use `replace_with_git_merge_diff` to modify `src/app/lib/widgets/ai_help_chat_widget.dart`.
    - Convert `_ChatBottomSheet` to a `ConsumerWidget`. Check the user's `tierProvider` AI action limits.
    - If `aiActionsUsed >= aiActionsLimit`, fallback the AI text generation to a standard manual text input field with a subtle "Upgrade to restore AI replies" button that triggers `UpgradeBottomSheet`, replacing the disabled "Type your question..." TextField.

6. **Verify `ai_help_chat_widget.dart`**:
    - Use `run_in_bash_session` with `cat` to verify changes in `src/app/lib/widgets/ai_help_chat_widget.dart`.

7. **Product Addition Flow & Exhaustion Simulation**:
    - Use `write_file` to create `src/app/lib/screens/manage_products_screen.dart` with a "Products" screen with an "Add Product" button that triggers the bottom sheet if `productsCount >= maxProducts`.
    - Use `replace_with_git_merge_diff` to add the route for `/products` mapping to `ManageProductsScreen()` inside the `routes:` array within `ShellRoute` in `src/app/lib/router.dart`.
    - Use `replace_with_git_merge_diff` to add the nav item for `/products` in the `children` array of `NavigationDrawer` in the `_Sidebar` class in `src/app/lib/router.dart`.

8. **Verify `manage_products_screen.dart` and `router.dart`**:
    - Use `run_in_bash_session` to `cat` these two files.

9. **E2E Tests for the Exhaustion Limit**:
    - Use `write_file` to create `src/app/test/cuj_tier_upgrade_e2e_test.dart`.
    - Write a widget test simulating a Free tier user attempting to add an 11th product.
    - Verify `UpgradeBottomSheet` is displayed with the correct copy.
    - Test the one-tap upgrade button simulating an upgrade success and the bottom sheet dismissing.

10. **Verify test file**:
    - Use `run_in_bash_session` to `cat src/app/test/cuj_tier_upgrade_e2e_test.dart`.

11. **Run tests**:
    - Use `run_in_bash_session` to run `bazelisk test //src/app/...`. Ensure all tests pass.

12. **Pre-commit**:
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

13. **Submit changes**:
    - Stage changes using `run_in_bash_session` with `git add .`
    - Use `request_code_review`
    - Use `submit` with title "🚀 Nova: [Multi-Tenant SaaS Tier & Upgrade Path UX]".
