---
status: PENDING
agent: Researcher
priority: P1
---

# Title: Implement Hybrid Secrets Management MCP Proxy

## Problem Statement
The OHC Hybrid Architecture seamlessly shifts workloads between Standalone Desktop (local environment) and Cloud (Multi-tenant Kubernetes). However, when agents require external API keys or credentials, they currently lack a unified, secure access interface. In Standalone mode, secrets are stored locally, while in Cloud mode, they rely on Kubernetes Secrets or external Vaults. This fragmentation prevents agents from executing identical reasoning logic across both paradigms safely.

## Research Report
- **Market Context**: External tools like Claude Code and Replit Agent rely entirely on local environment variables or purely cloud-native secret stores, respectively. There is no unified hybrid model context protocol that dynamically adapts to the deployment topology.
- **OHC Requirement**: OHC requires a "Hybrid Secrets Management MCP Proxy" that dynamically exposes a standardized secret retrieval tool for agents.
- **Security Model**: The MCP must enforce strict `auth.Claims` isolation in Cloud mode to prevent cross-tenant secret access, while falling back to local host-based encrypted keychains in Standalone mode.

## Design Doc
- **Module Path**: `srcs/server/tools/secretsmcp`
- **Architecture**:
  - Implement the MCP server adhering to `srcs/server/tools/tools.go`.
  - Expose an MCP tool called `get_secret`.
  - Implement an interface `secrets.Provider` with `GetSecret(ctx context.Context, key string) (string, error)`.
  - Create `LocalSecretsProvider` that reads from `.ohc/secrets.json` or `.env`.
  - Create `CloudSecretsProvider` that maps to tenant-scoped Kubernetes Secrets or a simulated DB table.
  - Use `NewProviderFactory` to inject the correct provider dynamically based on `OHC_MULTITENANT`.

## Implementation Prompt
Hello Implementer agent!
1. Abstract the secret fetching logic behind an interface `secrets.Provider` with the method `GetSecret(ctx, key)`.
2. Implement `LocalSecretsProvider` reading from a simple local `.env` or JSON file.
3. Implement `CloudSecretsProvider` that enforces tenant isolation via `auth.ClaimsFromContext(ctx).OrganizationID`.
4. Create an MCP server in `srcs/server/tools/secretsmcp` that uses this provider to expose a `get_secret` tool to agents.
5. Ensure the tool correctly handles unauthorized access attempts.
6. Write unit tests achieving >90% coverage for the `secretsmcp` package.

## Priority
P1

## Estimated Scope
Small
