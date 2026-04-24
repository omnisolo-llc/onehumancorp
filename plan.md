1. Use `replace_with_git_merge_diff` to modify `SPIFFEAuthInterceptor` and `SPIFFEStreamInterceptor` in `srcs/server/orchestration/auth_interceptor.go`. The logic currently allows any `.ohc.global` suffix. I will restrict it to explicitly allow only `eu.ohc.global` and `eu-west.ohc.global` to match `validDomains` in `srcs/server/interop/types.go`.
2. Use `replace_with_git_merge_diff` to modify `validateSPIFFEID` in `srcs/server/agents/grpc/auth.go` to explicitly allow `eu.ohc.global` and `eu-west.ohc.global` instead of an open `.ohc.global` suffix.
3. Use `replace_with_git_merge_diff` to modify `validate_spiffe_id` in `srcs/server/agents/builtin/src/auth.rs` to explicitly allow `eu.ohc.global` and `eu-west.ohc.global` instead of an open `.ohc.global` suffix.
4. Use `run_in_bash_session` with `cat` to read the modified files and verify the edits were applied correctly.
5. Use `replace_with_git_merge_diff` to update `srcs/server/orchestration/auth_interceptor_test.go` so the test cases reflect the strict whitelist rather than open suffix matching.
6. Use `run_in_bash_session` with `cat` to read `auth_interceptor_test.go` and verify the test changes were applied correctly.
7. Execute `bazelisk test //...` to verify all tests pass.
8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
9. Use the `submit` tool to commit the code.
