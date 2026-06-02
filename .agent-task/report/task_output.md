issue_title: "[Architecture] Autonomous Multi-Tenant Loyalty and Rewards Engine"
issue_description: |
  # Research Report: Autonomous Multi-Tenant Loyalty and Rewards Engine

  ## Problem Statement
  Small business owners, such as Priya the Boutique Owner and Fatima the Food Cart Operator, struggle to retain customers and drive repeat business without resorting to heavy discounting. Existing platforms like Shopify or Wix require complex third-party integrations (e.g., Smile.io) to set up loyalty programs. These are often difficult to configure, lack omnichannel synchronization (in-person tap-to-pay vs. online purchases), and require manual management of points and rewards. OHC needs an invisible, AI-managed loyalty engine that automatically tracks customer purchases, accrues points across all sales channels, and proactively engages customers with personalized rewards, all managed effortlessly from a mobile device.

  ## Research Report
  **Market Analysis:**
  - **Shopify:** Relies on third-party apps like Smile.io or Yotpo. Setup requires technical configuration, and omnichannel sync often requires expensive higher-tier plans.
  - **Square:** Offers Square Loyalty, which is well-integrated for in-person POS but less seamless for a unified digital + physical experience across diverse business types.
  - **Wix:** Basic loyalty features available but often require manual intervention and lack proactive AI engagement.

  **Opportunity for OHC:**
  By embedding loyalty natively into the multi-tenant architecture, OHC can leverage the AI Customer Success department ("The Ambassador") to automatically enroll customers, track points seamlessly whether they pay via an online link or in-person Stripe Terminal, and auto-generate personalized reward messages (e.g., "Hi [Name], you're 1 coffee away from a free pastry at Fatima's!").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Mobile App / PWA] --> API[gRPC / REST Gateway]
      API --> LoyaltyService[Loyalty & Rewards Service]
      LoyaltyService --> DB[(PostgreSQL Tenant DB - RLS Enabled)]
      DB --> EventBus[Redis Pub/Sub]
      EventBus --> AIAmbassador[AI Customer Success Agent]
      EventBus --> MarketingAgent[AI Marketing Agent]
      StripeWebhook[Stripe Webhooks] --> API
      StripeTerminal[In-Person POS] --> API
  ```

  ### Mobile UX Flow (375px First)
  1. **Owner View (The Manager):** A simple "Loyalty Program" toggle in the Operations dashboard. Once enabled, AI auto-generates the program rules based on business type (e.g., "Spend $10, get 1 point" or "Buy 9 lessons, get the 10th free").
  2. **Customer View:** After a transaction, a clean, glassmorphic card appears showing points earned and progress towards the next reward. No app download required; progress is tied to their phone number or email.

  ### AI Agent Integration Points
  - **Customer Success ("The Ambassador"):** Monitors customer point balances and sends autonomous WhatsApp/SMS messages when a reward is unlocked or a customer is close to a milestone.
  - **Finance & Payments ("The Accountant"):** Automatically applies earned discounts or free items at checkout, requiring zero manual calculation from the business owner.

  ### Key Design Decisions
  - **Native Omnichannel Sync:** Points are tracked using a unified customer profile, ensuring online purchases and Stripe Terminal in-person transactions accrue together.
  - **Zero-Config Setup:** The AI Advisory department suggests the optimal loyalty structure (points vs. punch-card) based on the business type (e.g., punch-card for Leo the Music Tutor, points for Priya's Boutique).
  - **Row-Level Security (RLS):** Strict isolation in PostgreSQL ensures customer loyalty data is segregated by `tenant_id`.

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the core gRPC endpoints and PostgreSQL schemas for the `LoyaltyService`. The system must support programmatic point accrual based on webhook events from Stripe (both online and Terminal). Create the necessary database tables with `tenant_id` and RLS enabled. Expose a simple API for the Flutter frontend to toggle the loyalty program on/off and for the AI Customer Success agent to query point balances. Ensure all queries are scoped by tenant.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
