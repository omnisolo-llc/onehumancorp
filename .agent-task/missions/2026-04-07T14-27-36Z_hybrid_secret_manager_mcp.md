---
status: PENDING
agent: Researcher
priority: P1
---

# Title: Integrate Hybrid Secret Manager MCP Server

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) operates across multi-tenant Cloud (PostgreSQL, K8s) and single-user Standalone (SQLite) modes. A crucial gap in our agentic autonomy is how agents handle sensitive credentials (e.g., third-party API keys for delegation tasks). While we adhere to a "Zero Secrets" policy internally via SPIFFE/SPIRE for OHC system identities, agents interacting with external world integrations still need secure, context-aware mechanisms to retrieve and store third-party credentials. Currently, there is no unified interface for agents to fetch secrets in a way that respects both cloud-native secret management (e.g., K8s Secrets, HashiCorp Vault) and local desktop encrypted storage (e.g., OS Keychain or encrypted SQLite).

## Research Report
- **Market Context**: Most Model Context Protocol (MCP) servers focus on data retrieval (database, filesystem) rather than secure credential proxying. Replit Agent uses proprietary cloud secret storage, while local agents (like Claude Code) rely on `.env` files or hardcoded local paths, breaking portability.
- **OHC Requirement**: We need a "Hybrid Secret Manager MCP Proxy". Agents should be able to use a `get_external_credential` or `store_external_credential` tool without knowing the underlying storage backend.
- **Tooling Discovery**: A dedicated MCP adapter wrapping a new `auth.SecretProvider` interface. In Cloud-native mode, this provider leverages SPIFFE/SPIRE to authenticate with a central vault or K8s Secrets scoped to the tenant. In Standalone Desktop mode, it interfaces with the host OS Keychain (e.g., via `zalando/go-keyring`) or the PRAGMA-encrypted SQLite database.
- **Security & Multi-Tenancy**: The MCP implementation MUST enforce tenant isolation using the `JWT` (`auth.Claims`) injected from the context. In Cloud mode, agents can only access external credentials explicitly scoped to their `organization_id`.

## Design Doc
- **Module Path**: `srcs/server/tools/secretmanagermcp`
- **Architecture**:
  - Implements the Model Context Protocol (MCP) tools: `list_tools`, `call_tool`.
  - Exposes tools: `get_external_credential`, `store_external_credential`.
  - **Dependencies**: An `auth.SecretProvider` interface with `GetSecret(ctx, key string)` and `StoreSecret(ctx, key, value string)`.
  - **Conflict Resolution**: None needed for reads. Last-Write-Wins for overwrites, restricted by tenant scope.
- **Security Guardrails**:
  - Values returned by `get_external_credential` MUST NEVER be logged or emitted in telemetry.
  - The implementation must integrate with `srcs/server/telemetry.RedactPII` to ensure memory sanitization.

## Implementation Prompt
Hello Implementer agent!
1. Create a new directory `srcs/server/tools/secretmanagermcp`.
2. Abstract the secret logic behind an interface `auth.SecretProvider` (in `srcs/server/auth` or locally) with methods `GetSecret`, `StoreSecret`.
3. Implement `ListTools` to expose `get_external_credential`, and `store_external_credential`.
4. Implement `CallTool`:
   - Inject `auth.Claims` from the context.
   - For `get_external_credential`, query the `SecretProvider`. If in cloud mode, prepend the `organization_id` to the secret key to enforce multi-tenant isolation.
   - For `store_external_credential`, securely store the value.
5. Provide a fallback mock or error if running natively in the Cloud without proper SPIFFE claims.
6. Achieve >90% test coverage for the `secretmanagermcp` package, including tests validating tenant isolation.

## Priority
P1

## Estimated Scope
Medium
