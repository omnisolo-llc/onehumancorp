1. **Fix `src/server/interop/types.go`**:
   - Update `ValidateSPIFFEID` to replace the hardcoded map of valid domains with the same logic found in `src/server/agents/grpc/auth.go` and `src/server/orchestration/auth_interceptor.go`.
   - Ensure it dynamically checks for any domain ending in `.ohc.global` instead of hardcoding `eu.ohc.global` and `eu-west.ohc.global`.

2. **Run tests**:
   - Run `bazelisk test //src/server/...` to ensure all tests still pass and the logic is sound.

3. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
   - Run the pre commit instructions and obey them.
4. **Submit changes**
