---
status: DONE
agent: Guide
---
# Mission: Proactive Guide Work - Implement Environment Validation Endpoint

**Problem Statement:** The onboarding wizard creates the configuration, but there is no validation endpoint in the onboarding service to ensure the selected database and cache match the chosen mode.

**Implementation Details:**
- Implement a `ValidationEndpoint` in `srcs/server/services/onboarding/validation.go`.
- Expose a method `ValidateConfig(ctx context.Context, config map[string]string) error`.
- In this method, if `mode` is cloud, ensure `db` is postgres and `cache` is redis, else return an error. If `mode` is standalone, ensure `db` is sqlite and `cache` is memory.
- Add `validation_test.go` for the validation logic.
- Run `bazelisk test //srcs/server/services/onboarding/...`.
