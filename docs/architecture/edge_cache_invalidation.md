# Edge-Caching & Dynamic Storefront CDN Pattern

## Problem Context
Small business storefronts require high-performance loading times, particularly on mobile connections (4G/5G). Traditional dynamic pages require full database queries for each load, resulting in high Time to First Byte (TTFB).

## Caching Strategy
OHC implements an Edge Worker pattern utilizing `Stale-While-Revalidate` (SWR) headers paired with tag-based cache invalidation. This guarantees instant edge-loads while allowing dynamic content adjustments through aggressive cache purges.

## `StorefrontRouter` Component
The `StorefrontRouter` (implemented in `src/server/builder/edge.rs`) intercepts incoming tenant storefront requests. It utilizes the `HybridCache` to serve the initial HTML shell at edge speed, resolving mappings securely.

## Cache Invalidation Triggers
To avoid staleness, events that affect the storefront view must purge cache tags:
- `ProductCreated` / `ProductUpdated`: Clears `entity:product:{id}` and `tenant-id:{tenant_id}`. Handled inside operations and sync mesh tasks (`src/server/workers/department_workers.rs`, `offline_sync.rs`).
- `BlockUpdated` / `BlockCreated`: Rebuilding the storefront design immediately purges the `tenant-id:{tenant_id}` tag to flush layout and copy edits instantly (`src/server/builder/api.rs`).

## Observability
All edge purges utilize tagging to precisely invalidate content without impacting shared caching segments. Invalidation errors log to the general OHC operational trace.
