issue_title: "[Research] Universal Edge-Cached Dynamic Storefront & Agentic SEO Architecture"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Architecture

  ## Track 1: Architectural Gap & Scaling Discovery

  Small business owners using OneHumanCorp (OHC) need their storefronts to load instantly for customers worldwide and rank highly on search engines (Google, Bing). However, because OHC storefronts are highly dynamic (displaying real-time inventory, booking availability, and personalized AI-driven content), traditional static site generation (SSG) is insufficient. Search engine crawlers struggle with complex, client-side rendered (CSR) JavaScript applications, leading to poor SEO performance.

  **Competitor Solutions:**
  - **Shopify:** Uses server-side rendering (SSR) and edge caching (Oxygen), requiring complex Liquid templates or Hydrogen (React) frameworks that are too advanced for zero-tech users.
  - **Wix/Squarespace:** Provide decent SSR and SEO tools but require manual configuration of meta tags and structured data, which our users often neglect.
  - **Vercel/Next.js (ISR):** Incremental Static Regeneration is powerful, but managing cache invalidation globally across a multi-tenant SaaS for millions of permutations is error-prone.

  **Identified Gap:**
  OHC lacks an architecture that combines the speed of edge-cached static sites with the freshness of dynamic data, fully optimized for search engines via AI. We need **Agentic SEO Pre-rendering** to automate this without user intervention.

  ## Track 2: Selected Architecture Deep Dive

  ### Architecture Design (Mermaid)

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

  ### Implementation Strategy

  1. **Agentic SEO Optimizer (Marketing Agent):**
     - Autonomously generates `<title>`, `<meta description>`, and Open Graph tags based on the storefront's content.
     - Generates Schema.org JSON-LD (e.g., `Product`, `LocalBusiness`, `Service`) to ensure search engines understand the offerings.

  2. **Edge-Cached Pre-rendering Engine:**
     - On cache miss, the Go backend fully renders the HTML (SSR) including the AI-generated SEO metadata, tailored for a 375px mobile-first viewport.
     - The output is cached at the edge (CDN) for immediate delivery.

  3. **Intelligent Cache Invalidation (Operations Agent):**
     - Targeted cache purges triggered by state changes (e.g., inventory drops to zero, price updates).
     - The Ops Agent fires an event to the Cache Invalidator to purge specific URLs from the Edge KV.

  ## Track 3: Technical Integrity & Mobile-First Review

  - **Mobile-First UX Flow:** The pre-rendering engine must prioritize the 375px viewport layout in the generated HTML.
  - **Performance Targets:** LCP < 1.5s, FID < 100ms, CLS 0, Lighthouse SEO Score 100/100 without user configuration.
  - **Zero Trust & Security:** Multi-tenant isolation is maintained during SSR by enforcing strict row-level security (RLS) policies in PostgreSQL based on the requested tenant domain.

  ## Implementation Prompt (For Implementer Agent)

  **Objective:** Implement the backend foundation for Agentic SEO Pre-rendering and Intelligent Cache Invalidation.

  **Expected Outcome:**
  - A pre-rendering module in the Go backend that generates HTML with injected SEO metadata (meta tags, JSON-LD) provided by a simulated Marketing Agent.
  - An event-driven cache invalidation pipeline where state changes trigger invalidation events.
  - Integration with the existing multi-tenant architecture.

  **Acceptance Criteria:**
  - When a storefront route is requested, the server returns fully formed HTML containing appropriate `<title>`, `<meta>`, and `<script type="application/ld+json">` tags.
  - The cache invalidation module correctly receives events and queues purge requests.
  - 100% unit test coverage for the new modules.
  - (Optional for this phase) Playwright E2E test verifying the presence of SEO tags on a seeded storefront page.

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
