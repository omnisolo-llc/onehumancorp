# Title: [mcp] Integration Blueprint: Viper Configuration Management MCP

## Problem Statement
The OHC Hybrid Architecture supports both Cloud-native (multi-tenant PostgreSQL/Redis) and Standalone Desktop (single-user SQLite) modes. Currently, agents in the swarm lack a unified, mode-aware mechanism to dynamically read, write, and observe system and user-level configuration values. In Cloud deployments, configuration changes should be persisted to a centralized database (like Postgres) to distribute updates across all stateless pods serving a specific tenant. In Standalone mode, configurations must be persisted locally (e.g., via SQLite or a local config file). Without a unified Configuration Management MCP, agents are either restricted to hardcoded configurations or must implement redundant, mode-specific logic.

## Research Report
Market analysis shows that many modern agent frameworks either rely on static dotfiles (`.env`, `config.yaml`) which do not scale for multi-tenant environments, or they expect access to a cloud-based Key-Value store, alienating local-first use cases.

The Go ecosystem has a widely adopted configuration library called `Viper` (github.com/spf13/viper). Viper supports reading from JSON, TOML, YAML, HCL, envfile and Java properties config files, reading from environment variables, and reading from remote config systems like etcd or Consul. It also supports watching changes and unmarshaling into structs.

By integrating Viper as the core engine for a Hybrid Configuration Management MCP, OHC can seamlessly manage agent context, application settings, and feature toggles. The MCP will leverage Viper's remote key/value store support for Cloud execution (e.g., pointing it to a multi-tenant aware KV store or adapting it for Postgres), and use Viper's local file watching or SQLite support for low footprint Standalone execution. This reinforces OHC's "Unfair Advantage" of functioning effortlessly across boundaries using an industry-standard library.

### Competitive Analysis
| Capability | Static Config (e.g., .env) | Cloud Key-Value (e.g., Consul) | OHC Hybrid Config MCP (via Viper) |
| :--- | :--- | :--- | :--- |
| **Local Zero-Dependency** | ✅ Yes | ❌ No | ✅ Yes (Viper Local) |
| **Cloud Scale & Distribution**| ❌ No | ✅ Yes | ✅ Yes (Viper Remote) |
| **Dynamic Runtime Updates** | ❌ No | ✅ Yes | ✅ Yes (Viper Watch) |
| **Multi-Tenant Isolation** | N/A | ✅ Yes | ✅ Yes (organization_id scoped via adapter) |

## Design Doc
**Architecture:**
- Create a new package `srcs/server/lib/integrations/viper_config/`.
- Introduce a `ConfigManager` implementing the standard MCP Tool interface, wrapping `viper.Viper` instances.
- Select the storage backend dynamically based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Configure Viper to use a remote KV store (e.g., etcd) or write a custom `remote.crypt` adapter for PostgreSQL. Queries must strictly enforce multi-tenant isolation by appending the `organization_id` to where clauses or KV paths.
- **Standalone Mode:** Configure Viper to read/write from local config files or SQLite.

**API Contracts:**
- `GetConfig(ctx context.Context, key string) (string, error)` (Wraps `viper.GetString`)
- `SetConfig(ctx context.Context, key string, value string) error` (Wraps `viper.Set` and `WriteConfig`/remote write)
- `ListConfigs(ctx context.Context, prefix string) (map[string]string, error)` (Wraps `viper.AllSettings`)

**Security:**
- Enforce rigid `organization_id` isolation in Cloud Mode to prevent cross-tenant data leaks (e.g., using `tenant_id` as the root path in the remote KV store).
- Ensure that sensitive settings (like API keys) are routed through the Secrets Manager rather than the generic Configuration Manager.

## Implementation Prompt
"Implement the Hybrid Configuration Management MCP tool in `srcs/server/lib/integrations/viper_config/` using `github.com/spf13/viper`.
1. Create `config.go` defining the `ConfigManager` and its MCP capabilities (`GetConfig`, `SetConfig`, `ListConfigs`).
2. Implement environment-agnostic logic. To determine if the connection is Cloud, check: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Cloud mode, configure Viper to use a remote backend, ensuring `organization_id` is mandatory and used to scope every query/path.
4. For Standalone mode, configure Viper targeting the local file system or SQLite instance.
5. Create comprehensive tests in `config_test.go`, verifying both the remote behavior (mocked) and the local fallback. Ensure 100% test coverage.
6. Create an E2E test verifying an agent can set and retrieve a configuration value correctly in both simulated modes.
7. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies, including `github.com/spf13/viper`."

## Priority
P1

## Estimated Scope
Medium
