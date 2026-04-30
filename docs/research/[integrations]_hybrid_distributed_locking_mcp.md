# Teammate Mesh Distributed Locking (Hybrid)

Our mission is to support distributed locking across Cloud (Redis) and Standalone (Local file/SQLite advisory locks).

The `DistributedLock` currently lives in `src/server/agents/legacy_mesh.rs` and hardcodes Redis usage. We need to create a `MeshLock` trait that abstracts this, and provide:
- `RedisLock` (Cloud mode)
- `LocalLock` (Standalone mode - can be based on local advisory locks, an in-memory structure with `tokio::sync::Mutex`, or file-based).

Let's check the rest of the codebase to see if there are other lock implementations or where `legacy_mesh.rs` is used.
