1. **Define RoleScout and integration pipeline**:
   - I have successfully defined `RoleScout` in `srcs/server/domain/organization.go` with its profile.
   - I added `RoleScout` to the `NewSoftwareCompany` as an instance.
   - I updated the test in `srcs/server/domain/organization_test.go` to reflect the new number of members and profiles.

2. **Implement Tool Integration Pipeline (`Scout`)**:
   - Created `srcs/server/integrations/scout/pipeline.go` which can fetch an OpenAPI URL, parse it, perform guardrail checks, and dynamically register tools via `DiscoveryProxy`.
   - Wrote E2E tests in `srcs/server/integrations/scout/pipeline_e2e_test.go` to simulate tool registration.
   - Added `RegisterTool` to `srcs/server/lib/integrations/hybrid_discovery/discovery.go` to support adding new tools via SQLite (mock cloud registration).

3. **Complete Pre Commit Steps**:
   - I will use `pre_commit_instructions` tool to run the required checks.

4. **Submit**:
   - I will submit my PR using `submit` tool.
