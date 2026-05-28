issue_title: "Autonomous A/B Testing & Conversion Optimization Engine"
issue_description: |
  # [Architecture] Autonomous A/B Testing & Conversion Optimization Engine

  ## Title
  Autonomous A/B Testing & Conversion Optimization Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Maya (baker) do not run A/B tests. The concept of "Conversion Rate Optimization" (CRO) is completely foreign to them. They launch a storefront layout, write a product description once, and leave it forever, missing out on massive revenue opportunities. Traditional CRO tools (like Optimizely or VWO) require significant technical expertise, JavaScript snippet injection, understanding of statistical significance, and manual variant creation.

  Even if they wanted to test, solopreneurs lack the website traffic required to reach traditional statistical significance quickly. They need an invisible, autonomous engine that acts as an elite marketer—automatically generating micro-variations of product copy, adjusting the placement of the "Add to Cart" button, or testing different hero images. This engine must continuously run in the background, measuring conversion events (purchases, bookings) and quietly promoting the winning variants without the owner ever configuring a test or analyzing a funnel.

  ## Research Report
  *   **Shopify / Wix / Squarespace:** None of these platforms offer native, autonomous A/B testing on their base tiers. Shopify requires expensive third-party apps (like Shogun or neat A/B testing apps) that still require the merchant to manually write variant copy and set up the test parameters. Wix provides basic A/B testing, but it's entirely manual.
  *   **Traditional CRO Tools (Optimizely, VWO):** Built for enterprise marketing teams with developers. Fails the "Grandmother Test" completely due to complex setups and jargon (p-values, confidence intervals).
  *   **The OHC Advantage:** OHC’s architecture—featuring multi-tenant edge-caching and the AI Marketing Agent—is perfectly suited to abstract away CRO. By treating the storefront UI as a dynamic, generative output rather than a static file, the Marketing Agent can slightly mutate the generated UI at the edge for different visitor sessions. By hooking into the `Distributed State Machine` for conversion events, OHC can use multi-armed bandit algorithms to autonomously route more traffic to higher-converting UI fragments, solving the "low traffic" problem common to SMBs by continuously optimizing rather than waiting for discrete test conclusions.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      VISITOR_SESSION ||--o{ EDGE_ROUTER : "Requests Storefront"
      EDGE_ROUTER }|--|| VARIANT_CACHE : "Fetches UI"
      EDGE_ROUTER ||--o{ EXPERIMENT_LEDGER : "Logs Exposure"

      CHECKOUT_ENGINE ||--o{ EXPERIMENT_LEDGER : "Logs Conversion (Reward)"

      EXPERIMENT_LEDGER }|--|| MARKETING_AGENT : "Provides Bandit Data"

      MARKETING_AGENT {
          string spiffe_identity "Zero Trust access"
          string tenant_id "Multi-tenant boundary"
      }

      MARKETING_AGENT ||--o{ GENERATIVE_UI_ENGINE : "Mutates Component (e.g. Button Color/Copy)"
      GENERATIVE_UI_ENGINE ||--o{ VARIANT_CACHE : "Pushes New Variants"
  ```

  ```mermaid
  sequenceDiagram
      participant Visitor as Mobile Visitor
      participant Edge as Edge Router (Variant Cache)
      participant Ledger as Experiment Ledger
      participant Agent as Marketing Agent
      participant Cart as Checkout Engine

      Visitor->>Edge: GET /maya-bakery/custom-cake
      Edge->>Edge: Multi-Armed Bandit Selection
      Edge-->>Visitor: Return Variant B ("Book Tasting" button)
      Edge->>Ledger: Log Exposure (Session ID, Variant B)
      Visitor->>Cart: Purchases Custom Cake
      Cart->>Ledger: Log Conversion (Session ID, Variant B, $150)
      Ledger->>Agent: Event Stream Updated
      Agent->>Agent: Re-calculate Bandit Weights
      Agent->>Edge: Update Cache Routing Weights (Favor Variant B)
  ```

  ### Mobile UX Flow (375px)
  *   **Visitor View:** The visitor experiences a lightning-fast, edge-cached mobile site. They have no idea they are in a test. The UI adheres to OHC's Translucent Glass and Unifi card designs.
  *   **Merchant View (Command Center):** The merchant does *not* see an "A/B Testing Dashboard." Instead, they receive a plain-language Activity Feed notification from the Marketing Agent once a week: *"I tested changing your 'Buy Now' button to 'Pre-order Today' and it increased sales by 12%. I have permanently applied this change to your storefront."*
  *   **1-Tap Revert:** The notification includes a simple `[Undo]` button if the owner disliked the change, passing the Grandmother test by hiding all technical complexity.

  ### Performance & Edge Constraints
  *   **Latency Target:** Variant resolution at the edge must occur in < 50ms. We cannot block page rendering to query a central database for test assignments.
  *   **Edge Routing:** The `EDGE_ROUTER` must handle traffic splitting locally using cached bandit weights, falling back to a default variant if the cache is cold.
  *   **Payload Size:** UI variants must be small HTML/JSON component fragments, not entire page reloads, to maintain the Zero-Drop mobile experience.

  ### Zero Trust & Security
  *   **Tenant Isolation:** The `EXPERIMENT_LEDGER` must strictly partition data by `tenant_id`. Exposures and conversions for Maya's bakery cannot bleed into the multi-armed bandit calculations for Priya's boutique.
  *   **Identity:** The `MARKETING_AGENT` must authenticate via SPIFFE/SPIRE when pushing new variants to the `VARIANT_CACHE`.

  ### AI Agent Integration Points
  *   **Marketing Agent:** The core driver. Periodically analyzes the `EXPERIMENT_LEDGER`, calculates multi-armed bandit rewards, mutates UI component schemas (e.g., changing text from "Consultation" to "Quick Chat"), and updates the routing weights at the edge.
  *   **Finance Agent:** Feeds average order value (AOV) data back to the Marketing Agent, ensuring the optimization target isn't just "clicks," but actual revenue.

  ## Implementation Prompt
  **Task:** Build the Autonomous A/B Testing & Conversion Optimization Engine.

  **User Journey (CUJ):**
  1. The AI Marketing Agent autonomously generates a slight variation of a product page's Call-to-Action button text.
  2. The Edge Router splits incoming visitor traffic between the original and the new variant using a multi-armed bandit strategy, logging exposure events.
  3. When a visitor checks out, the Checkout Engine logs a conversion event tied to the exposed variant.
  4. The system autonomously adjusts traffic weights toward the winning variant based on revenue, and the owner receives a simple summary notification in their activity feed.

  **Acceptance Criteria:**
  *   Implement the `EXPERIMENT_LEDGER` to track exposure and conversion events with strict multi-tenant (`tenant_id`) isolation.
  *   Implement the multi-armed bandit weight calculation logic for the Marketing Agent.
  *   Ensure the edge traffic router can serve variants in < 50ms without hitting the primary relational database on every request.
  *   Create the mechanism for the Marketing Agent to push plain-language notifications to the merchant's Activity Feed.
  *   Do NOT prescribe specific database schemas, caching technologies (e.g., Redis vs Memcached), or API routing frameworks. Focus on the secure, performant architectural flow between the edge, the ledger, and the agent.

  ## Priority
  P1

  ## Estimated Scope
  Large

issue_priority: "P1"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
