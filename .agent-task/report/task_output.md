issue_title: "[integrations] Hybrid Distributed Lock MCP"
issue_description: |
  # Title: [integrations] Hybrid Distributed Lock MCP

  ## Problem Statement
  OHC operates in both Cloud-native (multi-tenant) and Standalone (single-user) modes. When the swarm interacts with shared resources (e.g., modifying a shared configuration file, updating a centralized state, or processing a unique task), mutually exclusive access is required to prevent race conditions and data corruption. Cloud deployments require a distributed lock manager backed by Redis (e.g., Redlock) to synchronize locks across horizontal agent pods. In contrast, Standalone mode needs a zero-dependency local implementation (e.g., SQLite-backed locks or in-memory mutexes) to maintain the lightweight architecture. Currently, agents lack a unified MCP Tool for dynamic distributed locking across these environments.

  ## Research Report
  Most existing agentic frameworks configure locks statically or rely strictly on a centralized infrastructure like Redis. This breaks down in hybrid architectures where an agent might be executing on a user's local machine without access to a distributed cache. By introducing a Hybrid Distributed Lock MCP, OHC agents can request locks dynamically. The underlying implementation will route the request to Redis in Cloud mode, or to a local SQLite/in-memory lock manager in Standalone mode, delivering an "Unfair Advantage" for smooth local-to-cloud handoffs without code changes.

  ### Competitive Analysis

  | Feature | OHC Hybrid Lock MCP | Traditional Cloud Brokers (e.g., Redis) | Local-Only Lock Managers |
  | :--- | :--- | :--- | :--- |
  | **Cloud Scale** | Yes (Redis backed) | Yes | No |
  | **Local Zero-Dependency** | Yes (SQLite/In-Memory) | No | Yes |
  | **Dynamic Mode Switching** | Yes | No | No |
  | **Multi-Tenant Isolation** | Yes | Yes | N/A |

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Agent] -->|Acquire/Release Lock via MCP| B(Hybrid Lock Manager)
      B --> C{Is Cloud Mode?}
      C -->|Yes| D[Redis Distributed Lock]
      C -->|No| E[SQLite / In-Memory Lock]
  ```

  ## Design Doc
  **Architecture:**
  - Create a new package `src/server/integrations/distributed_lock/`.
  - Introduce a `LockManager` implementing the MCP Tool interface.
  - Dynamically select the backend driver based on `std::env::var("OHC_MULTITENANT") == Ok("true".to_string())`.
  - **Cloud Mode:** Utilize Redis (e.g., via `redis::AsyncCommands`) to implement a distributed locking algorithm (like Redlock, similar to `src/server/orchestration/queue/redis_lock.rs`).
  - **Standalone Mode:** Implement an in-memory or SQLite-backed locking mechanism.

  **API Contracts:**
  - `AcquireLock(ctx context.Context, resource string, ttl time.Duration) (string, error)` (Returns a lock token).
  - `ReleaseLock(ctx context.Context, resource string, token string) error`.

  **Security:**
  - Ensure `organization_id` (or `tenant_id`) prefixes are rigorously applied to resource keys in Cloud mode to enforce cross-tenant isolation and prevent one tenant from locking another tenant's resources (e.g. `ohc:lock:{tenant_id}:{resource_type}:{resource_id}`).

  ## Implementation Prompt
  "Implement the Hybrid Distributed Lock MCP tool in `src/server/integrations/distributed_lock/`.
  1. Create `lock.rs` defining the `LockManager` and its MCP capabilities (`AcquireLock`, `ReleaseLock`).
  2. Implement environment-agnostic logic. To determine if the connection is Cloud, check: `std::env::var("OHC_MULTITENANT") == Ok("true".to_string())`.
  3. For Cloud mode, implement a Redis-backed distributed lock ensuring `organization_id` is used as part of the lock key. Use the `redis` crate or implement a standard Redlock algorithm.
  4. For Standalone mode, implement a robust in-memory or SQLite-backed lock.
  5. Create comprehensive tests in `lock_test.rs`, mocking Redis and validating the Standalone local fallback. Ensure 100% test coverage.
  6. Create an E2E test verifying that two simulated agents cannot concurrently hold a lock for the same resource. Mock the AI models.
  7. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies, and add it to the workspace members in `Cargo.toml`."

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
