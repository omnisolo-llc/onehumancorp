issue_title: "Implement Edge-Cached Dynamic Storefronts with Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical owners like Maya (baker) and Leo (creator) rely on social media virality. When a post goes viral, traffic spikes can overwhelm unoptimized storefronts, causing slow load times, high bounce rates, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. These owners lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG).

  ## Research Report
  - **Competitor Landscape:** Platforms like Shopify offer strong edge delivery but require third-party apps for advanced SEO. Developer-focused tools (Vercel/Next.js) offer excellent performance (ISR, Edge computing) but are inaccessible to non-technical users. Wix/Squarespace provide easier SEO tools but require manual configuration and lack autonomous scalability.
  - **The OHC Differentiator:** OHC must provide Universal Edge Caching and Agentic SEO Pre-rendering invisibly. All storefront reads hit a global edge cache automatically. Agentic cache invalidation ensures accurate stock levels instantly when inventory changes. Agentic SEO pre-rendering automatically generates optimized static HTML with meta tags and structured data upon site updates.

  ## Design Doc
  ### Architecture
  - **Universal Edge Caching:** Storefront read requests are served from a CDN/Edge cache (e.g., Cloudflare/Fastly equivalent).
  - **Agentic Cache Invalidation:** The Operations Agent monitors inventory changes (e.g., via PostgreSQL logical replication or application events) and immediately purges specific edge cache keys to prevent overselling.
  - **Agentic SEO Pre-rendering:** The Marketing Agent detects content/catalog updates and triggers a headless browser (or equivalent pre-rendering pipeline) to generate static, SEO-optimized HTML for storefront pages, pushing the results to edge storage.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      Client[Mobile/Web Client] -->|Storefront Request| CDN[Global Edge Cache]
      CDN -->|Cache Miss| API[OHC Gateway API]
      API --> DB[(Central PostgreSQL)]

      Owner[Owner App] -->|Updates Catalog| API
      API --> DB

      DB -- Change Data Capture --> OpAgent[Operations Agent]
      DB -- Event Stream --> MktAgent[Marketing Agent]

      OpAgent -->|Purge Key| CDN
      MktAgent -->|Pre-render HTML| Headless[Headless Browser]
      Headless -->|Push Static HTML| CDN
  ```

  ### Mobile UX Flow
  - Maya updates her cake catalog in the OHC app (375px viewport).
  - A toast notification (translucent glass style) appears: "Updating storefront...".
  - The AI Assistant (Marketing Agent) works invisibly in the background to pre-render the new catalog page.
  - A second toast notification confirms: "Storefront updated and optimized for search."
  - Customers loading the storefront on mobile experience sub-second load times even during viral traffic spikes.

  ### AI Agent Integration Points
  - **Marketing Agent:** Listens for catalog/content updates and orchestrates the SEO pre-rendering pipeline.
  - **Operations Agent:** Listens for inventory changes (sales, restocks) and orchestrates targeted edge cache invalidation.

  ## Implementation Prompt
  **User Outcome:** The owner updates their catalog, and the system automatically ensures the storefront is lightning-fast and SEO-optimized without any manual configuration.
  **CUJ:**
  1. Maya logs into the OHC mobile app.
  2. She adds a new "Summer Strawberry Cake" to her catalog.
  3. The system saves the item to the database.
  4. The Marketing Agent automatically triggers an SEO pre-rendering job for the storefront catalog page.
  5. The pre-rendered page is cached at the edge.
  6. A customer visits the storefront and receives the lightning-fast, pre-rendered page.

  **Acceptance Criteria:**
  - Implement the core logic for the Marketing Agent to detect catalog updates and trigger pre-rendering.
  - Implement a mechanism to store and serve the pre-rendered content (simulating edge caching locally if necessary).
  - Provide a clear, actionable UI notification to the owner during the update process.

  ## Top 5 Things That Do Not Make Sense In Codebase
  1. Lack of a unified event streaming strategy for agent communication (mix of direct calls, cron, and basic queues).
  2. Potential race conditions in multi-tenant data isolation when concurrent writes happen.
  3. Frontend code contains deeply nested API logic rather than abstracted data service layers.
  4. Redundant types and DTOs duplicated across different microservices.
  5. Missing automated E2E tests for the caching layer invalidation.

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
