issue_title: "[Architecture] Autonomous Omnichannel Pre-Order and Waitlist Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (the baker) and Fatima (the food cart operator) rely on pre-orders and waitlists to manage capacity and cash flow. Managing these across DMs, Instagram, and phone calls is chaotic, leading to dropped orders, overselling, and lost revenue. They need an invisible engine that seamlessly converts interest into secured waitlist spots or paid pre-orders, synced across all channels, without complex configuration.

  ## Research Report
  - **Competitive Audit**:
    - **Shopify/Wix**: Rely on third-party apps for pre-orders, which often clash with inventory systems and add monthly costs. They do not natively support DM-based pre-orders.
    - **Square**: Basic pre-order support for food, but lacks omnichannel integration (e.g., cannot take a pre-order via Instagram DM autonomously).
    - **OHC Advantage**: OHC's Teammate Mesh allows the "Salesperson" and "Operations" agents to coordinate a waitlist or pre-order across the public storefront and social DMs simultaneously, maintaining a single source of truth in the Universal Ledger.
  - **Key Findings**:
    - Pre-orders are essential for capacity-constrained businesses (bakers, food carts) to forecast demand.
    - Non-technical owners struggle to manage the transition from "Waitlist" to "Active Order".

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ WAITLIST_CAMPAIGN : "creates"
      WAITLIST_CAMPAIGN ||--o{ PRE_ORDER_ENTRY : "receives"
      PRE_ORDER_ENTRY }|--|| CUSTOMER360 : "placed by"

      WAITLIST_CAMPAIGN {
          uuid id
          string name "e.g., Thanksgiving Pies"
          int max_capacity
          timestamp drops_at
      }

      PRE_ORDER_ENTRY {
          uuid id
          string status "WAITLIST, SECURED, FULFILLED"
          int deposit_amount
          string source "Storefront, IG_DM"
      }
  ```

  ### Mobile UX Flow (375px)
  - **Dashboard**: A translucent glass card showing active drops (e.g., "Thanksgiving Pies: 40/50 Secured").
  - **1-Tap Publish**: The user types "I'm making 50 pies for Thanksgiving" and the AI agent auto-generates a waitlist page and an Instagram post.
  - **Fulfillment**: On drop day, the Operations agent auto-charges the remaining balance and moves entries to the standard order queue.

  ### AI Agent Integration
  - **The Promoter**: Creates the waitlist landing page and social announcements.
  - **The Ambassador**: Replies to DMs asking "Can I get a pie?" with a direct 1-tap secure pre-order link.
  - **The Manager**: Monitors capacity, stops accepting pre-orders at 50, and triggers the final billing.

  ## Implementation Prompt
  **Goal**: Build the "Autonomous Omnichannel Pre-Order and Waitlist Engine" to allow non-technical owners to launch and manage capacity-constrained drops.
  **CUJ**: Maya wants to sell 50 custom cakes for Mother's Day. She tells the app "Start a waitlist for 50 Mother's Day Cakes at $40 deposit." The app sets up the campaign. A customer DMs her on Instagram "I need a cake!", the Ambassador agent replies with the pre-order link. Once 50 cakes are secured, the waitlist auto-closes.
  **Acceptance Criteria**:
  - Implement `WAITLIST_CAMPAIGN` and `PRE_ORDER_ENTRY` tables with RLS.
  - Integrate with the Teammate Mesh to allow DM-based pre-order securement.
  - Create the 375px mobile UI for campaign monitoring.
  - Ensure capacity locks use Redis/PostgreSQL `SKIP LOCKED` to prevent overselling.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_estimated_scope: Large
