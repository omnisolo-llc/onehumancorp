issue_title: "Implement Autonomous Customer Lifecycle & Loyalty Engine"
issue_description: |
  ## Problem Statement
  Small business owners such as Maya (home baker) and Priya (boutique operator) are overwhelmed by manual CRM tasks. Currently, if a customer makes an online purchase, sends an Instagram DM, and taps-to-pay in person, they are logged as three separate identities. Setting up complex logic for loyalty rewards (e.g., Klaviyo workflows or Shopify segments) is too technical and time-consuming. OHC requires an invisible, autonomous loyalty and CRM engine that stitches cross-channel interactions into a unified identity and manages rewards automatically.

  ## Research Report
  - **Shopify & Klaviyo**: Highly capable but requires substantial setup (30-60 minutes), manual audience segmentation, and managing overlapping discount codes.
  - **Wix Bookings & Loyalty**: Basic manual loyalty programs, but relies heavily on owner-initiated campaigns. No native wallet integration or autonomous point tracking across physical/digital.
  - **Square**: Simple POS loyalty (phone number at checkout) but disconnected from online e-commerce behavior and lacks proactive agentic outreach for churn prevention.
  - **The OHC Opportunity**: We must implement an event-sourced `LoyaltyLedger` and a unified `Customer360` profile that handles data deduplication and reward tracking implicitly in the background.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER_360 ||--o{ INTERACTION_TIMELINE : "has many"
      CUSTOMER_360 ||--o{ LOYALTY_LEDGER : "has many"
      CUSTOMER_360 {
          uuid customer_id PK
          uuid tenant_id FK
          string universal_identifier
          int total_ltv_cents
          string status
      }
      INTERACTION_TIMELINE {
          uuid event_id PK
          uuid customer_id FK
          string channel
          string intent
          timestamp occurred_at
      }
      LOYALTY_LEDGER {
          uuid ledger_id PK
          uuid customer_id FK
          int points_delta
          string reason
          timestamp created_at
      }
      MARKETING_AGENT }|--|| CUSTOMER_360 : "monitors & enrolls"
      OPERATIONS_AGENT }|--|| LOYALTY_LEDGER : "adjusts on purchase"
  ```

  ### Mobile UX Flow (375px)
  1. **Zero-Touch Redemption**: At checkout (POS or online), the 375px UI displays a translucent glassmorphism banner: "You have a free cake available! [Tap to Apply]".
  2. **Push Notification Approval**: The Marketing Agent pushes a notification to the owner: "Sarah just reached VIP status. I've drafted a thank you note and an exclusive 15% discount. [Approve & Send] / [Edit]".

  ### AI Agent Integration
  - **The Manager (Operations)**: Listens to order fulfillment events and automatically appends point additions to the `LoyaltyLedger`.
  - **The Ambassador (Customer Success)**: Uses the `InteractionTimeline` as context for answering queries (e.g., "Yes Sarah, you can use your 50 points on this order!").
  - **The Promoter (Marketing)**: Automatically segments customers by LTV and drafts win-back campaigns for slipping customers.

  ## Implementation Prompt
  - Build the `Customer360` profile and event-sourced `LoyaltyLedger` tables in PostgreSQL with strict Row Level Security by `tenant_id`.
  - Expose a gRPC/REST endpoint for the POS client to fetch a customer's available loyalty balance seamlessly during a transaction.
  - Implement a mobile-first (375px) glassmorphism card for the OHC app that surfaces pending agentic loyalty actions for the owner to 1-tap approve.
  - Integrate a background worker that listens for `pos.payment_success` and `ecommerce.order_fulfilled` to update the `LoyaltyLedger` without manual owner input.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
