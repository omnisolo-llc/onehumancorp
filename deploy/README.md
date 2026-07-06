# CDN Integration Pattern

OneHumanCorp (OHC) utilizes a Universal Edge-Cached Dynamic Storefront architecture. To achieve sub-100ms storefront performance globally, we integrate with Edge CDNs (such as Cloudflare Workers or Fastly) and local reverse proxies (such as Varnish or NGINX).

## Architecture

1. **Edge Cache Layer:** A CDN/Reverse Proxy sits in front of the `ohc-core` application. It aggressively caches the storefront pre-rendered HTML for fast Time to First Byte (TTFB).
2. **Cache Tagging:** The `ohc-core` backend responds with specific cache headers (`Cache-Tag`, `Surrogate-Key`, and `ETag`). The edge layer uses these to associate content with specific tenants or products.
3. **Invalidation:** When a product, tenant, or inventory mutation occurs in `ohc-core`, a Redis Pub/Sub message (`cache_invalidation_events`) is published. A dedicated `Cache Invalidator` service listens to these events and purges the specific tags from both the localized CDN cache and the external Edge CDN using API calls.

## Deployment Instructions

### Local/Docker Compose (Varnish / Nginx)
For local development, we use an in-memory Hybrid CDN cache middleware located in `src/server/utils/edge_caching_middleware.rs`. No external CDN configuration is strictly required, though you may deploy a Varnish container configured to honor `Surrogate-Key` purging.

### Production (Cloudflare / Fastly)
When deploying to production:
1. Configure your CDN to respect `Cache-Control` and `Surrogate-Key` headers returned by the `/api/v1/storefront/*` endpoints.
2. The caching rules should bypass cache for specific dynamic hydration paths or POST requests.
3. Configure the `REDIS_URL` in the `ohc-core` container to allow the Cache Invalidator service to connect to the central message bus.

For more details on the cache invalidator logic, see `src/server/services/cache_invalidator.rs`.
