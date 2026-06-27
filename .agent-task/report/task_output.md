issue_title: "Implement Architectural Gap: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and poor SEO rankings. Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO. Existing platforms (Shopify, Wix, Squarespace) offer caching but often require manual configuration, apps, or lack autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.

  ## Research Report
  Our research into the SMB platform landscape reveals a critical gap between enterprise-grade performance and accessible SMB tools.
  - **Shopify:** Offers strong edge network capabilities for fast global delivery, but SEO often requires third-party apps for advanced optimization.
  - **Vercel/Next.js:** The gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users.
  - **Wix/Squarespace:** Provide easier SEO tools, but require manual configuration and lack autonomous scalability.

  By implementing a Universal Edge-Cached Dynamic Storefront combined with Agentic SEO Pre-rendering, OHC can provide enterprise-grade performance and discoverability to non-technical users invisibly.

  ## Design Doc

  ### Architecture Design
  This capability introduces a new layer to the OHC architecture:
  1.  **Edge Cache Layer (e.g., Cloudflare/Vercel Edge):** All public storefront read requests (`GET /store/*`) hit this edge cache first.
  2.  **Agentic Pre-renderer (Operations/Marketing Agent):** When core business entities change (e.g., product added, price updated, inventory sold out), the corresponding Agent triggers a pre-rendering process. This generates static HTML injected with optimal SEO meta tags and structured data.
  3.  **Agentic Cache Invalidation:** The Agent then autonomously invalidates the specific edge cache keys for the affected pages globally.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer/Search Engine Bot] -->|GET /store/product/123| B(Edge Cache Layer)
      B -- Cache Hit --> A
      B -- Cache Miss --> C[OHC Core API / Rust Backend]
      C --> D[(PostgreSQL Central Ledger)]

      E[Owner Action: Update Product] --> F[OHC Internal Events]
      G[Customer Action: Purchase / Inventory Zero] --> F

      F --> H{Operations/Marketing Agent}
      H -->|1. Generate Static HTML w/ SEO| I[Pre-rendering Engine]
      I -->|2. Upload Static Content| J[Object Storage / Edge Nodes]
      H -->|3. Invalidate Cache Key| B
  ```

  ### Mobile UX Flow (375px First)
  - **Owner Experience (Invisible):** The owner never sees settings for "Edge Caching" or "SSR". When they update a product description in the mobile app, they see a simple toast: "Changes saved. Storefront optimized."
  - **Customer Experience:** Instantaneous page loads (< 500ms) even on slow mobile networks, enabling seamless navigation and high conversion rates.

  ### AI Agent Integration Points
  -   **The Promoter (Marketing Agent):** Automatically drafts optimal SEO meta titles, descriptions, and structured JSON-LD data during the pre-rendering phase based on the product's natural language description and images.
  -   **The Manager (Operations Agent):** Hooks into inventory events. If an item sells out, it immediately triggers cache invalidation to reflect the "Sold Out" state globally, preventing double-booking.

  ### Key Design Decisions
  -   **Invisible Autonomy:** Hide all technical complexity. No cache TTL settings or SEO meta fields for the user unless they explicitly enter an "Advanced" mode.
  -   **Event-Driven Invalidation:** Rely on the existing internal event mesh to trigger agents for precise cache invalidation, ensuring data consistency without global cache purges.

  ## Implementation Prompt
  **User-Facing Outcome:** The SMB owner updates their storefront or inventory, and the changes are instantly, globally available with zero latency under heavy traffic. Search engines index their pages rapidly with perfectly structured data, without the owner ever configuring SEO settings.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1.  **Scenario:** Maya updates the price and description of her "Vegan Chocolate Cake".
  2.  **Action:** The backend processes the update and emits an internal event.
  3.  **Agent Action:** The Marketing Agent intercepts the event, generates optimized SEO metadata (title, description, JSON-LD), and triggers the pre-rendering engine.
  4.  **Edge Action:** The Agent invalidates the specific edge cache key for `/store/product/vegan-chocolate-cake`.
  5.  **Verification:** A subsequent `GET` request to that URL returns the newly pre-rendered, SEO-optimized HTML instantly from the edge cache, bypassing the database.
  6.  **E2E Test:** Create a Playwright E2E test that verifies the end-to-end flow: modify a product via the owner API/UI, assert the internal event is fired, and verify the public storefront endpoint returns the updated content with correct SEO meta tags within an acceptable timeframe (simulating edge invalidation).

  **Note to Implementer:** Do not prescribe specific edge providers (e.g., hardcode Cloudflare API). Define abstract interfaces for cache invalidation and pre-rendering that can be implemented by different providers in the future.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
