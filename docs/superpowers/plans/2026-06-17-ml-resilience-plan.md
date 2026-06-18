1. **Modify `src/server/db.rs`**
   - Locate `execute_with_retry` function.
   - Change `let max_attempts = 10;` to `let max_attempts = 3;`.

2. **Modify `src/server/orchestration/state/parity_test.rs`**
   - Update `test_execute_with_retry_chaos_exhaustion` to assert 3 instead of 10.

3. **Verify**
   - Run `cargo test -p ohc-mono --manifest-path Cargo.toml -- orchestration::state::parity_test::test_execute_with_retry_chaos_exhaustion` to ensure tests pass.
   - Run `bazelisk test //src/server/orchestration/state:parity_test` if possible.

4. **Pre-commit Check**
   - Run pre-commit instructions and apply formatting/linting as necessary.
   - Ensure the `bazelisk test //...` is green.
