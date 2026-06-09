issue_title: "Agentic Global Edge Caching & SEO Pre-rendering for OHC Storefronts"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  **Author:** Principal Product Researcher
  **Status:** Published
  **Date:** 2024-06-06

  ## 1. Executive Summary
  This research investigates the critical need for a Universal Edge-Cached Dynamic Storefront combined with Agentic SEO Pre-rendering for the OneHumanCorp (OHC) platform. It identifies a major gap in current offerings where small businesses (SMBs) suffer from slow load times during traffic spikes and poor search engine visibility due to dynamic rendering limitations. By leveraging edge caching and AI-driven SEO pre-rendering, OHC can provide enterprise-grade performance and discoverability to non-technical users invisibly.

  ## 2. Market Context & Pain Points
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to:
  - **High Latency & Timeouts:** Frustrating potential customers and increasing bounce rates.
  - **Lost Revenue:** Every second of delay directly impacts conversion rates.
  - **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Complexity Barrier:** SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## 3. Competitive Landscape
  - **Shopify:** Offers strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts. SEO is robust but often requires third-party apps for advanced optimization.
  - **Vercel/Next.js Ecosystem:** The gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users without significant development investment.
  - **Wix/Squarespace:** Provide easier SEO tools, but they still require manual configuration and lack the autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.

  ## 4. The OHC Differentiator: Invisible & Autonomous
  OHC's approach must go beyond providing caching infrastructure. It must be **invisible and autonomous**.
  - **Universal Edge Caching:** All storefront reads must hit a global edge cache (e.g., Cloudflare) automatically. No configuration needed by the user.
  - **Agentic Cache Invalidation:** When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally, ensuring accurate stock levels and preventing overselling.
  - **Agentic SEO Pre-rendering:** When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process. This generates highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge. This ensures web crawlers instantly see the most relevant, fast-loading version of the site, boosting organic ranking without the user lifting a finger.

  ## 5. Strategic Value to OHC
  Implementing this architecture positions OHC not just as a store builder, but as an enterprise-grade performance engine.
  - **Guaranteed Uptime & Speed:** Crucial for user trust during their most important moments (viral spikes).
  - **Automated Growth:** Agent-driven SEO pre-rendering passively increases organic traffic, directly impacting the SMB's bottom line.
  - **Cost Efficiency:** Offloading reads to the edge significantly reduces the load and scaling costs of the central PostgreSQL database.

  ## 6. Architecture & System Design
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts and product catalog.
  - **Edge Cache (Redis / Cloudflare Workers KV):** A distributed edge cache storing pre-rendered static HTML representations of product pages and storefronts. Key pattern: `ohc:edge_cache:{tenant_id}:storefront:{page_id}`.
  - **Cache Invalidation Queue:** A PostgreSQL-backed job queue for managing distributed cache purge events across edge nodes.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels. Upon inventory depletion, it enqueues a cache invalidation job for the affected product pages.
  - **Marketing Agent ("The Promoter"):** Triggers autonomous SEO pre-rendering whenever product descriptions, images, or storefront themes are updated. It analyzes content changes and updates meta tags/structured data accordingly before pushing to the edge cache.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] -->|Edge Network| B{Edge Cache}
      B -->|Hit: Static HTML| A
      B -->|Miss| C[OHC Storefront Renderer]
      C -->|Pre-Render| B
      D[Maya's Bakery Updates Content] --> E[Marketing Agent]
      E -->|Agentic Pre-Render & Metadata| C
      F[Customer Purchases Item] --> G[Operations Agent]
      G -->|Inventory Update| H[PostgreSQL DB]
      G -->|Cache Purge Event| B
  ```

  ### Mobile-First Implementation
  - Storefronts served from the edge cache must be highly optimized for mobile devices (375px viewports).
  - Images should be lazily loaded and compressed into next-gen formats (WebP).
  - Initial HTML payload must be minimal to achieve near-instant First Contentful Paint (FCP).

  ## 7. Implementation Prompt
  **User-Facing Outcome:** As a business owner, my website loads instantly for every customer worldwide, even during viral traffic spikes, and it ranks highly on Google—without me ever configuring a CDN, SEO settings, or caching rules. The AI handles it all invisibly.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A tenant (e.g., "Maya's Bakery") has an active OHC storefront with a product catalog.
  2. The Marketing Agent updates the description for "Vegan Chocolate Cake."
  3. The system autonomously triggers an SEO pre-rendering job.
  4. The job generates optimized static HTML with updated meta tags and pushes it to the simulated Edge Cache (Redis).
  5. A customer (simulated client) requests the product page and receives the pre-rendered HTML from the cache with minimal latency (bypassing the core DB).
  6. The Operations Agent processes a sale that depletes the stock of "Vegan Chocolate Cake."
  7. The system instantly invalidates the corresponding edge cache entry.
  8. The next customer request fetches the updated (sold out) state and re-caches it.
  9. **Playwright E2E Tests:** Verify cache hits/misses, confirm cache invalidation upon inventory updates, and ensure the delivered HTML contains the correct SEO meta tags.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
