issue_title: "OHC Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Architecture Design Document: Edge-Cached Dynamic Storefront & Agentic SEO

  ## Problem Statement
  Small business owners (SMBs) like Maya the Baker or Leo the Musician rely on social media virality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, causing high latency, timeouts, and lost revenue. In addition, SMBs struggle to configure Server-Side Rendering (SSR) or Static Site Generation (SSG) correctly for SEO, leading to poor search engine visibility. They need a system that autonomously handles high-traffic spikes via edge caching and pre-renders content for web crawlers without any manual configuration.

  ## Research Report
  - **Competitor Gaps**: Shopify provides strong edge networks (via Cloudflare) but requires 3rd-party apps for advanced SEO. Vercel/Next.js is the gold standard for developers but inaccessible to non-technical users. Wix/Squarespace provide basic SEO but lack autonomous, instant scalability during massive spikes.
  - **The OHC Opportunity**: Implement "Invisible & Autonomous" caching and SEO. All storefront reads hit a global edge cache automatically. When inventory changes, the Operations Agent autonomously invalidates the specific edge cache key. When marketing content updates, the Marketing Agent triggers SEO pre-rendering to generate optimized static HTML.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer/Web Crawler] -->|HTTP Request| B(Global Edge Cache)
      B -->|Cache Hit| C[Deliver Static/Cached Content]
      B -->|Cache Miss| D[PostgreSQL/Dynamic Render]
      D --> B
      E[Operations Agent] -->|Inventory Change| F[Cache Invalidation API]
      F -->|Purge Key| B
      G[Marketing Agent] -->|Content Update| H[Agentic SEO Pre-renderer]
      H -->|Generate Static HTML| B
  ```

  ### Mobile UX Flow (375px)
  - **Zero Configuration**: The user sees no caching or SEO configuration settings. The entire process happens invisibly in the background.
  - **Performance Signals**: The owner dashboard displays plain-language health signals, e.g., "Your storefront is loading in 0.2s and is optimized for Google."
  - **Agent Feed**: When the SEO agent completes a major pre-rendering pass, a card appears in the Agent Feed: "I've optimized your new cake listings for Google Search. They are now lightning fast."

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager")**: Listens for inventory or product changes and issues targeted cache invalidation requests to the Edge Cache.
  - **Marketing Agent ("The Promoter")**: Triggers background tasks to pre-render full HTML pages with optimized meta tags and structured JSON-LD data when content changes.

  ## Implementation Prompt
  - **Objective**: Build an autonomous edge caching and SEO pre-rendering system.
  - **Tasks**:
    1. Implement a caching layer proxy that intercepts storefront reads and serves cached responses.
    2. Build an API for targeted cache invalidation.
    3. Modify the Operations Agent to trigger cache invalidation upon inventory updates.
    4. Implement a background worker (Agentic SEO Pre-renderer) that generates static HTML with SEO metadata when product content changes.
  - **Constraint**: Do NOT prescribe specific database schemas or edge providers (e.g., Cloudflare vs. Fastly) at this stage. Focus on the internal caching proxy, the invalidation API, and the worker architecture.

  ## Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
