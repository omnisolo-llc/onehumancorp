<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Scout 🔍 (Resource Scout & Tool Integrator)

## Problem Statement
The One Human Corp (OHC) ecosystem relies on AI agents interacting with various external platforms via the `integrations` module. However, maintaining and discovering new capabilities, API endpoints, or missing integration workflows is currently a manual process. There is a need for an automated "Scout" agent or tool integrator capable of querying APIs, discovering resource schema details, and dynamically fetching schema definitions to auto-generate integration stubs or update existing MCP layers.

## Research Report
The current OHC architecture implements the `Integration` interface (in `srcs/server/integrations/catalog.go`) with an in-memory `Registry` that abstracts external services (Git, Chat, Issues) into unified agent interfaces.
Presently, MCP implementations (in `srcs/server/tools/`) map internal functionalities or external APIs rigidly. For a Swarm Intelligence to dynamically adapt to new APIs, a Scout mechanism must systematically explore available schemas (like OpenAPI definitions) and translate them into compatible tools.

Key investigations:
- **Capabilities of Scout:** Needs to parse OpenAPI/Swagger specs, introspect REST or GraphQL endpoints, and output Go structs matching `mcp.Tool` formats.
- **Integration with the Registry:** Should be able to register dynamically discovered endpoints.
- **Security:** Requires strict validation to prevent malicious schema injection or SSRF when exploring remote APIs.

## Design Doc
1. **Scout Engine (`srcs/server/tools/scoutmcp/`)**:
   - Create a new MCP module `scoutmcp` that acts as the core engine for schema discovery.
   - Implement `DiscoverAPI(url string)` which fetches and parses OpenAPI JSON/YAML.
   - Implement `GenerateTool(operationID string, schema map[string]interface{}) *mcp.Tool`.

2. **Integration Registration**:
   - Provide an interface or CLI command to pass a newly scouted API directly to the `integrations` module.

3. **User Interaction (Wizard)**:
   - Provide `WizardSteps()` in a new `ScoutIntegration` (if represented as a catalog item) allowing the single human operator to approve dynamically scouted tool connections.

4. **Testing Context**:
   - Mock a fake external OpenAPI server in the test suite to validate Scout's parsing and tool translation capabilities without relying on live APIs.

## Implementation Prompt
Implement a new `scoutmcp` module within `srcs/server/tools/` and a `ScoutIntegration` within `srcs/server/integrations/`.

- **Step 1:** Create `srcs/server/tools/scoutmcp/scout.go`. It should define an MCP provider that can take an OpenAPI 3.0 URL as input, fetch it, and return a list of parsed operational endpoints as potential agent tools.
- **Step 2:** Create `srcs/server/integrations/scout/scout.go`. Implement the `Integration` interface (`Metadata()` and `WizardSteps()`). This integration acts as a dynamic wrapper allowing the human operator to approve new scouted APIs.
- **Step 3:** Add the `ScoutIntegration` to the `Catalog` in `srcs/server/integrations/catalog.go`.
- **Step 4:** Write comprehensive unit and E2E tests for `scoutmcp` using a mocked HTTP server returning a dummy OpenAPI specification. The tests must satisfy the 100% test coverage requirement.

Ensure all Go files use the `github.com/onehumancorp/mono` import path correctly, and `BUILD.bazel` files are updated.

## Priority
P1 - High Priority. Enables autonomous capability expansion for the swarm.

## Estimated Scope
- 2-3 weeks.
- Requires changes to `srcs/server/tools/`, `srcs/server/integrations/`, and new Go test suites.
- Requires new Bazel build targets for the `scout` packages.

</div>