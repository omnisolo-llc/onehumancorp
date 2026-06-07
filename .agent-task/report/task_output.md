issue_title: "Implement Edge-Cached Storefront Store SEO & Multi-Channel Inventory Sync"
issue_description: |
  ## Issue Brief: Edge-Cached Storefront Store SEO & Multi-Channel Inventory Sync

  ## 1. Problem Statement
  Small business owners like Priya (Boutique Owner) and Maya (Home Baker) need their storefronts to load instantly, rank well on Google, and seamlessly sync physical and online inventory. Currently, legacy platforms fail on two fronts:
  - **Performance/SEO:** Storefronts are often dynamically rendered, causing slow load times during traffic spikes and poor search engine indexability (SEO penalties). Small business owners cannot configure complex CDNs or ISR/SSG systems.
  - **Inventory Sync:** Online carts and in-person Point-of-Sale (POS) tap-to-pay are out of sync. This leads to double-booking and out-of-stock scenarios, requiring Priya to manually reconcile offline sales or use costly third-party integrations.

  ## 2. Research Report
  Our competitive analysis (Shopify, Wix, Squarespace, Vercel/Next.js) shows that:
  - **SEO & Caching:** While Vercel/Next.js provides excellent Edge ISR (Incremental Static Regeneration), it's inaccessible to non-technical SMBs. Shopify provides CDN caching but complex SEO optimization often requires apps.
  - **Inventory POS Sync:** Competitors like Square offer strong POS but lack integrated agentic workflows. Shopify POS is robust but requires higher-tier plans for full sync.
  - **OHC Differentiator:** OHC must provide "Universal Edge Caching" and "Agentic SEO Pre-rendering" completely invisibly. Additionally, we need a "Distributed Locks (Redis Redlock)" and local-first POS offline sync architecture to seamlessly merge online and offline inventory counts.

  ## 3. Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Customer/Web] -->|Reads| B[Cloudflare / Edge Cache]
      B -- Cache Miss --> C[OHC API Server]
      C --> D[(PostgreSQL Central Ledger)]

      E[Priya / Mobile POS] -->|Offline Sync Mutation| C
      C -->|Update| D
      C -->|Invalidate Storefront Cache| B

      F[Operations Agent] -->|Listen for Inventory Updates| C
      F -->|Trigger SEO Pre-rendering| B
  ```

  ### Mobile UX Flow (375px)
  - **Storefront View:** A fast-loading, cached page showing available inventory.
  - **POS View:** Priya's mobile app (375px) processes a tap-to-pay offline. The app queues the transaction. Upon network restore, it sends an `OfflineSyncRequest` to the backend. The backend updates PostgreSQL, enqueues an `offline_pos_sync` job, and instantly invalidates the edge cache tags for the specific tenant and product. This prevents online customers from double-booking.

  ### AI Agent Integration
  - **The Operations Agent:** Monitors the background `offline_pos_sync` jobs and cache invalidations. If an item sells out due to an offline sync, it can trigger push notifications to Priya and suggest drafting a restock order.
  - **The Marketing Agent:** When inventory changes or new products are added, the agent triggers SEO pre-rendering to update meta tags and push the static HTML to the edge cache.

  ## 4. Implementation Prompt
  **Target Persona:** Priya (Boutique Owner)
  **Feature Name:** Edge-Cached Storefront & Multi-Channel Inventory Sync

  **Outcome:** Priya can sell the last "Red Dress" in-store using her mobile POS. The app syncs offline, instantly updating the central ledger and invalidating the storefront's edge cache. Online customers immediately see the item as "Sold Out", preventing double-booking. Search engines always index a fast, cached version of the storefront.

  **Critical User Journey (CUJ):**
  1. Priya's boutique has 1 "Red Dress" left.
  2. A customer buys it in-store using Priya's 375px mobile POS view.
  3. The mobile app sends an `OfflineMutation` to the backend.
  4. The backend deducts the inventory in PostgreSQL and enqueues the `offline_pos_sync` job.
  5. The backend instantly invalidates the edge cache tags for Priya's tenant and the "Red Dress" product.
  6. A new online customer visits the storefront; the cache miss triggers a fetch from the DB, showing "Sold Out", and caching the new state.

  **Acceptance Criteria:**
  - Build the edge caching invalidation logic within the `offline_sync` pipeline.
  - Design the `offline_pos_sync` background worker to process finalized offline POS transactions.
  - Ensure the mobile POS UX for the sync status is clear and touch-friendly (≥ 44x44px).
  - Add E2E tests verifying that an offline sync mutation correctly updates inventory and prevents subsequent online cart checkouts.

  ## 5. Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, performance, inventory]
assignees: []
