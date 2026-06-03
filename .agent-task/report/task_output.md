issue_title: "[Architecture] Edge-Caching Dynamic Storefronts for High-Traffic Drops"
issue_description: |
  # Architecture Brief: Edge-Caching Dynamic Storefronts for High-Traffic Drops

  ## Title
  Edge-Caching Dynamic Storefronts for High-Traffic Drops & Viral Commerce

  ## Problem Statement
  Small business owners like Priya (Boutique owner) or Maya (Baker) often experience sudden spikes in traffic due to a viral TikTok or an Instagram feature. Current web builder platforms often struggle under sudden load, leading to slow load times or downtime during critical sales windows (e.g., product drops). When Maya announces her limited-edition vegan cake, she needs absolute confidence that her storefront will remain instantly responsive, even if thousands of users click the link simultaneously. The platform must provide edge-cached, highly performant storefronts without requiring the owner to understand CDN configuration or caching strategies.

  ## Research Report
  - **The "Viral Drop" Problem**: Traditional database-driven e-commerce platforms can suffer from slow TTFB (Time to First Byte) under sudden load unless aggressively scaled.
  - **Edge Computing Trends**: Modern architectures (like Vercel, Cloudflare Pages, or Fastly) push pre-rendered static assets and lightweight compute to the edge, resulting in near-instant load times globally.
  - **Competitor Analysis**:
    - *Shopify*: Handles high traffic well but relies on a massive centralized infrastructure. Caching is robust but can be opaque.
    - *Wix/Squarespace*: Can struggle with sudden, massive concurrency for dynamic content unless using their enterprise tiers.
  - **Discovery**: OHC needs a multi-tenant edge-caching strategy. Storefronts should be compiled into static assets (HTML/CSS/JS) and pushed to a global CDN. Dynamic actions (like "Add to Cart" or "Check Inventory") should be handled by lightweight edge functions or background sync queues to prevent the main database from bottlenecking.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT-DASHBOARD ||--o{ STOREFRONT-BUILDER : "Publishes Updates"
      STOREFRONT-BUILDER ||--o{ EDGE-CDN : "Pushes Static Assets"
      CUSTOMER-BROWSER }|--|| EDGE-CDN : "Fetches UI (Cached)"
      CUSTOMER-BROWSER ||--o{ EDGE-COMPUTE : "Dynamic Actions (Add to Cart)"
      EDGE-COMPUTE }|--|| ASYNC-QUEUE : "Queues High-Volume Writes"
      ASYNC-QUEUE ||--o{ OHC-CORE-API : "Processes Safely"
      OHC-CORE-API }|--|| INVENTORY-LEDGER : "Updates State"
  ```

  ### Edge-First Design Principles
  1.  **Statically Generated Core**: The product catalog, home page, and static content are pre-rendered and served from the CDN edge closest to the customer.
  2.  **Stale-While-Revalidate**: Inventory updates (e.g., item sold out) trigger a background revalidation of the edge cache, ensuring the next customer sees the updated state without blocking the current request.
  3.  **Client-Side Hydration for Dynamic State**: The shopping cart and user session are managed client-side and synced via lightweight API calls, separate from the main page load.
  4.  **Queue-Based Checkout**: High-concurrency checkout requests (like a product drop) are placed in a Redis-backed queue. Customers see a fair "waiting room" or immediate optimistic confirmation rather than a crashed database.

  ### Mobile UX Flow (375px First)
  - **Customer View (Viral Click)**: Maya's customer clicks the link in her Instagram bio. The page loads in < 1s (LCP) from the edge cache, displaying the limited-edition cake.
  - **Interaction**: The customer taps "Pre-order". A micro-interaction (glassmorphism spinner) shows while the edge function reserves the inventory slot.
  - **Feedback**: Instant visual confirmation of reservation, transitioning smoothly to the payment flow.

  ### AI Agent Integration Points
  - **Marketing & Advertising Agent**: Automatically monitors social media for viral velocity. If a post is gaining rapid traction, the agent pre-emptively warms the cache in relevant geographic regions.
  - **Operations Agent**: Manages the inventory allocation queue during a drop. If inventory sells out while customers are in the queue, the agent automatically offers them a place on a waitlist or a similar alternative product.

  ## Implementation Prompt
  **To Implementer Agent:**
  Design and implement the multi-tenant edge-caching infrastructure for OHC published storefronts. Ensure that when a business owner updates their catalog via the `StorefrontService`, a background job compiles the relevant pages into static assets and pushes them to the configured CDN/Edge provider. Implement the stale-while-revalidate pattern for inventory counts to ensure high availability during traffic spikes. The frontend must fetch static content from the CDN and handle dynamic state (cart, checkout) via the `CoreAPI`. Validate the architecture by simulating a high-concurrency "product drop" (e.g., 1000 requests/sec) against a test tenant's storefront, ensuring the TTFB remains under 200ms and the core database is not overwhelmed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
