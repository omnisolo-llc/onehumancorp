issue_title: "[Architecture] Edge-Caching & Dynamic Storefront CDN Implementation"
issue_description: |
  # Architecture Brief: Edge-Caching & Dynamic Storefront CDN

  ## Problem Statement
  Small business owners (Maya, Carlos) require instant-loading, beautiful storefronts to capture high-intent leads from social media, particularly on poor 4G/5G connections. Currently, every page view on their dynamic storefronts requires a round-trip to the centralized OHC database to retrieve tenant configuration and product inventory. This centralized approach leads to high Time to First Byte (TTFB), decreasing conversion rates for global users and causing scalability challenges for OHC during traffic spikes. Small businesses need the performance of enterprise CDNs completely invisibly, without managing cache invalidation or DNS.

  ## Research Report
  - **Edge Caching Needs**: To meet Mobile-First Performance targets (LCP < 1.5s on 4G), the initial HTML document must be served from the edge rather than a centralized origin.
  - **Dynamic Content Challenge**: Traditional static caching would result in overselling due to frequent inventory changes.
  - **Micro-caching and SWR (Stale-While-Revalidate)**: OHC can leverage SWR at the edge. The edge serves a slightly stale but ultra-fast HTML shell, while fetching the latest dynamic inventory in the background or via client-side hydration.
  - **Multi-Tenant Routing**: The Edge Worker must dynamically resolve the incoming host to the correct OHC `tenant_id` at the edge to serve the right cached content.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ STOREFRONT : "configures"
      TENANT ||--o{ PRODUCT : "sells"
      STOREFRONT ||--|{ EDGE_CACHE_TAG : "tagged with"
      PRODUCT ||--|{ EDGE_CACHE_TAG : "tagged with"

      EDGE_NETWORK {
          string worker_id
          string edge_kv_store
          string cache_status
      }

      EDGE_CACHE_TAG {
          string tag_id "e.g., tenant-id:123"
          string entity_type "e.g., product, config"
      }

      TENANT {
          uuid id
          string custom_domain
          string cache_status
      }

      EDGE_NETWORK ||--o{ EDGE_CACHE_TAG : "invalidates based on"
  ```

  ### Key Architectural Invariants
  1. **Edge-Driven Resolution**: Mapping of custom domains to `tenant_id` must occur at the edge.
  2. **Tag-Based Invalidation**: Cached responses must be tagged with `tenant-id:{id}` and `entity:{type}:{id}`.
  3. **Optimistic Inventory Hydration**: Edge serves cached HTML with placeholder buttons. Client-side script fetches real-time inventory count.

  ### UI Wireframes & Screen Flow (375px First)
  - **Customer Storefront View**: Storefront loads in <500ms. UI displays a shimmer skeleton while client-side hydration securely fetches live "Buy" button state and inventory count.
  - **Merchant Dashboard View**: Merchant UI hides all CDN terminology. Saving a product shows "Storefront Updated instantly".

  ### Mobile UX Flow
  - Product images automatically optimized at edge (WebP/AVIF format, resized to 375px width).
  - Edge-cached PWA ensures catalog remains visible offline.

  ### AI Agent Integration Points
  - **Operations Manager Agent**: Triggers cache purge events via backend event mesh.
  - **Ambassador Agent**: Queries edge cache status via `StorefrontRouter` metrics to verify localized caching issues.

  ## Implementation Prompt
  Implement the Edge-Caching layer for OHC Storefronts. Create the `StorefrontRouter` Edge Worker script that resolves incoming requests to the OHC `tenant_id`. Implement the caching strategy using `Stale-While-Revalidate` headers from the OHC Origin. In the OHC Backend `ProductService`, add an event listener that triggers a tag-based cache purge API call to the CDN whenever a product or tenant setting is modified. Ensure the merchant UI remains completely unaware of this caching layer.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
