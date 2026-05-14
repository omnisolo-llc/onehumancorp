# Architecture Deep Dive: Multi-Tier Hybrid Caching

## Overview
Caching in OHC is not just a performance optimization; it is a fundamental requirement for the "Standalone Sovereignty" model. This document details how we handle data consistency and availability across Cloud and Standalone environments.

## The HybridCache Abstraction
We use a unified `HybridCache<T>` struct that provides a consistent API for both environments:
- **Standalone Mode**: The cache operates strictly in-memory using a thread-safe `RwLock<HashMap>`. It uses a Least-Recently-Used (LRU) eviction policy based on a configurable capacity.
- **Cloud Mode**: The cache uses local memory as L1 and Redis as L2. This provides sub-microsecond access for the hottest data while allowing multi-node consistency via Redis.

## Data Categorization & TTL Policies

### 1. Rare Changers (The "Ice" Layer)
- **Examples**: Domain definitions, Marketplace items, Legal policy templates.
- **TTL**: 1 Hour.
- **Strategy**: Aggressive L1/L2 caching. Invalidation occurs only on platform releases or manual refresh.

### 2. Slow Changers (The "Water" Layer)
- **Examples**: Business profiles, Agent definitions, Product catalog.
- **TTL**: 10 - 60 Minutes.
- **Strategy**: Cache with event-driven invalidation. When a user updates their profile, a mesh event triggers a cache purge.

### 3. Fast Changers (The "Steam" Layer)
- **Examples**: Tenant Analytics, Resource usage, Task counts.
- **TTL**: 15 - 60 Seconds.
- **Strategy**: Very short TTL to prevent "refresh spam" from causing database load.

## Cache Invalidation via Teammate Mesh
To maintain consistency in Cloud mode without sacrificing speed, we utilize the Teammate Mesh (NATS/Redis PubSub) for invalidation.

```rust
// On update:
db.update_agent(agent).await?;
mesh.publish("cache:invalidate:agent", agent_id).await?;

// On mesh event:
fn handle_invalidation(agent_id: String) {
    AGENT_CACHE.invalidate(agent_id);
}
```

## Edge Cases and Failure Modes
- **Redis Down**: `HybridCache` automatically fails back to local-only operation. Metrics are emitted to alert operators of the degraded state.
- **Memory Pressure**: The L1 layer monitors process memory and will shrink the LRU capacity if the system reaches 90% of allocated RAM.

## Implementation Best Practices

1. **Avoid Over-Caching**: Do not cache data that is unique per request (e.g., search results with many filters) unless the exact same parameters are seen frequently.
2. **Serde Overhead**: Be mindful of the serialization cost when using Redis. For small objects, the cost of JSON encoding/decoding can sometimes exceed the DB query time. In these cases, prefer local memory only.
3. **Key Namespacing**: Always prefix cache keys with the service name and organization ID to prevent cross-tenant leakage. Example: `org_service:domains:global` or `dashboard:orders:tenant_123`.
4. **Cache Stampede Prevention**: Use a "single-flight" pattern where only one worker fetches from the DB if a cache miss occurs for a popular key, while others wait for the result.
