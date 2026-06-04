issue_title: "Architecture Gap: Edge-Caching Dynamic Storefronts for High-Traffic Viral Events"
issue_description: |
  # Problem Statement
  Currently, OneHumanCorp (OHC) provides non-technical users with the ability to launch storefronts quickly. However, when an OHC merchant experiences a viral event (e.g., a TikTok video goes viral for Maya the Home Baker or Priya the Boutique Owner), the sudden influx of concurrent requests directly hits the backend API and PostgreSQL database. This lack of edge caching for dynamic storefronts leads to high latency, degraded mobile performance (especially on 375px viewports and slow connections), and potential downtime. A robust edge-caching layer is required to ensure that read-heavy product catalogs and storefront pages are served with near-zero latency globally, while still allowing dynamic elements (like "sold out" toggles or inventory counts) to be updated accurately.

  # Research Report
  - **Competitor Analysis:**
    - **Shopify:** Utilizes a global CDN with Fastly, extensively caching storefront pages at the edge. They implement "stale-while-revalidate" and tag-based cache invalidation to handle inventory changes rapidly without overwhelming origin servers.
    - **Wix / Squarespace:** Both employ aggressive edge caching for static assets and HTML, with targeted invalidation when a user publishes changes.
    - **Vercel / Next.js:** Popularized the pattern of Incremental Static Regeneration (ISR) and edge middleware to personalize cached content without hitting the database.
  - **Findings:** OHC's current architecture relies on direct API queries for storefront rendering. This is inefficient for public, read-heavy data. We need to introduce an edge caching tier (e.g., Cloudflare Workers or AWS CloudFront with Lambda@Edge) that caches storefront data (products, prices, images) per `tenant_id`.
  - **Key Challenge:** Balancing high cache hit rates with the need for immediate consistency on critical data (e.g., Fatima needs her "sold out" toggle to reflect instantly so she doesn't get overbooked).

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer as Mobile User (375px)
      participant Edge as Edge CDN / Cache
      participant API as OHC Core API
      participant DB as PostgreSQL (Multi-Tenant)
      participant Worker as AI Operations Agent

      Customer->>Edge: GET /storefront/{tenant_id}/products
      alt Cache Hit
          Edge-->>Customer: Return Cached Storefront Data (Low Latency)
      else Cache Miss or Stale
          Edge->>API: Fetch Storefront Data
          API->>DB: Query Products (RLS applied)
          DB-->>API: Return Data
          API-->>Edge: Return Data & Set Cache-Control (Tags)
          Edge-->>Customer: Return Data
      end

      Note over API,DB: Inventory Update Event
      Worker->>API: Toggle Item "Sold Out"
      API->>DB: Update Inventory Status
      API->>Edge: Invalidate Cache Tag {tenant_id}-products
  ```

  ## Mobile UX Flow (375px)
  1. **User Experience:** The storefront loads instantly on a 375px screen, even on a 3G network, because the HTML/JSON payload is served from a local edge node.
  2. **Optimistic UI:** When the business owner updates inventory (e.g., toggles "sold out"), the UI updates optimistically. Behind the scenes, the API invalidates the edge cache for that specific tenant.
  3. **Visual Excellence:** The loading state (if any) uses smooth, translucent glass skeletons that mirror the final UniFi-style card layouts, ensuring no layout shift.

  ## AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Monitors inventory levels. When stock reaches zero, it automatically triggers a cache invalidation event for the storefront to prevent overselling.
  - **Marketing Agent ("The Promoter"):** When generating new promotional content or redesigning the site, it flushes the edge cache so new customers see the updated design immediately.

  ## Key Design Decisions
  - **Tag-Based Invalidation:** Cache entries will be tagged with `{tenant_id}` and resource types (e.g., `tenant:123:products`). This allows OHC to purge exactly what changed without clearing the entire global cache.
  - **Stale-While-Revalidate:** For non-critical data (like product descriptions or reviews), the edge will serve stale content while asynchronously fetching fresh data from the origin.
  - **Zero Trust & Multi-Tenancy:** The edge layer must enforce strict path isolation (`/tenant/{tenant_id}/...`) so one tenant's cache cannot leak into another's.

  # Implementation Prompt
  Implement the Edge-Caching Dynamic Storefront layer.
  1. Introduce an edge caching middleware (or CDN configuration) that intercepts requests to public storefront API endpoints.
  2. Cache the responses at the edge with appropriate `Cache-Control` and `Surrogate-Key` (or Cache Tags) headers.
  3. Implement a cache invalidation service in the OHC Backend that is triggered by state changes (e.g., inventory updates, layout publishes) and purges the specific tenant's tags at the edge.
  4. Ensure that the AI Operations Agent automatically triggers this invalidation when it detects stock-outs.
  5. **Acceptance Criteria:** Storefront endpoints must return `HIT` from the edge cache on subsequent requests. An update to a product via the owner's dashboard must reflect on the public storefront within 2 seconds. The solution must handle 10,000 concurrent requests to a single storefront without increasing origin database load.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []