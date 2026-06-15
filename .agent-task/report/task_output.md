issue_title: "Agentic SEO Pre-rendering & Universal Edge-Caching for Storefronts"
issue_description: |
  ## Mission Queue Protocol Report: Agentic SEO Pre-rendering & Universal Edge-Caching

  **Priority:** P1
  **Estimated Scope:** Large

  ### Target Persona
  **Priya (Boutique Owner) & Maya (Home Baker)**
  They experience unpredictable traffic spikes from social media virality. When a TikTok or Instagram reel goes viral, their storefronts must load instantly globally. Furthermore, they do not understand SEO concepts like structured data, meta tags, or OpenGraph, yet they desperately need high organic discoverability to grow their business.

  ### Problem Statement
  Currently, OHC storefronts rely on dynamic data fetched from a centralized multi-tenant PostgreSQL database. While this ensures real-time accuracy (e.g., inventory tracking), it creates a bottleneck during high traffic, causing slow load times (Time to First Byte > 500ms for distant global users) and potential outages. Additionally, search engines struggle to index dynamic, client-side rendered content effectively. Small business owners cannot configure complex CDNs, Server-Side Rendering (SSR), or manage SEO metadata themselves.

  ### Research Report
  - **Market Landscape:** E-commerce giants like Shopify use global edge networks (Cloudflare) to cache storefronts, but dynamic personalization often requires heavy apps or falls back to origin. Developer platforms like Vercel/Next.js offer Incremental Static Regeneration (ISR) and Edge Functions, but these require coding expertise. Simpler builders like Wix provide basic caching but struggle with complex dynamic features at scale.
  - **OHC Opportunity:** OHC must close the gap by providing "invisible" enterprise-grade performance. Storefronts must load instantly (< 50ms) globally while maintaining 100% dynamic capabilities (e.g., instant "Sold Out" states).
  - **The "Promoter" Agent's Role:** The Marketing Agent ("The Promoter") must autonomously manage SEO, pre-rendering optimized static HTML injected with meta tags and structured data, pushing it directly to the edge without user intervention.

  ### Design Doc
  **Architecture Overview:**
  1. **Agentic SEO Pre-rendering (The Promoter):**
     - "The Promoter" continuously monitors tenant catalogs and business details.
     - It autonomously generates highly optimized static HTML shells containing JSON-LD schema, OpenGraph tags, and relevant keywords.
     - These shells are pushed to an Edge CDN.
  2. **Stale-While-Revalidate (SWR) & Edge KV:**
     - The Edge CDN serves the static HTML instantly.
     - Dynamic data (e.g., `inventory_count`, `price`) is stored in a highly distributed Edge Key-Value (KV) store.
     - An Edge Worker (e.g., Cloudflare Worker) intercepts the HTML response and injects the real-time dynamic data *before* returning it to the user.
  3. **Real-time Cache Invalidation (Operations Agent):**
     - When inventory is updated (e.g., an item is bought online or via POS), the Operations Agent mutates the central PostgreSQL database.
     - This mutation immediately triggers a cache invalidation event via a message queue (e.g., Redis Pub/Sub), updating the Edge KV and purging relevant HTML shells globally within milliseconds.

  **Data Model & State:**
  - **Central Origin:** PostgreSQL (`products` table, `inventory_count`).
  - **Edge State:** Edge KV Store (`tenant:{id}:product:{product_id}:inventory`).
  - **Message Bus:** Redis Pub/Sub for publishing invalidation events from the Go Backend to the Edge.

  **Mobile UX Flow (375px):**
  - **Zero Cumulative Layout Shift (CLS):** The pre-rendered HTML ensures structural stability.
  - **Instant Load:** The storefront appears instantly even on slow connections (e.g., 3G).
  - **Progressive Hydration:** Interactive elements hydrate selectively after the initial paint.

  ### Implementation Prompt
  **For the Implementer Agent:**
  Your goal is to build the core backend infrastructure for the Edge-Cached Dynamic Storefront.

  1. **Implement "The Promoter" SEO Generator:** Create a worker or service in the Rust/Go backend that takes a tenant's product data and generates structured SEO metadata (JSON-LD, OpenGraph).
  2. **Edge KV Sync Logic:** Implement the event-driven mechanism where inventory updates (e.g., `ProductCreated`, `InventoryUpdated`) publish events to update a simulated Edge KV store (use Redis locally to simulate the Edge KV).
  3. **Simulate Edge Worker Injection:** Create a middleware or proxy layer (simulating a Cloudflare Worker) that serves a basic HTML shell and injects the dynamic inventory data from the Redis KV before returning the response.
  4. **Acceptance Criteria:**
     - E2E tests must verify that an inventory update in the central database instantly reflects in the simulated Edge KV.
     - A simulated storefront request must return the pre-rendered SEO data and the correct dynamic inventory state without hitting the primary database.
     - Strict multi-tenant isolation must be maintained in the KV store key schemas.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
