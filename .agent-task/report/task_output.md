issue_title: "[Architecture] Cross-Channel Interaction Timeline & Event-Driven Loyalty Mesh"
issue_description: |
  # Cross-Channel Interaction Timeline & Event-Driven Loyalty Mesh

  ## Problem Statement
  Small business owners suffer from hopelessly fragmented customer data. When a customer DMs Maya on Instagram to ask about a cake, buys a smaller item later via her web storefront, and finally visits her pop-up shop to tap their card for an in-person purchase, they appear as three entirely separate individuals in Maya's system. To offer a loyalty discount or personalize her communication, Maya would need to manually piece together these interactions. For an owner handling everything on their phone, this manual CRM merging is impossible. OHC must invisibly stitch these fragmented interactions into a single, cohesive customer profile.

  ## Research Report
  - **Shopify/Wix**: Rely on third-party apps (e.g., Smile.io, Yotpo) which add "Cost Creep" and setup complexity.
  - **Klaviyo**: Powerful but requires technical knowledge of data flows and segmentation.
  - **OHC Advantage**: By integrating the loyalty engine directly into the KAIROS Teammate Mesh, OHC can treat customer retention as an autonomous background process rather than a manual marketing task. Our event-driven architecture allows real-time inference of customer "Mood" to drive proactive engagement.

  ## Design Doc
  ### Data Model (Customer360, Interaction Timeline & Loyalty)
  We move beyond simple "Customer" records to a `Customer360` profile that unifies interactions across all departments.

  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER360 : "owns"
      CUSTOMER360 ||--o{ INTERACTION_TIMELINE : "recorded in"
      CUSTOMER360 ||--o{ LOYALTY_LEDGER : "accrues"
      CUSTOMER360 ||--o{ SUBSCRIPTION_STATE : "manages"

      CUSTOMER360 {
          uuid id
          string email
          string phone
          string mood "AI-inferred (Loyal, At-Risk, Inactive)"
          jsonb preferences "e.g., contact method"
      }

      INTERACTION_TIMELINE {
          uuid id
          string source "Order, DM, Booking, Support"
          string sentiment "Positive, Neutral, Negative"
          timestamp occurred_at
      }

      LOYALTY_LEDGER {
          uuid id
          int points_balance
          string tier_name
          timestamp last_updated
      }
  ```

  ### Key Architectural Invariants
  1. **Zero-Jargon Segmentation**: No "Segments" or "Lists". The system uses AI-inferred "Moods" (e.g., "Needs Attention", "VIP") to trigger actions.
  2. **Multi-Tenant Isolation**: Customer data and private interaction sentiment are strictly isolated via PostgreSQL RLS at the `tenant_id` level.
  3. **Event-Driven Loyalty**: Points and rewards are not "calculated" on view; they are event-sourced and recorded in the `LOYALTY_LEDGER` to ensure real-time accuracy across mobile and web.

  ### Mobile UX Flow
  1. **Dashboard: Customer Pulse Card**: A translucent glass card showing "3 VIPs" and "2 At-Risk" customers. Tapping the card opens the "Customer Interaction Timeline" with a smooth spring animation.
  2. **The "1-Tap Retention" Flow**: "Ambassador drafted a 'Miss You' reply for Leo 🎸". A 375px wide bottom sheet with a blurred background shows the drafted message and a large "Approve & Send" button in OHC Primary Green.
  3. **Customer Interaction Timeline**: A vertical, non-jargon timeline showing "Order Placed", "Inquiry Answered", "Sentiment: Happy 🌟". Instead of "LTV: $540.23", the UI says "Top 5% Spender".

  ## Implementation Prompt
  Build the "Autonomous Customer Lifecycle & Loyalty Engine" to eliminate "Retention Friction" for non-technical small business owners.
  1. **Lifecycle Logic**: Implement the backend service that unifies orders, DMs, and bookings into a single `Customer360` view with a full `INTERACTION_TIMELINE`.
  2. **Mood Transitions**: Enable AI-inferred "Mood" transitions based on event frequency and sentiment.
  3. **Event-Driven Loyalty**: Ensure `OrderCompleted` events trigger immediate, multi-tenant-isolated updates to the `LOYALTY_LEDGER`.
  4. **1-Tap Approval Integration**: Actionable drafts must appear in the mobile Activity Feed with clear "Approve" or "Edit" paths.
  5. **Zero-Jargon UI**: The implementation must strictly avoid technical terms like "LTV," "Churn," or "Retention Rates," using plain human language (e.g., "Frequent Buyer," "Needs Attention").

  **Priority**: P1 (High)
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
