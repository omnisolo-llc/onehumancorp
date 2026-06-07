issue_title: "Implement Edge-Cached Dynamic Storefront for Optimal SEO and Loading Speed"
issue_description: |
  # Research Report: Edge-Cached Dynamic Storefront for Optimal SEO and Speed

  ## Problem Statement
  Small business owners rely heavily on organic search visibility (SEO) and low-friction discovery to acquire customers. Currently, monolithic platforms or entirely client-side rendered apps suffer from slow Initial Contentful Paint (ICP) times, harming mobile conversion rates. Business personas like Maya (baker) and Priya (boutique operator) need their storefronts to load instantly worldwide, even on spotty 3G/4G connections, and rank highly in Google without understanding technical SEO.

  ## Research Report
  - **Market Landscape**:
    - *Shopify*: Uses heavily cached Liquid templates, yielding fast server responses but sometimes bloated client-side JS payloads.
    - *Wix/Squarespace*: Provide built-in SEO tools but their heavy monolithic architecture can result in suboptimal Core Web Vitals.
    - *Modern headless (Vercel/Next.js)*: Achieves near-instant edge delivery (via CDN and Edge Functions) and perfect SEO via Server-Side Rendering (SSR) and Static Site Generation (SSG).
  - **The OHC Opportunity**:
    - OHC needs to offer an "Edge-Cached Dynamic Storefront" out-of-the-box. This means utilizing SSR/SSG coupled with a globally distributed CDN to cache HTML/JSON near the user. We need to decouple the slow core data paths from public storefront reads. This guarantees ultra-fast load times globally and perfect SEO, giving OHC merchants a technical competitive advantage out of the box.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Customer Mobile/Web Client]
      CDN[Global Edge CDN / Vercel Edge]
      Cache[Redis Storefront Edge Cache]
      OHCBackend[OHC Go/Rust API Gateway]
      DB[Central PostgreSQL DB]

      Client -->|Request Page| CDN
      CDN -->|Hit| Client
      CDN -->|Miss| Cache
      Cache -->|Hit| CDN
      Cache -->|Miss| OHCBackend
      OHCBackend -->|Fetch Data| DB
      OHCBackend -->|Generate HTML/JSON| Cache
  ```

  ### Core Components
  1.  **Edge Caching Layer**: Utilize an edge CDN (or Redis cache for local/standalone deployments) to serve pre-rendered storefront pages.
  2.  **Server-Side Rendering (SSR)**: The web layer must support SSR to ensure search engine crawlers see fully rendered HTML content immediately.
  3.  **Cache Invalidation Strategy**: When an owner updates a product, inventory, or setting via the OHC app, a webhook/event triggers cache invalidation for the affected routes to ensure consistency.

  ### Mobile UX Flow
  - The customer hits the link-in-bio or Google search result.
  - The page loads instantaneously (< 1.5s LCP) on mobile (375px viewport).
  - Images are delivered in WebP format and lazy-loaded if below the fold.

  ### AI Agent Integration
  - **The Marketing Agent**: Automatically analyzes page load times and Core Web Vitals, generating a plain-language health report for the owner. It also suggests metadata optimizations (Title, Description) that are automatically injected into the SSR output.

  ## Implementation Prompt
  **Feature Name**: Edge-Cached Dynamic Storefront
  **Target Persona**: Maya (Baker) / Priya (Boutique)
  **Outcome**: Customers can access the public storefront instantly worldwide, improving SEO and mobile conversion rates. The owner never configures caching; it just works.

  **Next Actions**:
  1. Design the Next.js/React SSR architecture for the public storefront routes to ensure SEO indexing.
  2. Implement an Edge-Caching/Redis layer to serve these routes.
  3. Create the event-driven cache invalidation logic triggered when a product or store setting is updated in the central PostgreSQL database.
  4. Ensure all public pages achieve a >90 Mobile PageSpeed Insights score.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
