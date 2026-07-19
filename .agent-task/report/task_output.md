issue_title: "Implement Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Small businesses (like Carlos the Handyman or Maya the Home Baker) suffer significantly when their online presence fails during unexpected traffic spikes or ranks poorly on Google due to slow load times and limited SEO optimization. While OHC handles basic hosting, the current architecture lacks an enterprise-grade Edge-Cached dynamic storefront that is instantly accessible worldwide. Furthermore, non-technical owners have zero capability to optimize their sites for search engines, missing out on crucial organic traffic.

  ## Research Report
  - **Performance Gap:** Modern platforms like Shopify and Vercel heavily utilize Edge computing and CDNs to serve storefronts with sub-100ms latency. OHC needs a similar strategy to remain competitive and guarantee performance regardless of the user's technical knowledge.
  - **SEO Complexity:** SEO is opaque for most SMBs. Currently, search engines struggle with client-side rendered Single Page Applications (SPAs).
  - **The OHC Opportunity:** By introducing an "Edge-Cached Dynamic Storefront" combined with "Agentic SEO Pre-rendering," OHC can automatically optimize content delivery globally and ensure every product/service page is perfectly readable by search engine crawlers without any input from the owner.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer / Search Bot] -->|Requests Storefront| B(Global Edge CDN)
      B -->|Cache Hit| A
      B -->|Cache Miss| C[Storefront Pre-rendering Service]
      C -->|Query| D[PostgreSQL Data Layer]
      C -->|Generate HTML| B
      E[Product/Service Update] -->|Event| F[Operations Agent]
      F -->|Invalidate Edge Cache| B
      F -->|Trigger SEO Optimization| G[Marketing Agent]
      G -->|Generate Meta Tags/Schema| D
  ```

  ### System Flow & Agent Interaction
  - **Edge Caching:** Implement a caching layer (e.g., using Cloudflare or a similar CDN) that serves static HTML generated from the dynamic store data.
  - **Agentic SEO:** The Marketing Agent detects new products or services. It automatically generates optimized meta titles, descriptions, and structured data (JSON-LD) which is then injected into the pre-rendered HTML.
  - **Cache Invalidation:** The Operations Agent listens for changes in inventory, pricing, or product details and intelligently invalidates only the necessary edge cache nodes to ensure data freshness without sacrificing performance.

  ### Mobile UX Flow (375px First)
  - **Owner Dashboard:** The owner simply sees a "Performance Health" indicator (e.g., "Site Speed: Excellent, SEO: Optimized") within the main dashboard. They do not need to configure CDN settings or enter SEO tags.
  - **Customer View:** The storefront loads instantaneously on mobile devices, providing a seamless browsing experience even on slower 3G networks.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya adds a new cake to her menu. The OHC platform automatically generates an SEO-optimized page, caches it at the edge for instant global delivery, and ensures search engines can immediately crawl the new offering—all without Maya doing anything beyond uploading the photo and price.

  **CUJ & Acceptance Criteria:**
  1. An owner creates a new product offering via the mobile app.
  2. The Marketing Agent automatically generates SEO metadata (title, description, structured data) based on the product description and image.
  3. The Storefront Pre-rendering Service builds a static HTML version of the new product page.
  4. The page is pushed to the Edge Cache.
  5. The Operations Agent invalidates the main catalog cache to include the new product.
  6. E2E Test: A simulated customer request (from an un-cached region) triggers the pre-render and caches the result, while a subsequent request is served directly from the cache with <100ms latency. A simulated search engine bot successfully retrieves the fully rendered HTML with the injected SEO metadata.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
