---
status: DONE
agent: Maintainer
---
# 🛡️ Maintainer: [Hybrid Privacy] Strict Opt-in Telemetry Enforcement - Extra Tests Isolation

## Problem Statement
The Hybrid architecture must strictly enforce user data privacy in standalone mode, which requires robust test isolation. While the primary `telemetry_test.go` file was previously updated to remove deprecated `os.Setenv`/`os.Unsetenv` usage in favor of `t.Setenv()`, similar isolated pollution remained in `telemetry_extra_test.go`. These functions were still mutating the global environment variables directly, leading to race conditions and test failures during parallel test execution.

## Research Report
- Evaluated `srcs/server/telemetry/telemetry_extra_test.go`.
- Found multiple instances of `os.Setenv("OHC_STANDALONE", ...)` and `os.Unsetenv("OHC_STANDALONE")`.
- This pattern is dangerous in Go tests because it permanently modifies the test runner's environment without automated cleanup in case of a crash or panic, unlike `t.Setenv()`.

## Design Doc
1. **Refactor Environment Tests**: Modernize `telemetry_extra_test.go` to use `t.Setenv()` rather than manual `os.Setenv` / `os.Unsetenv`.
2. **Remove Manual Cleanup**: Remove all the manual `defer func() { if originalStandalone != "" { os.Setenv(...) } }()` boilerplate.
3. **Clean Imports**: Remove the unused `"os"` package import in `telemetry_extra_test.go`.

## Implementation Prompt
1. In `srcs/server/telemetry/telemetry_extra_test.go`, replace `os.Unsetenv("OHC_STANDALONE")` with `t.Setenv("OHC_STANDALONE", "")`.
2. Remove the manual capture of `originalStandalone := os.Getenv("OHC_STANDALONE")` and the associated deferred cleanup function.
3. Remove the unused `"os"` import from the import block.
4. Verify tests pass.

## Priority
P1

## Estimated Scope
Small
