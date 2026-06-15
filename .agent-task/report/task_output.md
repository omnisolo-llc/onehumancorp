issue_title: "Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  **Problem Statement:**
  Small business owners need their storefronts to load instantly and rank highly on search engines to acquire customers. However, dynamic features typically require round-trips to centralized origin servers, adding significant latency and increasing cart abandonment rates. Traditional SEO practices are too complex for non-technical users and often fail when relying heavily on client-side rendering.

  **Research Report:**
  The gap between high-performance enterprise e-commerce and accessible SMB platforms is significant. Competitors like Shopify offer strong edge network capabilities but require third-party apps for advanced SEO. Vercel/Next.js offer developer tools requiring substantial configuration. Wix/Squarespace rely heavily on basic CDN caching for static assets. OHC must deliver "instant" loading (sub-100ms Time to First Byte) globally while maintaining 100% dynamic capabilities. Furthermore, "The Promoter" AI agent must autonomously manage SEO without the user ever touching a meta tag.

  **Design Doc:**
  - **Architecture:** The architecture involves Universal Edge Caching (e.g., Cloudflare Workers), Edge KV Store for dynamic data (inventory/pricing), and a Central Origin (Go API Gateway, Storefront Service, PostgreSQL Read Replica).
  - **Agentic SEO Pre-rendering:** "The Promoter" (Marketing Agent) autonomously pre-renders fully optimized, static HTML shells for all storefront pages, injecting precise meta tags, OpenGraph data, and structured JSON-LD schemas. These static shells are pushed to the Edge CDN.
  - **Edge KV Hydration:** The Edge CDN serves the pre-rendered HTML instantly. Crucial dynamic data is stored in a globally distributed Edge Key-Value (KV) store. An Edge Worker intercepts the HTML response and injects the live KV data before sending it to the client.
  - **Mobile UX Flow:** The pre-rendered HTML shell and edge-injected dynamic data ensure Zero Cumulative Layout Shift (CLS) on 375px screens. Heavy interactive components hydrate progressively.
  - **AI Agent Integration:** The Marketing Agent generates SEO metadata. The Operations Agent triggers cache invalidations upon state changes.

  **Implementation Prompt:**
  - Implement "The Promoter" agent's logic to generate structured SEO metadata (JSON-LD) based on a tenant's product catalog.
  - Design the Edge KV schema (e.g., `tenant:{id}:product:{id}:inventory`) to store real-time availability.
  - Create a pre-rendering service that generates static HTML shells and pushes them to the edge cache.
  - Write E2E tests (Playwright) that verify a simulated storefront loads instantly from a mock cache and correctly displays dynamic "Sold Out" states fetched from a mock Edge KV store.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
