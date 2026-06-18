issue_title: "Distributed Edge-Cached Storefront Dynamic Pricing Engine"
issue_description: |
  # Mission Queue Protocol Report

  ## Title
  Distributed Edge-Cached Storefront Dynamic Pricing Engine

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Carlos (field service owner) often need to adjust prices dynamically based on demand, inventory levels, or localized events (e.g., weekend sales, flash pop-up events). However, legacy e-commerce platforms like Shopify and Wix rely heavily on static pricing unless expensive third-party apps are used. This makes it difficult for owners to implement yield management or time-based discounts (like happy hour pricing for Fatima's food cart).

  When using an edge-cached dynamic storefront, prices must be updated globally without breaking the cache or requiring the owner to manually update hundreds of SKUs. The OHC platform lacks a centralized, multi-tenant Dynamic Pricing Engine that coordinates with the Operations and Sales Agents to automatically adjust prices based on real-time signals, while ensuring these changes reflect instantly on the localized storefront without a full cache invalidation.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Rely on manual price adjustments or third-party apps for dynamic pricing. Heavy reliance on full page caching makes real-time price changes difficult without custom storefronts.
  - **Airlines/Hotels (Industry Benchmark):** Use sophisticated yield management systems. These are too complex for SMBs.
  - **OHC Opportunity:** Implement an "invisible" yield management system. The AI Operations Agent monitors inventory velocity and competitor signals, suggesting pricing adjustments to the owner. Once approved, the Dynamic Pricing Engine pushes these updates using edge-computed fragments or localized pricing APIs, ensuring the main storefront remains cached and fast.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Operations Agent: The Manager] -->|Analyzes Inventory Velocity| B(Dynamic Pricing Engine)
      B -->|Suggests Adjustment| C[Action Required Queue: Owner Feed]
      C -->|Owner Approves 1-Tap| B
      B -->|Updates Price Ledger| D[(Cloud Postgres: Pricing Rules)]
      B -->|Invalidates Price Fragment Cache| E[Edge Network / CDN]
      F[Customer browsing Storefront] -->|Fetches Page| E
      E -->|Serves Static UI + Dynamic Price API| F
  ```

  ### Mobile UX Flow
  - **Home Feed (Mobile):** Top card shows "Yield Opportunity: Red Dresses selling 3x faster than normal."
  - **Interaction:** Tapping the card opens a glassmorphism detailed view. The Operations Agent suggests a 10% price increase for the weekend to maximize revenue.
  - **Action:** Primary button "Approve Price Increase", secondary "Ignore".
  - **Visual Design:** Translucent Glass materials, clear visual indicators of the current vs. proposed price, and an interactive sparkline showing sales velocity.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Continuously monitors inventory and sales velocity against historical baselines. Triggers the Dynamic Pricing Engine when thresholds are met.
  - **Sales Agent:** When quoting services (like Carlos's repair service), it can apply dynamic pricing rules based on current workload and availability (e.g., higher price for same-day emergency repairs).

  ### Key Design Decisions
  - **Edge-Computed Fragments:** The main storefront remains statically generated and edge-cached for maximum SEO and performance. Prices are fetched asynchronously via a fast, localized Pricing API or injected at the edge using Workers/Functions, ensuring changes are instant.
  - **Multi-Tenant Isolation:** Pricing rules are strictly isolated per `tenant_id` within the PostgreSQL database.
  - **Owner Control:** The AI never changes prices autonomously without owner approval unless the owner specifically enables "Auto-Pilot" mode for yield management.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, I receive a proactive notification suggesting I increase the price of an item that is selling unusually fast. I tap "Approve," and the price instantly updates on my edge-cached storefront without needing to re-deploy or manually edit the product.
  **CUJ & Acceptance Criteria:**
  1. The Operations Agent detects a high sales velocity for a specific product.
  2. The Dynamic Pricing Engine generates a proposed pricing rule and places it in the `ActionRequiredQueue`.
  3. The owner logs into the OHC mobile app, sees the suggestion card, and taps "Approve."
  4. The pricing rule is saved to the PostgreSQL database under the tenant's isolation boundary.
  5. The edge cache for the pricing fragment is invalidated or updated via the localized Pricing API.
  6. Provide Playwright E2E tests: A user logs in, approves the dynamic pricing suggestion, and verifies that the storefront displays the new price correctly without a full page reload.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
