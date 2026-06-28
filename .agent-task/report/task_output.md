issue_title: "Implement High-Performance Agentic SEO Pre-rendering & Universal Edge-Cached Dynamic Storefront"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to:
  - **High Latency & Timeouts:** Frustrating potential customers and increasing bounce rates.
  - **Lost Revenue:** Every second of delay directly impacts conversion rates.
  - **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Complexity Barrier:** SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Offers strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts. SEO is robust but often requires third-party apps for advanced optimization.
  - **Vercel/Next.js Ecosystem:** The gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users without significant development investment.
  - **Wix/Squarespace:** Provide easier SEO tools, but they still require manual configuration and lack the autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.
  - **OHC Opportunity:** Implement an invisible, autonomous caching architecture. All storefront reads must hit a global edge cache automatically. When inventory changes, the Operations Agent instantly purges the specific edge cache key globally, ensuring accurate stock levels. Furthermore, the Marketing Agent should autonomously trigger a pre-rendering process upon website updates, generating highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Browser/Crawler] -->|Request| B(Universal Edge Cache / CDN)
      B -->|Cache Hit| A
      B -->|Cache Miss| C[Dynamic Storefront Engine]
      C --> D[PostgreSQL Central Ledger]

      E[Operations Agent] -->|Inventory Update| F(Cache Invalidation Queue)
      F -->|Purge Key| B

      G[Marketing Agent] -->|Content Update| H(SEO Pre-render Worker)
      H -->|Generate Static HTML + Meta| I(Blob Storage / Edge Push)
      I --> B
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - This is primarily a backend and infrastructure feature, but its effect is visible on the frontend.
  - **Storefront (Mobile):** The main storefront (e.g., viewing a product catalog) should load instantly (< 100ms) on a 375px device, even under heavy load.
  - **Action:** No direct user action is required for caching or pre-rendering; it happens invisibly. The owner might see a notification in the Agent Feed: "Marketing Agent optimized your store for Google Search."
  - **Visual Design:** The storefront UI remains consistent with the current OHC design system (glassmorphism, clear typography), but its perceived performance is dramatically improved.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Integrated with the inventory management system. When stock changes (e.g., via POS offline sync or online purchase), it dispatches a cache invalidation event for the affected product pages.
  - **Marketing Agent ("The Promoter"):** Integrated with the content management system. When a product description or store policy is updated, it triggers a background worker to pre-render the affected pages, optimizing meta tags and structured data (JSON-LD) using LLMs, and pushing the static HTML to the edge cache.

  ### Key Design Decisions
  - **Autonomous Operation:** The user never configures caching rules or SEO meta tags manually. The agents handle this based on high-level business goals.
  - **Edge-First Architecture:** By default, all public-facing read requests should be served from the edge cache.
  - **Granular Invalidation:** Cache invalidation must be highly granular (e.g., by product ID or tenant ID) to avoid unnecessary cache misses.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when my store goes viral on TikTok, my storefront loads instantly for thousands of simultaneous visitors without crashing. When people search for my products on Google, my pages rank higher because they are pre-rendered and optimized for SEO automatically.
  **CUJ & Acceptance Criteria:**
  1. A new product is created in the OHC platform.
  2. The Marketing Agent automatically generates SEO-optimized meta tags and triggers a pre-rendering job.
  3. The pre-rendered HTML is stored in the Universal Edge Cache.
  4. A request to the product page is served directly from the edge cache (Cache Hit) with ultra-low latency.
  5. The product goes out of stock.
  6. The Operations Agent automatically purges the cache for that specific product page.
  7. Provide Playwright E2E tests: A test script simulates high concurrency reads against a product page, verifying that responses are served from the cache (indicated by headers) and that the cache is correctly invalidated upon a stock change.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
