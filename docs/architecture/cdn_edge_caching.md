# CDN Edge Caching Integration & Strategy

OneHumanCorp (OHC) employs an edge-caching layer to ensure sub-100ms storefront delivery globally, heavily relying on CDN features to merge static speeds with dynamic inventory updates.

## Architecture

1. **Edge Tier (e.g., Cloudflare, Fastly, or local NGINX proxy):**
   - Intercepts requests to the public storefront APIs (`/api/v1/storefront/`).
   - Serves pre-rendered, cached HTML shells enriched with SEO metadata directly from the edge.

2. **Core Tier (`ohc-core`):**
   - The application core handles cache generation upon cache misses.
   - Core listens to events such as `inventory.updated` and product mutations.
   - Upon detecting these mutations, `ohc-core` publishes specific cache invalidation events.

3. **Cache Invalidator Service:**
   - Subscribes to the `cache_invalidation_events` Redis pub/sub channel.
   - Extracts relevant cache tags (e.g., `tenant-id:<id>` or `entity:product:<id>`).
   - Clears the internal `HybridCache` and hits the local CDN Cache.
   - *In Production:* Triggers CDN-specific purge APIs (like Cloudflare's Purge by Cache-Tag API or Fastly's Surrogate-Key purge) asynchronously.

## Cache Tags & Surrogate Keys

Every pre-rendered storefront product HTML emitted by `ohc-core` includes tags to map the response to the underlying tenant and product.
- **Header format:** `Cache-Tag` or `Surrogate-Key`.
- **Examples:** `tenant-id:33333333-3333-3333-3333-333333333333`, `entity:product:44444444-4444-4444-4444-444444444444`.

When product inventory drops below a threshold or its details change, the cache invalidation event purges *only* the specific item via its `entity:product:<id>` tag, ensuring that high-traffic stores remain available from the edge.

## Dynamic Hydration

To prevent caching personalized or highly dynamic data (such as an individual user's shopping cart count), the pre-rendered shell served by the edge cache delegates dynamic state to client-side hydration.

- The HTML output includes placeholders (`<div id="cart-badge"></div>`).
- A lightweight script runs client-side post-load to pull state from `localStorage` or execute direct (cache-bypassing) API fetches to populate these placeholders immediately.

## Testing Locally

For local development and testing, the `deploy/docker-compose.yml` stack includes an `edge-cache` service utilizing NGINX.
- It acts as a rudimentary CDN, proxying requests to `ohc-core` and caching responses for `/api/v1/storefront/`.
