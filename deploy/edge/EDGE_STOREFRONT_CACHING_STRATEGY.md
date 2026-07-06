# Edge-Cached Dynamic Multi-Tenant Storefront Serving Architecture

## Overview
Small business storefronts built with OHC must load instantly on mobile networks (3G) and rank highly on search engines. Traditional dynamic SSR is too slow, and pure SPA suffers in SEO. OHC uses a globally distributed, edge-cached serving architecture via Cloudflare Workers.

## Edge Worker Routing & Resolution
DNS and initial HTTP requests hit the edge network (Cloudflare). The Cloudflare Worker script intercepts these requests.
1. The script inspects the incoming `Host` header (the custom domain).
2. It maps the custom domain to the internal `tenant_id` by looking up the mapping in the Edge KV store (`env.OHC_DOMAINS`).
3. If not found in KV (cache miss), it dynamically hits the backend resolution API (`/api/v1/storefront/resolve?domain=...`) and populates the Edge KV cache with `expirationTtl: 3600`.
4. It rewrites the request path and proxies the request to the correct SSR rendering service at the backend (e.g., `/api/v1/storefront/{tenant_id}/{product_id}`).

## Caching Strategy
- **Stale-While-Revalidate**: The backend injects headers such as `Cache-Control: public, s-maxage=60, stale-while-revalidate=86400`. This allows the CDN to serve stale content to users immediately while asynchronously fetching the fresh HTML.
- **Cache Tags / Surrogate Keys**: The backend injects `Surrogate-Key` (or `Cache-Tag`) headers representing the tenant and the resources presented (e.g., `tenant-id:1234`, `entity:product:5678`).
- **Asset Compression**: Image assets referenced in the HTML are automatically compressed to WebP and CDN-fronted.

## Invalidation Strategy
When the "Marketing Agent" or owner updates a product, changes a price, or modifies content, the backend Operations service triggers an invalidation payload to the cache invalidation service.
The Cache Invalidation Service invokes a `PURGE` request to the Edge CDN with the specific `Surrogate-Key` tags involved in the update. This guarantees that only the affected pages are refreshed at the edge while others remain fast.

## Testing Plan
- **Domain Resolution**: The E2E tests validate that the `/api/v1/storefront/resolve` endpoint successfully converts custom domains into tenant IDs and returns 404 for unknown domains.
- **Cache Hits/Misses**: The E2E tests mock sequential hits to ensure that the initial fetch registers as a cache `MISS` and populates the cache, while the subsequent hit registers as a cache `HIT`.
- **Invalidation Triggers**: E2E tests send webhook payloads with specific entity tags to validate that the backend purges the local and CDN caches effectively.
