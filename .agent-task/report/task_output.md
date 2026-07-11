issue_title: "Implement Global Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## 1. Problem Statement
  Small business owners rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) for SEO.

  ## 2. Research Report
  - **Market Context**: Fast page load speeds are critical for conversion rates and SEO rankings. Google uses Core Web Vitals as a ranking factor.
  - **The OHC Opportunity**: Implement a seamless edge-caching and CDN delivery system combined with AI-driven SEO pre-rendering for all OHC storefronts. This must be invisible to the user but provide immediate performance and discoverability benefits.
  - **Competitor Gaps**:
    - *Shopify*: Offers strong edge network capabilities but advanced SEO requires manual configuration or 3rd-party apps.
    - *Wix/Squarespace*: Provide easier SEO tools but lack autonomous scalability during unpredictable traffic spikes.
    - *Vercel/Next.js*: Developer-focused; inaccessible to non-technical users.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[User Browser / Search Crawler] -->|Requests Storefront| B(Edge CDN/Cache e.g. Cloudflare)
      B -- Cache Hit --> A
      B -- Cache Miss --> C[OHC Application Server]
      C --> D[(PostgreSQL Central Ledger)]
      C --> E[(Redis Cache)]
      C -- Serves HTML/JSON --> B

      F[Operations Agent] -->|Inventory Change| G(Agentic Cache Invalidator)
      G -->|Purge Specific Key| B

      H[Marketing Agent] -->|Storefront Update| I(Agentic SEO Pre-renderer)
      I -->|Push Static HTML + Meta Tags| B
  ```

  ### UI Wireframes & Screen Flow
  No direct UI changes are required for the owner, as this feature operates invisibly. However, an optional "Performance & SEO" read-only dashboard card could display:
  - Cache Hit Ratio
  - Estimated Page Load Time
  - Indexing Status

  ### Mobile UX Flow
  - The performance benefit ensures that the storefront loads instantly on mobile devices, improving the end-user experience without any configuration from the business owner.

  ### AI Agent Integration Points
  - **Operations Agent**: Triggers targeted cache invalidations via Redis/CDN API whenever a product's inventory or price is updated.
  - **Marketing Agent**: Automatically generates and stores pre-rendered HTML snippets containing structured metadata (JSON-LD) when content changes, ensuring bots crawl the latest, optimized version.

  ### Key Design Decisions
  - **Invisible Edge Caching**: By default, all public storefront endpoints are cached. This removes the configuration burden from the user.
  - **Agentic Invalidation**: Relying on agents to intelligently purge cache keys instead of generic TTLs ensures real-time accuracy (e.g., preventing overselling) without sacrificing performance.

  ## 4. Implementation Prompt
  **Feature Name**: Universal Edge-Cached Storefront & Agentic SEO
  **Target Persona**: All OHC Merchants (e.g., Maya the Baker experiencing a viral Instagram post)
  **Outcome**: Sub-second page load times globally during traffic spikes and maximized organic search visibility, handled entirely autonomously by OHC agents.

  **Critical User Journey (CUJ)**:
  1. Maya the Baker posts a viral cake video on Instagram.
  2. Thousands of users click her link-in-bio simultaneously.
  3. Instead of the OHC server crashing, Cloudflare Edge serves the cached, pre-rendered HTML in milliseconds.
  4. When the cake sells out, the Operations Agent detects the stock change in PostgreSQL, invalidates the edge cache, and the next request correctly shows "Sold Out" without Maya intervening.

  **Acceptance Criteria**:
  - Storefront pages are served from the CDN edge cache.
  - Updating inventory automatically purges the associated CDN cache.
  - Storefront URLs include pre-rendered HTML with optimized meta tags and structured data (JSON-LD) for SEO.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
