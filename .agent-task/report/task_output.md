issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Executive Summary
  This research investigates the critical need for a Universal Edge-Cached Dynamic Storefront combined with Agentic SEO Pre-rendering for the OneHumanCorp (OHC) platform. It identifies a major gap in current offerings where small businesses (SMBs) suffer from slow load times during traffic spikes and poor search engine visibility due to dynamic rendering limitations. By leveraging edge caching and AI-driven SEO pre-rendering, OHC can provide enterprise-grade performance and discoverability to non-technical users invisibly.

  ## 1. Market Context & Pain Points
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to:
  - **High Latency & Timeouts:** Frustrating potential customers and increasing bounce rates.
  - **Lost Revenue:** Every second of delay directly impacts conversion rates.
  - **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Complexity Barrier:** SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## 2. Competitive Landscape
  - **Shopify:** Offers strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts. SEO is robust but often requires third-party apps for advanced optimization.
  - **Vercel/Next.js Ecosystem:** The gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users without significant development investment.
  - **Wix/Squarespace:** Provide easier SEO tools, but they still require manual configuration and lack the autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.

  ## 3. The OHC Differentiator: Invisible & Autonomous
  OHC's approach must go beyond providing caching infrastructure. It must be **invisible and autonomous**.
  - **Universal Edge Caching:** All storefront reads must hit a global edge cache automatically. No configuration needed by the user.
  - **Agentic Cache Invalidation:** When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally, ensuring accurate stock levels and preventing overselling.
  - **Agentic SEO Pre-rendering:** When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process. This generates highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge. This ensures web crawlers instantly see the most relevant, fast-loading version of the site, boosting organic ranking without the user lifting a finger.

  ## 4. Architecture Design & Mobile Flow
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Merchant (Mobile App)
      participant OHC as OHC Backend
      participant Agent as Marketing Agent
      participant Storage as Pre-render Storage
      participant Edge as Global Edge Cache
      participant Customer as Customer (Browser)

      Owner->>OHC: Adds new Product
      OHC->>Agent: Trigger 'CatalogUpdated' Event
      Agent->>Agent: Generate SEO Meta & JSON-LD
      Agent->>Storage: Pre-render Static HTML with Meta
      Storage->>Edge: Push new HTML snapshot
      Customer->>Edge: Request Product Page
      Edge-->>Customer: Serve Static HTML (<50ms)
      Customer->>OHC: Purchases Last Item
      OHC->>Edge: Targeted Cache Invalidate (Product Key)
  ```

  ### Mobile UX Flow (375px)
  1. The merchant operates entirely in a mobile-first environment (375px wide).
  2. When a viral spike hits, the primary mobile dashboard displays a toast/card: "High traffic detected! Edge delivery is currently serving 10x normal load seamlessly."
  3. All complex configuration (cache TTLs, CDN endpoints) is completely hidden. The UI only shows the results of the agent's work.

  ## 5. Strategic Value to OHC
  Implementing this architecture positions OHC not just as a store builder, but as an enterprise-grade performance engine.
  - **Guaranteed Uptime & Speed:** Crucial for user trust during their most important moments (viral spikes).
  - **Automated Growth:** Agent-driven SEO pre-rendering passively increases organic traffic, directly impacting the SMB's bottom line.
  - **Cost Efficiency:** Offloading reads to the edge significantly reduces the load and scaling costs of the central PostgreSQL database.

  ## 6. Implementation Prompt & Technical Flow

  **Feature Name:** Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering
  **Target Persona:** Maya the Baker (relies on Instagram viral posts, needs fast load times and good SEO)

  **Outcome:** Maya's storefront is instantly fast globally due to edge caching. When she adds a new product, the Marketing Agent autonomously generates SEO metadata and pre-renders the page, and the Operations Agent manages cache invalidation when stock runs out.

  **Critical User Journey (CUJ):**
  1. Maya adds a new "Vegan Chocolate Cake" to her OHC product catalog.
  2. The Marketing Agent is triggered, automatically generates an optimized meta title, description, and structured data (JSON-LD) for the new product.
  3. The system triggers a pre-render job, creating a static HTML snapshot of the product page with the injected SEO metadata.
  4. The pre-rendered page is pushed to the global edge cache.
  5. A customer in another country clicks a link to the cake. The edge cache serves the pre-rendered HTML in <50ms.
  6. The customer buys the last cake. The Operations Agent triggers an immediate cache invalidation for that specific product key to update the "Sold Out" state.

  **Implementation Prompt for Engineering:**
  Design and implement the caching and pre-rendering pipeline.
  1.  **Cache Invalidation System:** Implement a mechanism (e.g., Redis pub/sub or an event bus) where inventory updates or catalog changes trigger targeted cache purges.
  2.  **Agentic SEO Pre-rendering:** Create a worker job that the Marketing Agent can trigger. This job should take product data, use the LLM to generate SEO metadata, inject it into an HTML template, and store the result (in an edge-compatible store or CDN).
  3.  **Storefront Routing:** Ensure storefront reads first attempt to fetch from the edge cache/pre-rendered store before falling back to dynamic database queries.
  4.  **No manual configuration for the user.** Do not expose CDN settings or SEO metadata fields in the primary UI unless placed behind an "Advanced Options" toggle.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
