issue_title: "[Architecture] Edge-Cached Dynamic Storefronts & Agentic SEO"
issue_description: |
  # Research Report: Edge-Cached Dynamic Storefronts & Agentic SEO

  ## Problem Statement
  For SMBs like Maya (baker) or Priya (boutique), discovering demand is just as critical as fulfilling it. Traditional platforms (Shopify, Wix) generate static or slow-loading dynamic pages that require manual SEO optimization (meta tags, sitemaps, structured data). Non-technical owners do not have the time or expertise to manage SEO. OHC needs a system where the AI Agent automatically generates, optimizes, and pre-renders highly performant storefront pages (sub-100ms TFB) distributed via edge caching. The storefront must feel "alive" to search engines without any manual configuration.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Fast CDN, but requires manual SEO setup or expensive third-party apps for advanced schema and structured data.
  - **Wix:** Historically poor SEO, now improved with "SEO Wiz," but still requires manual input and checklist completion.
  - **Next.js/Vercel Ecosystem:** Provides the technical capability (ISR, Edge Functions) but requires developers.

  **Market Needs:**
  SMBs need a platform where adding a new product (e.g., "Vegan Chocolate Cake") automatically generates a highly optimized landing page, complete with rich snippets (Schema.org), localized keywords, and pre-rendered HTML distributed to the edge. The system should proactively monitor search trends and suggest new category pages or blog content to capture localized intent.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Cloud/Edge
          CDN[Edge CDN] --> |Cache Miss| Origin[OHC API Gateway];
          CDN --> |Cache Hit| UserBrowser[Customer Browser];
          Origin --> StorefrontService[Storefront Pre-render Service];
      end

      subgraph OHC Backend
          StorefrontService --> DB[(Main DB / Products)];
          StorefrontService --> MarketingAgent[Marketing & SEO Agent];
      end

      MarketingAgent --> |Auto-generates| MetaData[SEO Meta & Schema];
      MarketingAgent --> |Monitors| Analytics[Search Trends];
      OpsAgent[Ops Agent] --> |Updates Inventory| DB;
      DB --> |Triggers Invalidation| CDN;
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard Feed:** Maya opens the OHC app. The Agent Feed displays a card: "Your new 'Vegan Chocolate Cake' product page is live and optimized for local searches in Austin. [View Page]".
  2. **Zero Configuration:** There are no "SEO Settings" tabs. The system infers the best keywords from the product description and business context.
  3. **Proactive Suggestions:** Another card appears: "We noticed high search volume for 'gluten-free baked goods' near you. Should I create a dedicated landing page for your gluten-free items? [Yes, draft page]".

  ### AI Agent Integration Points
  - **Marketing Agent (SEO):** Automatically generates meta descriptions, title tags, alt text for images, and JSON-LD structured data for every product and category page.
  - **Ops Agent (Cache Invalidation):** When inventory changes (e.g., product sold out), it triggers targeted cache invalidation at the edge to ensure customers always see accurate availability.

  ### Key Design Decisions
  - **Incremental Static Regeneration (ISR) / Edge Rendering:** Storefront pages must be pre-rendered for maximum speed and SEO performance, but capable of updating dynamically when inventory changes.
  - **Invisible SEO:** All SEO configurations are handled by the agent. The user only approves high-level strategies (e.g., creating a new landing page).

  ## Implementation Prompt
  Implement the Edge-Cached Dynamic Storefront rendering pipeline and Agentic SEO capability.
  - **User-Facing Outcome:** When an owner adds a new product or service, the system automatically generates a blazing-fast, SEO-optimized web page distributed globally. The owner receives proactive notifications about SEO performance and suggestions for new content.
  - **CUJ:**
    1. Owner adds a new product via the mobile app.
    2. Marketing Agent generates SEO metadata in the background.
    3. Storefront service pre-renders the page and pushes to the edge cache.
    4. Owner views the live page instantly.
    5. Owner updates inventory (item sold out).
    6. Cache invalidation triggers automatically.
  - **Acceptance Criteria:**
    - Pages achieve a 90+ Lighthouse performance and SEO score.
    - JSON-LD structured data is present and valid for products/services.
    - Cache invalidation works correctly upon product updates.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
