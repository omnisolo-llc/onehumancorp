issue_title: "Implement Edge-Caching Storefronts"
issue_description: |
  **Problem Statement**
  Small business owners (Maya, Carlos) require instant-loading, beautiful storefronts to capture high-intent leads from social media, particularly on poor 4G/5G connections. Currently, every page view on their dynamic storefronts requires a round-trip to the centralized OHC database to retrieve tenant configuration and product inventory. This centralized approach leads to high Time to First Byte (TTFB), decreasing conversion rates for global users and causing scalability challenges for OHC during traffic spikes. Small businesses need the performance of enterprise CDNs (like Shopify or Wix) completely invisibly, without managing cache invalidation or DNS.

  **Research Report**
  - Edge Caching Needs: To meet the Mobile-First Performance targets (LCP < 1.5s on 4G), the initial HTML document must be served from the edge (e.g., Cloudflare Workers or Fastly Compute) rather than a centralized origin.
  - Dynamic Content Challenge: OHC storefronts are highly dynamic. Maya's inventory count changes frequently. Traditional static caching would result in overselling.
  - Micro-caching and SWR (Stale-While-Revalidate): OHC can leverage SWR at the edge. The edge serves a slightly stale but ultra-fast HTML shell, while fetching the latest dynamic inventory in the background or via client-side hydration.
  - Multi-Tenant Routing: The Edge Worker must dynamically resolve the incoming host (e.g., `maya.ohc.app` or `maya-bakes.com`) to the correct OHC `tenant_id` at the edge to serve the right cached content.

  **Design Doc**
  1. Edge-Driven Resolution: The mapping of custom domains to tenant_id must occur at the edge to prevent centralized routing bottlenecks.
  2. Tag-Based Invalidation: Every cached response must be tagged with tenant-id:{id} and entity:{type}:{id}. When Maya updates a product, OHC Origin fires an API call to the Edge to purge only that specific tag, ensuring instant updates.
  3. Optimistic Inventory Hydration: The Edge serves the cached HTML with placeholder "buy" buttons. A lightweight client-side script fetches the real-time inventory count directly from the nearest Edge Worker KV store to enable/disable the button.

  **Implementation Prompt**
  Implement the Edge-Caching layer for OHC Storefronts. Create the StorefrontRouter Edge Worker script (e.g., Cloudflare Worker) that resolves incoming requests to the OHC tenant_id. Implement the caching strategy using Stale-While-Revalidate headers from the OHC Origin. In the OHC Backend ProductService, add an event listener that triggers a tag-based cache purge API call to the CDN whenever a product or tenant setting is modified. Ensure the merchant UI remains completely unaware of this caching layer.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
