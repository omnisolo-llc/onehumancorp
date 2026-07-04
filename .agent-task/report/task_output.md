issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  # Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture

  ## Problem Statement
  Small business owners using OneHumanCorp (OHC) need their storefronts to load instantly for customers worldwide and rank highly on search engines (Google, Bing). However, because OHC storefronts are highly dynamic—displaying real-time inventory, booking availability, and personalized AI-driven content—traditional static site generation (SSG) is insufficient. Furthermore, search engine crawlers struggle with complex, client-side rendered JavaScript applications, leading to poor SEO performance for our users. We need an architecture that combines the speed of edge-cached static sites with the freshness of dynamic data, fully optimized for search engines via AI.

  ## Target Personas
  - **Fatima (Food Cart):** Customers scanning a QR code or clicking a link in her bio need the menu to load in under 1 second on poor 3G connections. The "sold out" status must be real-time.
  - **Leo (Music Tutor):** Prospective students searching for "guitar lessons near me" on Google need to find his OHC-hosted profile page.
  - **Priya (Boutique):** Needs her product pages to show up in Google Shopping results with accurate metadata and rich snippets.

  ## Research Report
  ### Competitive Analysis
  - **Shopify:** Uses a mix of server-side rendering (SSR) and edge caching (Oxygen), but requires complex Liquid templates or Hydrogen (React) frameworks that are too complex for our zero-tech users.
  - **Wix/Squarespace:** Provide decent SSR and SEO tools, but require manual configuration of meta tags and structured data, which our users won't do.
  - **Vercel/Next.js (ISR):** Incremental Static Regeneration is powerful, but managing cache invalidation globally across a multi-tenant SaaS for millions of permutations is error-prone.

  ### OHC's Differentiation
  OHC will use **Agentic SEO Pre-rendering**. The "Marketing & Advertising" AI Agent will automatically generate optimized HTML snapshots and structured data (JSON-LD), cache them at the CDN edge, and intelligently invalidate them when the "Operations" Agent detects a relevant state change (e.g., inventory drops to zero).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Edge "CDN (Cloudflare / Fastly)"
          Worker[Edge Worker]
          KV[(Edge KV Cache)]
      end

      subgraph Backend "Go + Bazel Backend"
          API[API Gateway]
          PreRenderEngine[Pre-rendering Engine]
          CacheInvalidator[Cache Invalidator]
      end

      subgraph AI "AI Agent Departments"
          Marketing[Marketing Agent: SEO Optimizer]
          Ops[Operations Agent: State Monitor]
      end

      subgraph Storage
          DB[(PostgreSQL - Tenant DB)]
      end

      Client[Customer Browser / Crawler] --> Worker
      Worker -- Cache Hit --> Client
      Worker -- Cache Miss --> API

      API --> PreRenderEngine
      PreRenderEngine --> DB
      PreRenderEngine --> Marketing
      Marketing -- Generates Meta/JSON-LD --> PreRenderEngine

      PreRenderEngine --> Worker : Returns HTML + Caches in KV

      Ops -- Detects Inventory/Booking Change --> CacheInvalidator
      CacheInvalidator --> Worker : Purge specific KV keys
  ```

  ### Core Components
  1. **Agentic SEO Optimizer (Marketing Agent):** Automatically generates `<title>`, `<meta description>`, and Open Graph tags based on the storefront's content. Generates Schema.org JSON-LD (e.g., `Product`, `LocalBusiness`, `Service`) ensuring search engines understand the offerings.
  2. **Edge-Cached Pre-rendering Engine:** When a crawler (Googlebot) or user requests a page, the Edge Worker checks the KV cache. On a miss, the Rust backend fully renders the HTML (SSR) including the AI-generated SEO metadata, tailored for a 375px mobile-first viewport. The result is cached at the edge (CDN) for immediate delivery to subsequent visitors.
  3. **Intelligent Cache Invalidation (Operations Agent):** Instead of time-based expiry (TTL), the Operations Agent triggers targeted cache purges. For example, if an inventory item sells out, the Ops Agent fires an event to the Cache Invalidator to purge only that specific product's URL from the Edge KV.

  ### Mobile UX Flow
  1. A user taps a product link on their phone (375px width).
  2. The CDN Edge Worker receives the request and immediately serves the cached SSR HTML page in under 1 second.
  3. The page contains all necessary JSON-LD structured data and AI-generated SEO tags.
  4. Once loaded, client-side hydration connects the user to live operations (e.g., real-time cart functionality).

  ### AI Agent Integration Points
  - **Marketing Agent:** Triggered during the SSR pre-rendering phase on a cache miss to fetch/generate dynamic SEO tags and structured data.
  - **Operations Agent:** Monitors inventory and bookings. Upon state change, it triggers a cache invalidation event for the affected URLs.

  ## Scope
  - **Estimated Scope**: Large

  ## Implementation Prompt
  Implement the backend Pre-rendering Engine in Go to fully render HTML for public storefront routes on a cache miss. Integrate the Marketing Agent to inject dynamic JSON-LD and meta tags during this SSR phase. Ensure the page layout is optimized for a 375px mobile viewport. Additionally, implement an event listener for the Operations Agent that triggers a targeted cache invalidation when critical state changes (like inventory dropping to zero) occur. Provide E2E Playwright tests that simulate a customer request (cache miss and cache hit) and verify the presence of SEO tags, as well as an inventory update triggering a cache purge.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
