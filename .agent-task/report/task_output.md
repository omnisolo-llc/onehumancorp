issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  ## Title
  Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture

  ## Problem Statement
  Small business owners like Maya the Baker or Leo the Musician rely on their storefronts to convert social media traffic (e.g., from a viral TikTok or Instagram post) into sales. However, massive traffic spikes often overwhelm traditional unoptimized dynamic databases, resulting in high latency, connection timeouts, lost revenue, and poor customer experiences. Additionally, these dynamic storefronts suffer from poor Search Engine Optimization (SEO) because web crawlers struggle to index slow, client-side rendered content. SMB owners lack the technical expertise to set up CDNs, edge caching, or Server-Side Rendering (SSR) / Static Site Generation (SSG).

  ## Research Report
  - **Competitive Analysis:**
    - **Shopify:** Utilizes edge networks (Cloudflare) to deliver fast storefronts, but advanced SEO often requires expensive third-party apps and manual configuration.
    - **Wix / Squarespace:** Offer basic SEO tools, but they lack autonomous, instant scalability for dynamic components during unpredictable, massive traffic spikes.
    - **Vercel / Next.js:** The industry standard for edge computing and Incremental Static Regeneration (ISR), but inaccessible to non-technical users.
  - **OHC Opportunity:** Provide enterprise-grade performance and discoverability completely invisibly. OHC can build a universal edge caching layer combined with an AI-driven SEO pre-rendering process that autonomously builds static, fast-loading storefronts.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer/Web Crawler] -->|HTTP Request| B(Global Edge CDN - e.g., Cloudflare Workers)
      B -->|Cache Hit| C[Deliver Pre-rendered Static HTML]
      B -->|Cache Miss| D[Frontend Server]
      D --> E[(Tenant DB - PostgreSQL)]

      F[Operations Agent] -->|Inventory Update / Sold Out| G[Agentic Cache Invalidator]
      G -->|Purge Specific Key| B

      H[Marketing Agent] -->|Content / Product Added| I[Agentic SEO Pre-renderer]
      I -->|Generate Static HTML & Meta Tags| B
  ```

  ### Mobile UX Flow (375px First)
  - **Zero User Configuration:** The business owner sees absolutely no "Caching" or "SEO Optimization" toggles in the primary UI. It is 100% invisible.
  - **Operations Agent Feed (Mobile):** If an item sells out during a viral spike, the Operations Agent sends a mobile feed card: *"Your 'Signature Vegan Cake' just sold out and was updated on your storefront to prevent overselling. Would you like to draft a restock order?"*
  - **Marketing Agent Feed (Mobile):** When Maya adds a new cake, the Marketing Agent sends a notification: *"I've generated a new SEO-optimized page for 'Chocolate Ganache' and pushed it to our edge network so it loads instantly for your customers. I also added keywords for 'local chocolate cake'."*

  ### AI Agent Integration Points
  - **Agentic Cache Invalidator (Operations):** Monitors the core event mesh. When a critical state changes (e.g., inventory hits 0), it instantly triggers a cache invalidation request to the Edge CDN for the specific product or storefront path.
  - **Agentic SEO Pre-renderer (Marketing):** Detects when new products or services are published. It acts as an internal crawler, rendering the dynamic page, injecting optimized meta titles, descriptions, and JSON-LD structured data (schema markup), and pushing the resulting static HTML to the edge cache.

  ### Key Design Decisions
  - **Invisible by Default:** The complex orchestration of Edge CDN invalidation and ISR is completely abstracted away from the user.
  - **Edge First:** All storefront reads must hit the global edge cache automatically. Only dynamic cart mutations hit the central DB.
  - **Proactive SEO:** The Marketing Agent generates metadata based on product context without waiting for the owner to manually enter it.

  ## Implementation Prompt
  **User-Facing Outcome:** A non-technical owner like Maya experiences zero downtime or latency when her store goes viral. Web crawlers index her fast-loading pages immediately, improving her Google ranking, all without her touching a single technical setting.
  **CUJ & Acceptance Criteria:**
  1. An external user adds a new product via the OHC UI.
  2. The Marketing Agent automatically intercepts the "ProductCreated" event.
  3. The Marketing Agent pre-renders the product page HTML, generates SEO meta tags (title, description, OpenGraph, JSON-LD schema), and caches it at the edge layer.
  4. The Operations Agent updates the edge cache whenever inventory changes.
  5. Provide Playwright E2E tests: A user creates a product. Simulate a direct HTTP GET request to the public product URL and verify that the response is served instantly from the cache, containing the AI-generated SEO meta tags and accurate inventory status, without hitting the primary database.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []