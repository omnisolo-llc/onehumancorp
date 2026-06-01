issue_title: "[Architecture] Edge-Cached Dynamic Storefronts"
issue_description: |
  # Universal Edge-Cached Dynamic Storefronts

  ## Problem Statement
  Small business owners like Priya (Boutique) and Maya (Baker) need blazing-fast storefronts that load instantly on low-end mobile devices and spotty networks. While platforms like Shopify and Wix offer edge caching, they struggle with dynamic inventory (e.g., Maya's custom cake availability or Priya's size/color stock syncing in real-time) without breaking the cache or causing latency. OHC currently lacks a globally distributed, edge-cached serving layer that seamlessly blends static assets with real-time dynamic inventory data for multi-tenant storefronts.

  ## Research Report
  - **Shopify:** Utilizes a globally distributed edge network (Cloudflare) but often relies on client-side JS to fetch dynamic pricing/inventory, leading to layout shifts and slower Time to Interactive (TTI) on low-end devices.
  - **Wix:** Employs static site generation (SSG) with ISR (Incremental Static Regeneration), which can be slow to update when a high-demand item sells out instantly.
  - **OHC Opportunity:** By leveraging our Go/Bazel backend and Redis, we can implement an architecture that pushes pre-rendered HTML to the edge (CDN) while using lightweight edge computing (e.g., Cloudflare Workers or a lightweight Go proxy) to inject real-time state (inventory, personalized pricing) directly into the stream before it reaches the client. This guarantees sub-100ms TTI globally with zero stale inventory.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client(Mobile Browser - 375px) --> EdgeProxy(Edge Proxy / CDN Worker)
      EdgeProxy -- Cache Hit --> StaticCache(Edge HTML Cache)
      EdgeProxy -- Dynamic Injection --> InventoryRedis(Edge Redis Read Replica)
      EdgeProxy -- Cache Miss / SSR --> OHC_Backend(OHC Go Backend / Postgres)
      OHC_Backend --> InventoryRedis
  ```

  ### UX/UI Impact (Mobile-First 375px)
  - **Instant Load:** The user sees the full UI structure and images instantly.
  - **No Layout Shift:** Dynamic data (e.g., "Sold Out" badges, current price) is injected at the edge, meaning the initial HTML payload is already accurate.
  - **Low-Data Mode:** Edge optimization automatically converts images to WebP and strips non-essential JS for 3G networks.

  ### AI Agent Integration
  - **Operations (The Manager):** Real-time inventory updates from the agent immediately invalidate the specific tenant's edge cache slice or update the Edge Redis replica.
  - **Marketing (The Promoter):** When the agent updates the storefront design, it triggers a background pipeline to regenerate and distribute the static assets to the edge network.

  ## Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront architecture. Focus on the core backend routing and caching logic that allows a storefront request to be served with pre-rendered HTML while dynamically injecting real-time inventory state.
  - Create the data models for Storefront configurations and Edge Cache invalidation rules, ensuring strict multi-tenant isolation (`tenant_id`).
  - Implement a caching middleware or service layer in Go that simulates edge-injection (e.g., retrieving base HTML from a cache and hydrating specific dynamic tags via Redis).
  - Write comprehensive unit tests and at least one Playwright E2E test verifying that a simulated mobile client (375px) receives the correctly hydrated storefront instantly.
  - Apply the OHC Premium Token visual style (Glassmorphism, Outfit font) to the resulting storefront render.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
