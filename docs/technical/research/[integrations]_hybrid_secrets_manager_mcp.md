# Title: Integration Blueprint: Hybrid Secrets Manager MCP

## Problem Statement
OHC supports both Cloud-native (multi-tenant) and Standalone (single-user) modes. Local agents running in Standalone mode require secure, encrypted access to personal API keys and tokens, while Cloud agents need secure, tenant-isolated access to cluster secrets. A unified Model Context Protocol (MCP) tool is missing to abstract secret retrieval across both environments securely.

## Research Report
Existing MCP implementations generally rely on static environment variables (`.env`) or plaintext local files, which pose a security risk in a hybrid ecosystem. OHC's "Unfair Advantage" is its ability to seamlessly transition from local execution to cloud deployment. By introducing a Hybrid Secrets Manager MCP, local execution can leverage the host OS keyring or an encrypted SQLite vault, while cloud execution dynamically routes requests to a secure tenant-isolated backend (e.g., Vault or Kubernetes Secrets via SPIFFE).
- **Competitors:** Most frameworks (like Claude Code) assume local `.env` execution and do not scale to multi-tenant boundaries natively.
- **Proposed Solution:** Implement an application-level Hybrid Secrets Manager MCP tool providing a unified `GetSecret` API. The backend driver determines the storage mechanism based on the `OHC_MULTITENANT` flag.

## Design Doc
**Architecture:**
- Add a new package `src/server/lib/integrations/hybrid_secrets/`.
- Introduce a `SecretsManager` that implements the MCP Tool interface.
- Dynamically load the appropriate driver:
  - `Standalone`: Local encrypted SQLite or OS keyring driver.
  - `Cloud`: SPIFFE-authenticated tenant secret store.

**API Contracts:**
- `GetSecret(ctx context.Context, key string) (string, error)`
- `ListSecretKeys(ctx context.Context) ([]string, error)` (Returns metadata only, never values)

**DB Schema Changes:**
- None required.

**Security:**
- Cloud mode MUST validate `organization_id` to strictly enforce cross-tenant isolation.
- Secrets MUST NEVER be logged or emitted to telemetry.

## Implementation Prompt
"Implement the Hybrid Secrets Manager MCP tool in `src/server/lib/integrations/hybrid_secrets/`.
1. Create `secrets.go` defining the `SecretsManager` and its MCP capabilities (`GetSecret`, `ListSecretKeys`).
2. Implement environment-agnostic logic. Check `OHC_MULTITENANT` to determine the driver.
3. For Standalone mode, implement a basic local driver (e.g., reading from an encrypted local path or OS mechanism).
4. For Cloud mode, implement a mockable tenant-aware secret resolver. Ensure tenant isolation using `organization_id`.
5. Create tests in `secrets_test.go`. Use `t.TempDir()` for isolated local testing and mock the cloud driver. Ensure 100% test coverage.
6. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P1

## Estimated Scope
Medium
