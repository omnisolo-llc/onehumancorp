1. **Understand the problem**:
   The test suite `//src/server/orchestration:orchestration_test` has started timing out in CI (taking > 60s, which is the default timeout limit for normal tests when there's many tests).
   This is primarily due to multiple retry mechanisms inside `src/server/orchestration/service.go` which are sleeping up to 2 seconds for 5 iterations each time a `minimaxClientImpl` method fails. Over several failure tests like `TestMinimaxClientReasonFailure`, `TestMinimaxClientReasonEmptyResponse`, `TestMinimaxClientGenerateEmbeddingFailure`, and others in `service_grpc_test.go` and `service_test.go`, this translates into over 10-20 seconds per test, eventually breaching the 60 second default Bazel test timeout threshold when run in sequence or parallel depending on load.

2. **Formulate a solution**:
   Introduce a package-level variable `minimaxRetryDelay` in `src/server/orchestration/service.go`, initialized to `1 * time.Second`.
   Introduce another variable `minimaxOverloadRetryDelay` initialized to `2 * time.Second`.
   Replace the hardcoded `time.Sleep(1 * time.Second)` and `time.Sleep(2 * time.Second)` inside `minimaxClientImpl` with these variables.
   In `src/server/orchestration/minimax_client_test.go`, `src/server/orchestration/service_test.go`, and other test files testing Minimax, replace the delay with `time.Millisecond` or `0` for the duration of the tests to make them pass almost instantaneously, solving the timeout completely.

3. **Verify locally**:
   Change the variables inside tests using a custom helper or directly in the tests `minimaxRetryDelay = 1 * time.Millisecond`.
   Run `bazelisk test //src/server/orchestration:orchestration_test` and check its duration. It should drop dramatically from 60+s to <5s.

4. **Code changes**:
   In `src/server/orchestration/service.go`:
   ```go
   var minimaxRetryDelay = 1 * time.Second
   var minimaxOverloadRetryDelay = 2 * time.Second
   ```
   And substitute the `time.Sleep(...)` calls.
   In `src/server/orchestration/minimax_client_test.go` and other relevant test files, override them.
