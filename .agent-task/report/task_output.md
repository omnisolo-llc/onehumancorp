issue_title: "[Research] Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## 1. Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, leading to:
  - **High Latency & Timeouts:** Frustrating potential customers and increasing bounce rates.
  - **Lost Revenue:** Every second of delay directly impacts conversion rates.
  - **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Complexity Barrier:** SMBs lack the technical expertise to configure CDNs, caching layers, or SSR/SSG.

  ## 2. Research Report
  - **Shopify:** Offers strong edge network capabilities via Cloudflare. SEO is robust but requires apps for advanced optimization.
  - **Vercel/Next.js:** Gold standard for developers (ISR, Edge) but inaccessible to non-technical users.
  - **Wix/Squarespace:** Easier SEO tools but lack autonomous instant scalability during unpredictable viral spikes.
  - **OHC Opportunity:** Universal Edge Caching combined with Agentic SEO Pre-rendering. All storefront reads must hit a global edge cache automatically. AI agents autonomously manage cache invalidation and trigger pre-rendering to optimize static HTML for web crawlers instantly, ensuring peak performance and SEO without user configuration.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B(Global Edge CDN)
      B -->|Cache Hit| C[Deliver Static Page/Cache]
      B -->|Cache Miss| D[Dynamic Render Engine]
      D --> E[PostgreSQL Database]
      F[Operations Agent] -->|Inventory Change| G[Edge Cache Invalidator]
      H[Marketing Agent] -->|Content Update| I[SEO Pre-rendering Service]
      I -->|Push Static HTML| B
  ```

  ### Mobile UX Flow (375px First)
  - **SEO & Performance Dashboard:** A single summary card under the Marketing section.
  - **Display Only:** The card displays "Site Speed: Lightning Fast" and "SEO Status: Optimized for Google". No complex configuration toggles.
  - **Agent Feedback:** An activity feed shows cards like "Operations Agent purged edge cache for 'Vegan Cake' (Out of Stock)" or "Marketing Agent re-rendered storefront for faster Google indexing."

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** When inventory hits 0, it calls an internal API to invalidate the specific edge cache key globally, preventing overselling.
  - **Marketing Agent (The Promoter):** When product descriptions or images are updated, it queues a background job to perform SEO pre-rendering (injecting meta tags, JSON-LD) and pushes the static assets to the Edge CDN.

  ### Key Design Decisions
  - **Zero-Config Default:** The SMB owner does not see options for TTL, cache headers, or SSR vs. CSR. Everything is handled invisibly.
  - **Stale-While-Revalidate:** Use SWR patterns for storefront APIs to ensure customers never experience a cold start while content updates in the background.

  ## 4. Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront layer with Agent-driven cache invalidation and SEO pre-rendering.
  1. Introduce an Edge CDN integration (e.g., Cloudflare Workers or Fastly) for storefront routes.
  2. Implement an internal API for precise cache invalidation by key (e.g., `storefront:tenant_id:product_id`).
  3. Create an SEO Pre-rendering Service that generates static HTML with optimized meta tags and schema markup.
  4. Integrate the Operations Agent to trigger cache invalidation upon inventory changes.
  5. Integrate the Marketing Agent to trigger the Pre-rendering Service upon content updates.
  6. E2E Test: Modify a product's stock to 0, verify the cache invalidation job runs, and ensure subsequent requests from the edge return the updated "Out of Stock" state instantly.

  ## 5. Estimated Scope
  Large

  ## 6. Priority
  P1
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
