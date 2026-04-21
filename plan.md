1. Add `capabilities JSONB DEFAULT '[]'::jsonb` to `agent_session_data` table by writing a new SQL migration.
2. Update the `Session` model struct or define a new one to include `Capabilities []string`
3. Implement `CapabilityAuthorizer` component in `srcs/server/harness/authz/authorizer.go`
4. Register the new metric `CapabilityViolationTotal` in `telemetry.go` to log capability violations. Use the telemetry package inside `CapabilityAuthorizer` to record a violation when one occurs.
5. Create tests for `CapabilityAuthorizer`.
6. Complete pre-commit steps to make sure proper testing, verifications, reviews and reflections are done.
