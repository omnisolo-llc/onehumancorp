# Title: [integrations] Hybrid MCP Secrets Vault

## Problem Statement
OHC supports both Cloud-native (Kubernetes/Postgres) and Standalone (SQLite) modes. However, agents currently lack a unified, secure method for retrieving sensitive API keys (e.g., Jira, Stripe, OpenAI) that respects the deployment footprint. In Cloud mode, secrets must be fetched securely from Kubernetes Secrets (via SPIFFE). In Standalone mode, they must be fetched securely from the local OS Keychain (macOS Keychain, Windows Credential Manager) rather than plaintext files.

## Research Report
Market research indicates that most frameworks like CrewAI and AutoGen rely heavily on local `.env` files or hardcoded environment variables, which are vulnerable in multi-tenant environments and clumsy for local standalone deployments. OHC's Hybrid Architecture demands an MCP Tool that dynamically routes the secret retrieval based on the environment `OHC_MULTITENANT` mode without the agent needing to know the underlying infrastructure.

## Design Doc
**Architecture:**
- Add a new package `src/server/lib/integrations/secrets_vault/`.
- Introduce a `VaultManager` that implements the MCP Tool interface.
- Dynamically route based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Utilize a Kubernetes client to fetch secrets from the agent's specific namespace.
- **Standalone Mode:** Utilize a Go library like `github.com/zalando/go-keyring` to interact securely with the OS native keychain.

**API Contracts:**
- `GetSecret(ctx context.Context, key string) (string, error)`
- `PutSecret(ctx context.Context, key string, value string) error`

**Security:**
- Must validate `organization_id` in cloud mode.
- Ensure secrets are never logged or stored in LangGraph state explicitly (redact before serialization).

## Implementation Prompt
"Implement the Hybrid MCP Secrets Vault tool in `src/server/lib/integrations/secrets_vault/`.
1. Create `vault.go` defining the `VaultManager` and its MCP capabilities (`GetSecret` and `PutSecret`).
2. Implement driver-agnostic logic. To determine if the connection is Cloud, use: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`. For Cloud, use standard K8s Secret APIs. For Standalone, use `github.com/zalando/go-keyring`.
3. Ensure `GetSecret` output is wrapped in a type that triggers `RedactInterfacePII` to prevent PII leakage.
4. Create tests in `vault_test.go` mocking both the OS Keyring and K8s API.
5. Update or create the adjacent `BUILD.bazel` file, ensuring `srcs` array accurately reflects the new files."

## Priority
P1

## Estimated Scope
Medium
