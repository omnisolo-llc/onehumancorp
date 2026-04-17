# MCP Secret Vault Design Document

## Problem
In OHC Hybrid Architecture, managing secrets (API keys, OAuth tokens) requires switching between multi-tenant secure storage (like HashiCorp Vault or Cloud KMS) and local secure storage (like the OS Keychain/Keyring or local encrypted files). Agents executing tasks locally in Standalone Mode need a unified way to request and store secrets without being aware of the underlying environment context. Currently, there is no standardized protocol or MCP tool to bridge this gap securely.

## Solution
Create an `mcp_secret_vault` tool that acts as an abstraction layer. In Cloud Mode, it interfaces with the central PostgreSQL/Vault backend using SPIFFE/SPIRE for identity validation. In Standalone Mode, it interfaces with the host OS Keychain (e.g., using Go's `zalando/go-keyring`).

### Architecture
A unified `mcp_secret_vault` MCP Tool that dynamically resolves the storage backend based on the runtime context (Cloud vs. Standalone).
- **Local**: Uses OS Keyring for single-user encryption.
- **Cloud**: Uses PostgreSQL encrypted columns (or external Vault API) with strict tenant isolation.

### API Contract
The tool will expose an MCP compatible interface:
- `get_secret(key, tenant_id)` - Retrieves a secret by key.
- `set_secret(key, value, tenant_id)` - Stores or updates a secret securely.

### Implementation Details
The tool will have an interface `SecretStorage` with two implementations: `CloudAdapter` and `LocalAdapter`. The tool itself will route to the appropriate adapter based on the environment or initialization configuration.

- `CloudAdapter` will take a `*sql.DB` connection and ensure tenant boundaries are respected.
- `LocalAdapter` will utilize `zalando/go-keyring` using "ohc-standalone" as the service name.

### Security
Ensure secrets are never logged or exposed in telemetry. The MCP tool must perform context-aware sanitization and require explicit agent permissions.

### UI Wireframes
A secure "Secrets Management" panel in the Desktop Mode UI, protected by a user PIN/Biometrics, rendered with Outfit typography and Glassmorphism (20px blur, 200% saturation). *(Implementation of UI is out of scope for this backend task).*
