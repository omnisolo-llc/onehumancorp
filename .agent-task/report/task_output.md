issue_title: "[Platform] Implement Multi-Tenant Local Edge Caching for Dynamic Storefronts"
issue_description: |
  # Research Report: Multi-Tenant Local Edge Caching for Dynamic Storefronts

  ## 1. Problem Statement
  Small business owners running e-commerce or booking sites on OHC rely heavily on fast page load times to prevent bounce rates and lost revenue. However, generating personalized, dynamic, and multi-tenant storefront pages for every single request creates high latency and database load. Competing builders (like Shopify and Wix) use globally distributed CDNs and complex cache invalidation, which are expensive and technically challenging to emulate on a single-server deployment. OHC needs a robust, local, multi-tenant edge-caching layer that delivers sub-10ms page loads without compromising dynamic capabilities or data freshness.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify achieve fast response times by aggressively caching catalog pages at the CDN edge and fetching dynamic data (cart state, inventory) via client-side JavaScript. This architecture is complex to maintain.
  - **The OHC Opportunity**: By implementing an intelligent, application-aware HTTP caching layer (or Reverse Proxy cache) integrated directly into the OHC monolithic deployment, we can achieve edge-like performance with significantly lower architectural complexity.
  - **Competitor Gaps**:
    - *Shopify*: Excellent performance but relies on an expensive proprietary CDN infrastructure (Fastly/Cloudflare).
    - *Wix*: Caches heavily but often struggles with the "Time to First Byte" (TTFB) for dynamic, personalized user sessions.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Browser/Mobile] -->|HTTP Request| Nginx[NGINX Reverse Proxy Cache]
      Nginx -- Cache Hit --> Client
      Nginx -- Cache Miss --> Server[OHC Go Server]
      Server --> DB[(PostgreSQL)]
      Server --> Valkey[(Valkey/Redis Cache)]
      Server -->|HTTP Response + Cache Tags| Nginx

      CacheInvalidator[Operations Agent / Cache Service] -->|Purge Request| Nginx
  ```

  ### Key Design Decisions
  - **Cache Layer**: Utilize the existing NGINX container in the `docker-compose.yml` (`edge-cache` service) as the primary HTTP reverse proxy cache.
  - **Cache Keys**: The cache key MUST incorporate the `tenant_id` (derived from the host or path) to ensure strict multi-tenant isolation. No cross-tenant data leakage is permitted.
  - **Invalidation Strategy**: Implement surrogate keys (Cache-Tags) in the OHC server response headers. When a product, service, or configuration is updated in the database, the OHC server must trigger a targeted purge request to NGINX for the associated cache tags (e.g., `tenant:123`, `product:456`).
  - **Dynamic Content**: Highly dynamic content (e.g., shopping cart, live inventory lock status) should be excluded from the NGINX cache or loaded asynchronously via API endpoints that bypass the cache.

  ### Mobile UX Impact
  - Near-instantaneous page loads on 375px mobile devices, even on slow 3G networks, directly addressing the needs of personas like Fatima (food cart operator with slow mobile data).

  ## 4. Implementation Prompt
  **Feature Name**: OHC Multi-Tenant Edge Caching System

  **Outcome**: Sub-10ms TTFB for public storefront pages across all tenants, with automated, targeted cache invalidation when business owners update their catalog or settings.

  **Next Actions**:
  1.  **Configure NGINX**: Update the `deploy/docker/nginx/nginx.conf` to enable proxy caching. Define the cache zone, configure cache keys to include tenant identifiers, and set up a mechanism for targeted cache purging (e.g., using a Lua script or an open-source purge module).
  2.  **Implement Cache Headers**: Modify the OHC Go server (specifically the storefront rendering routes) to emit appropriate `Cache-Control` and custom surrogate key headers (e.g., `X-Cache-Tags`).
  3.  **Implement Cache Invalidation**: Create a service within the OHC server that listens for database mutation events (e.g., product updates, setting changes) and sends HTTP PURGE requests to the NGINX edge cache for the affected tags.
  4.  **Bypass Dynamic Routes**: Ensure API routes handling carts, checkout, and live inventory (Redis locks) explicitly bypass the NGINX cache.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
