issue_title: "Research: High-Scale Edge Caching for Dynamic Storefronts"
issue_description: |
  As OneHumanCorp scales, the loading speed of individual tenant storefronts is critical. Small business owners like Maya (The Home Baker) rely heavily on social media (Instagram, TikTok) links. When a post goes viral, the influx of traffic can overwhelm origin servers if not properly cached at the edge. A slow storefront increases bounce rates and lost revenue. We must implement a comprehensive Edge-Cached Dynamic Storefront architecture to ensure <200ms TTFB (Time to First Byte) globally, even for dynamic content.

  ## Market Gap & Competitor Analysis
  - **Shopify:** Utilizes a globally distributed edge network (Fastly/Cloudflare) but charges premium rates for advanced edge caching configurations. It handles traffic spikes well but is highly centralized.
  - **Wix:** Caches static assets aggressively but dynamic product variants often require roundtrips to origin, slowing down the perceived performance on mobile.
  - **Squarespace:** Good static caching, but lacks robust edge-side composition for highly personalized AI-driven storefronts.
  - **OHC Opportunity:** By leveraging Edge Workers (Cloudflare Workers/Fastly Compute@Edge) and a distributed Redis cache, OHC can compose personalized, dynamic storefronts *at the edge*, bypassing the Go backend entirely for 95% of read traffic.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Customer Mobile Browser] -->|HTTPS| Edge[Edge Node / CDN]
      Edge -->|Cache Hit| Response[Cached HTML/JSON]
      Edge -->|Cache Miss| EdgeWorker[Edge Compute]
      EdgeWorker -->|Fetch| GlobalRedis[Global Redis / KV]
      EdgeWorker -->|Fallback| Origin[OHC Go Backend / DB]
      Origin -->|Updates| GlobalRedis
      Origin -->|Invalidation| Edge
  ```

  ### Core Principles
  1. **Stale-While-Revalidate (SWR):** Edge nodes serve stale content instantly while revalidating in the background.
  2. **Surrogate Keys / Cache Tags:** Every product, variant, and tenant setting is tagged. When a merchant updates a price, only that specific tag is invalidated globally.
  3. **Edge-Side Includes (ESI) / Edge Composition:** The core shell of the storefront is cached. User-specific state (like cart count) is fetched asynchronously or composed at the edge using lightweight tokens.

  ### Implementation Strategy
  - **Phase 1: Aggressive Static Caching & Cache Tags.** Implement `Cache-Control` headers and cache tags in the Go backend.
  - **Phase 2: Edge KV Storage.** Sync tenant catalog data to a global low-latency KV store (e.g., Cloudflare KV or Redis Enterprise Active-Active).
  - **Phase 3: Edge Rendering.** Move the rendering of the storefront HTML from the origin server to Edge Workers.

  ## Implementation Prompt
  **Task for Implementer Agent:**
  Implement the backend core structure, db models, and caching mechanism. Focus on Phase 1 logic. Add surrogate tags logic and integration with Redis or caching logic. Create Edge Worker shell and test caching. Add end-to-end tests for Maya's edge caching edge case to see if page loads faster.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
