issue_title: "Implement Agentic SEO Pre-rendering & Universal Edge Caching"
issue_description: |
  # Architecture: Agentic SEO Pre-rendering & Universal Edge Caching

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency and timeouts, which frustrate potential customers and increase bounce rates. Moreover, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## Research Report
  - **Market Context:** Platforms like Shopify offer strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts. SEO is robust but often requires third-party apps for advanced optimization. Vercel/Next.js Ecosystem is the gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users without significant development investment. Wix/Squarespace provide easier SEO tools, but they still require manual configuration and lack the autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.
  - **The OHC Differentiator:** OHC's approach must go beyond providing caching infrastructure. It must be invisible and autonomous.
    - **Universal Edge Caching:** All storefront reads must hit a global edge cache (e.g., Cloudflare) automatically. No configuration needed by the user.
    - **Agentic Cache Invalidation:** When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally, ensuring accurate stock levels and preventing overselling.
    - **Agentic SEO Pre-rendering:** When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process. This generates highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge. This ensures web crawlers instantly see the most relevant, fast-loading version of the site, boosting organic ranking without the user lifting a finger.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Marketing Agent] -->|Update Website| B(SEO Pre-rendering Pipeline)
      B --> C[Generate Static HTML & Meta Tags]
      C --> D[Push to Edge Cache / CDN]
      E[Operations Agent] -->|Update Inventory| F(Cache Invalidation Worker)
      F --> D
      G[Web Crawler / User] -->|Request Page| D
  ```

  ### Mobile UX Flow (375px)
  This feature is mostly invisible to the user. However, a success notification can be surfaced in the Agent Feed:
  1.  **Card on Feed:** "The Promoter Agent has optimized your storefront for search engines. View Preview [Button]"
  2.  **Preview Screen:** A simple preview of how the store looks on Google Search.

  ### Key Design Decisions
  - **Zero Configuration:** The user does not configure caching rules or SEO meta tags manually. The AI determines the optimal tags based on the storefront content.
  - **Proactive Pre-rendering:** Instead of Server-Side Rendering on request (which can be slow on cold starts), the system statically generates the storefront on every content/inventory change.

  ## Implementation Prompt
  **Feature Name:** Agentic SEO Pre-rendering & Edge Caching
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya updates her storefront with a new cake. Behind the scenes, the system automatically generates a statically rendered HTML version with perfect SEO meta tags and pushes it to an edge cache. If she sells out, the cache is instantly invalidated to reflect "Sold Out" globally.

  **Next Actions:**
  1. Implement a pipeline to generate statically rendered HTML pages for the storefront (product pages, home page).
  2. Integrate an AI task (e.g., in `local_seo.rs` or `growth.rs`) that automatically generates appropriate SEO `<meta>` tags and structured JSON-LD data for the storefront based on the catalog data.
  3. Implement a generic interface for interacting with an Edge Cache / CDN (e.g., setting cache keys and sending invalidation requests).
  4. Wire up the Cache Invalidation Worker to trigger whenever inventory changes (via Operations Agent or direct purchase).
  5. Add a simple notification in the Agent Feed indicating successful SEO optimization.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
