issue_title: "Implement Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol Report: Edge-Cached Dynamic Storefront & Agentic SEO

  ## Problem Statement
  Small business owners relying on OHC need high-performance storefronts that can withstand viral traffic spikes without manual setup or technical knowledge. Currently, all storefront reads hit the central PostgreSQL database dynamically. This approach incurs high latency during spikes, risks timeouts, increases infrastructure costs, and hurts organic discoverability (SEO) because search engine crawlers struggle with slow, client-rendered content. We need an invisible, zero-touch edge-caching and pre-rendering solution.

  ## Research Report
  Based on the competitor analysis (`[research]_universal_edge_cached_dynamic_storefront_seo.md`), enterprise e-commerce platforms leverage CDNs for storefront delivery. Solutions like Vercel’s Next.js (ISR, edge rendering) exist, but they are inaccessible to non-technical users. Shopify provides edge delivery out of the box but requires apps for deep SEO. The gap OHC must close is a fully automated, agentic pre-rendering and caching pipeline. The "Marketing Agent" and "Operations Agent" must invisibly invalidate cache and pre-render SEO-optimized HTML upon any inventory or catalog changes.

  ## Design Doc
  **Architecture Overview**

  ```mermaid
  graph TD
      A[Buyer / Web Crawler] --> B(Global Edge Cache)
      B --> |Cache Hit| C[Static HTML / SEO Meta Tags]
      B --> |Cache Miss| D(Agentic SEO Pre-renderer Worker)
      D --> E{OHC Job Queue}
      E --> F(Operations/Marketing Agent Mutation)
      F --> |Inventory/Catalog Change| E
  ```

  1.  **Global Edge Cache Layer:** A CDN (e.g., Cloudflare or a simulated edge layer) fronts all storefront read requests (`/storefront/:tenant_id/*`).
  2.  **Agentic SEO Pre-renderer:** A new asynchronous worker (`AgenticSEOWorker`) attached to the `ohc_job_queue`. When triggered, it renders the storefront into static, highly-optimized HTML, injecting dynamic meta tags, JSON-LD structured data for products, and Open Graph tags.
  3.  **Autonomous Invalidation Pipeline:** When the `OperationsAgent` or catalog services mutate a product (e.g., inventory deduction via `CommitInventoryRequest`, price change), an event is dispatched to the `AgenticSEOWorker` to re-render the affected pages and purge the edge cache.

  **Mobile UX Flow (Invisible but critical)**
  - Users (buyers) on 375px viewports experience instant page loads (<50ms TTFB).
  - Business owners (Priya, Maya) manage products normally in the OHC mobile app. When they save changes, a subtle "Storefront updating..." toast appears, indicating the Agent is pre-rendering the changes globally.

  **AI Agent Integration Notes**
  - **Marketing Agent:** Can be prompted to review product descriptions and inject SEO-friendly keywords during the pre-rendering phase.
  - **Operations Agent:** Emits "Inventory Changed" events that trigger cache invalidation and targeted re-rendering (e.g., adding an "Out of Stock" badge to the static HTML).

  ## Implementation Prompt
  Implement the Agentic SEO Pre-rendering pipeline.
  -   **Outcome:** A background worker that listens for catalog/inventory changes, generates SEO-optimized static HTML representations of storefront pages, and updates a caching layer (or database table representing the edge cache).
  -   **CUJ:** Priya updates the price of a "Red Dress" in her OHC app. Behind the scenes, the `AgenticSEOWorker` generates the new HTML for the product page, injects updated JSON-LD schema, and invalidates the old cache. A buyer visiting the storefront URL instantly receives the new, statically-rendered HTML.
  -   **Acceptance Criteria:**
      -   Create a new worker module (e.g., `src/server/workers/seo_prerender_worker.rs`).
      -   Define the data schema for the cached pages.
      -   Hook into existing product update/inventory commit flows to trigger the job.
      -   Include unit tests verifying the generated HTML contains correct meta tags and JSON-LD.

  ## Priority
  `P1` (High - critical for user growth and platform scalability).

  ## Estimated Scope
  Medium.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
