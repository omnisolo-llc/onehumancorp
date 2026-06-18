issue_title: "Implement Edge-Cached Dynamic Storefront with Agentic SEO Pre-rendering"
issue_description: |
  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases. This leads to high latency, timeouts, lost revenue, and poor search engine visibility due to dynamic rendering limitations. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO. Existing platforms like Shopify offer strong edge networks but require third-party apps for advanced SEO, while Vercel/Next.js are inaccessible to non-technical users.

  ## Research Report
  - **Context:** Based on our market research (e.g., `docs/business/market_research/[research]_universal_edge_cached_dynamic_storefront_seo.md`), there's a critical gap between enterprise e-commerce performance and what's accessible to SMBs.
  - **Pain Points:** High latency during spikes, lost conversions, SEO penalties from slow client-side rendering, and the complexity barrier of configuring edge caching.
  - **Competitor Analysis:** Shopify uses Cloudflare but relies heavily on apps for SEO. Wix/Squarespace have easier SEO tools but lack instant edge scalability during unpredictable spikes.
  - **OHC Differentiator:** Universal Edge Caching combined with Agentic SEO Pre-rendering. This must be entirely invisible and autonomous. The platform should automatically offload reads to an edge cache and pre-render SEO-optimized HTML when an agent updates the website.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B[Edge CDN/Cache Layer]
      B -- Cache Hit --> A
      B -- Cache Miss --> C[OHC Application Server]
      C --> D[(PostgreSQL / ValKey)]
      E[Operations Agent] -- Inventory Update --> F[Cache Invalidation Service]
      F --> B
      G[Marketing Agent] -- Content Update --> H[SEO Pre-rendering Service]
      H -- Pushes Static HTML --> B
  ```

  ### Mobile UX Flow (375px First)
  - The feature is completely invisible to the end user (the business owner).
  - There are no settings or configuration screens to navigate.
  - Maya updates her product catalog (e.g., adds a new cake) via the existing mobile app flow.
  - A subtle notification or "Agent Activity" log may show: "Marketing Agent optimized your new product page for search engines and deployed it globally."
  - The storefront loads instantly for her customers on their mobile devices, regardless of traffic.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory and catalog state changes. Triggers precise edge cache invalidations when items sell out or details change.
  - **Marketing Agent:** Monitors content updates. Autonomously generates meta titles, descriptions, and structured data, then triggers the SEO Pre-rendering Service to push static HTML to the edge.

  ### Key Design Decisions
  - **Zero Configuration:** The user must never see words like "CDN", "Cache", "TTL", or "SSR".
  - **Event-Driven:** Cache invalidation and pre-rendering must be tightly coupled to business events (inventory updates, content generation).
  - **Multi-Tenant Safety:** Edge caching must strictly adhere to tenant isolation rules. Cache keys must incorporate `tenant_id`.

  ## Implementation Prompt
  **User Facing Outcome:** The user's public storefront is blazing fast globally, highly resilient to traffic spikes, and perfectly indexed by search engines, all without any configuration on their part.

  **CUJ (Critical User Journey):**
  1. User adds a new product or updates existing content via the OHC platform.
  2. The system saves the change to the central database.
  3. The Operations/Marketing Agents autonomously trigger cache invalidation and SEO pre-rendering for the affected pages.
  4. A customer visits the updated page and receives an instantly loading, fully SEO-optimized HTML response from the nearest edge node.

  **Acceptance Criteria:**
  - Implement a caching middleware or layer that intercepts storefront read requests and serves from a global edge cache (e.g., Redis/Valkey representation locally).
  - Implement an event listener that invalidates specific cache keys (scoped by `tenant_id`) upon product/content updates.
  - Implement a pre-rendering service that generates static HTML with SEO meta tags for storefront pages and pushes it to the cache.
  - Ensure 100% unit test coverage for the caching logic, invalidation, and pre-rendering triggers.
  - Provide a Playwright E2E test that simulates a product update, verifies the cache is invalidated, and confirms the new SEO-optimized page is served correctly.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
