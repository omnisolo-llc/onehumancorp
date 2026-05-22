# Title: [integrations] Hybrid Cache Manager MCP

## Problem Statement
OHC supports both Cloud-native (multi-tenant Postgres/Redis) and Standalone (single-user SQLite) modes. Currently, there is a gap in caching capabilities for agents operating across these environments. In Cloud mode, agents need to leverage a distributed cache like Redis to store ephemeral data, share state across pods, and reduce database load. In Standalone mode, agents need a lightweight local alternative (e.g., in-memory cache or SQLite-backed cache) without requiring a heavy Redis dependency. A unified Model Context Protocol (MCP) tool is missing to abstract these caching mechanisms securely.

## Research Report
Market analysis shows that typical agentic frameworks hardcode caching backends, making them either strictly local-first or cloud-dependent. OHC's Hybrid Architecture demands dynamic capability routing. By introducing an application-level Hybrid Cache Manager MCP tool, agents can use a unified `Get`, `Set`, and `Delete` API. The underlying driver will dynamically resolve to Redis when `OHC_MULTITENANT` is active, or a local in-memory/SQLite cache when running in standalone mode. This ensures high performance in both the Cloud Gateway and local Desktop deployments.

## Design Doc
**Architecture:**
- Add a new package `src/server/lib/integrations/hybrid_cache/`.
- Introduce a `CacheManager` that implements the MCP Tool interface.
- Dynamically load the appropriate driver based on `os.Getenv("OHC_MULTITENANT") == "true"`.
  - `Standalone`: Local in-memory or SQLite-backed cache.
  - `Cloud`: Redis-backed cache via go-redis.

**API Contracts:**
- `GetCache(ctx async context, key string) ([]byte, error)`
- `SetCache(ctx async context, key string, value []byte, ttl time.Duration) error`
- `DeleteCache(ctx async context, key string) error`

**Security:**
- Cloud mode MUST validate `organization_id` to strictly enforce cross-tenant key isolation (e.g., prefix keys with `tenant_id:`).
- Apply `RedactInterfacePII` to cache payloads to prevent PII leakage.

## Implementation Prompt
"Implement the Hybrid Cache Manager MCP tool in `src/server/lib/integrations/hybrid_cache/`.
1. Create `cache.rs` defining the `CacheManager` and its MCP capabilities (`GetCache`, `SetCache`, `DeleteCache`).
2. Implement environment-agnostic logic. Check `os.Getenv(\"OHC_MULTITENANT\") == \"true\"` to determine the driver.
3. For Standalone mode, implement a basic local in-memory driver (e.g., using a thread-safe map or a lightweight LRU cache).
4. For Cloud mode, implement a Redis-backed driver using `go-redis`. Ensure tenant isolation using `organization_id` by prefixing all keys.
5. Create tests in `cache_test.rs`. Test both the local in-memory driver and the Redis driver (use a mock Redis client or miniredis). Ensure 100% test coverage.
6. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P1

## Estimated Scope
Medium
