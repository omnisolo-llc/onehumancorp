# Architecture Blueprint: Bolt Latency Reduction

## Introduction
The Bolt Architecture is the performance standard for OHC Hybrid Agentic OS. This blueprint defines the mandatory patterns and constraints required to maintain sub-second response times.

## Core Pillars

### 1. Concurrency First
- **Sequential DB Queries are forbidden** in request handlers. All independent I/O must be wrapped in `tokio::join!` or `futures::future::join_all`.
- **Background Workers** must implement batched processing to handle throughput spikes without linear latency growth.

### 2. Intelligent Caching (HybridCache)
- Every gRPC service that reads from the database must evaluate caching suitability.
- Static metadata (Domains, Marketplace, Role Definitions) must have a minimum TTL of 1 hour.
- Tenant analytics must be cached for at least 60 seconds to protect the HUB during UI refresh loops.

### 3. Mobile Payload Projection
- API responses must support a `mobile_optimized` mode.
- Mobile projection must target a minimum **30% payload reduction** compared to desktop defaults.
- Heavily nested collections must be paginated or lazy-loaded on mobile.

### 4. Prompt Engineering as Performance Engineering
- System prompts are not just documentation; they are I/O overhead.
- All system prompts must pass through the `minify_system_prompt` pipeline.
- Avoid descriptive filler in agent names and roles; use concise, functional tokens.

## Implementation Guide

### Adding a new Cached Service
```rust
let cache = MY_SERVICE_CACHE.get_or_init(|| HybridCache::new(hub.redis_client.clone()));
if let Some(data) = cache.get(key).await {
    return Ok(data);
}
// Fetch from DB...
cache.set(key, data, Duration::from_secs(3600)).await;
```

### Implementing Mobile Optimization
```rust
let mut res = fetch_data().await;
if request.mobile_optimized {
    res.large_field = String::new();
    res.nested_list = vec![];
}
return res;
```

## Performance SLOs
| Metric | New York (5G) | Rural Mexico (3G) |
|--------|---------------|-------------------|
| Dashboard Load | < 200ms | < 800ms |
| AI Dispatch | < 50ms | < 200ms |
| Interaction Feedback | < 100ms | < 100ms |
