issue_title: "Implement Universal Edge-Cached Dynamic Storefront with Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Title
  Implement Universal Edge-Cached Dynamic Storefront with Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and SEO penalties. Current dynamic storefronts are too slow to load during peak traffic and suffer from poor SEO due to client-side rendering. There is a critical missing architectural piece: an autonomous edge-caching layer coupled with AI-driven SEO pre-rendering that works invisibly for the owner.

  ## Research Report
  - **Market Context**: Platforms like Shopify use robust edge networks (e.g., Cloudflare) for fast global delivery, but advanced SEO often requires third-party apps. Platforms relying on client-side rendering (SPA) suffer from poor crawler indexing unless SSR/SSG is configured, which is inaccessible to SMBs. Vercel/Next.js ecosystem provides ISR and Edge computing, but only for developers.
  - **The OHC Differentiator**: OHC can automate this entirely and make it invisible. All storefront reads must hit a global edge cache automatically. When an SMB updates inventory or content, an Agent must invalidate the edge cache automatically to ensure accurate stock levels. Furthermore, the Marketing Agent should autonomously trigger a pre-render of static HTML with injected SEO meta tags whenever storefront content changes.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as SMB Owner
      participant App as OHC Platform (Operations)
      participant DB as PostgreSQL
      participant Agent as Marketing/Operations Agent
      participant Cache as Edge Cache (Redis/Cloudflare mock)
      participant Customer as Web Customer/Crawler

      Owner->>App: Updates Product/Content
      App->>DB: Save Changes
      DB-->>Agent: Trigger Event (CDC / Webhook / Job Queue)
      Agent->>Agent: Generate SEO Meta & Pre-render HTML
      Agent->>Cache: Invalidate old & Push new HTML
      Customer->>Cache: Request Storefront
      Cache-->>Customer: Serve Static HTML (Edge Speed)
  ```

  ### Mobile UX Flow
  - **375px First**: The owner's view on mobile remains the simple "Update Product" or "Update Storefront" screen without new complex configurations.
  - The heavy lifting (pre-rendering, cache invalidation, edge distribution) happens invisibly in the background.
  - No new complex UI elements are added to the critical path. A simple "SEO Optimized & Live" badge or a non-blocking toast notification can be shown asynchronously when the edge cache is successfully seeded.

  ### AI Agent Integration
  - **Operations Agent**: Monitors real-time inventory changes. When an item sells out, it instantly triggers a targeted cache invalidation for that product page globally, ensuring accurate stock levels and preventing overselling.
  - **Marketing Agent**: Hooked into storefront content changes. Autonomously generates optimal Alt tags, Meta titles, and descriptions, then orchestrates the HTML pre-render process and pushes it to the edge cache.

  ### Key Design Decisions
  - Use an edge-caching mechanism (e.g., Redis layer abstracting Edge Cache) for all storefront read operations.
  - Pre-render HTML server-side or via an Agent background task to serve directly to crawlers and initial user requests.
  - Cache invalidation MUST be event-driven based on inventory locks/deductions and content updates, rather than TTL-based, to guarantee stock accuracy.

  ## Implementation Prompt
  Implement the universal edge-caching and agentic SEO pre-rendering pipeline for OHC storefronts.
  - **CUJ:** Maya updates her custom cake catalog. Behind the scenes, the Marketing Agent pre-renders the HTML with updated SEO tags and caches it. A customer clicks her viral TikTok link and instantly loads the cached storefront. Later, when a cake sells out, the Operations Agent invalidates the cache so subsequent visitors see truthful stock availability.
  - **Acceptance Criteria:**
    1. Introduce a caching layer (e.g., Redis) to store pre-rendered storefront HTML pages.
    2. Implement a background job (AI Agent driven) that triggers upon storefront or product updates. This job should generate SEO metadata, construct static HTML, and populate the cache.
    3. Modify the storefront API endpoint to intercept read requests, check the cache first, and return the pre-rendered HTML if available (fallback to dynamic rendering if miss).
    4. Ensure cache invalidation is correctly triggered on inventory changes to prevent overselling.
    5. No complex configuration exposed to the owner—it must happen autonomously.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
