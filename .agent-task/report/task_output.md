issue_title: "Build Subscription Ledger & AI-Native Recurring Revenue Engine"
issue_description: |
  # Research Report: AI-Native Subscription & Recurring Revenue Engine

  ## Problem Statement
  Small business owners and creators (like Leo the music tutor or Maya the baker) need a way to build predictable recurring revenue (subscriptions, retainers, memberships) without having to configure complex billing software like Chargebee or piece together expensive third-party Shopify apps (like ReCharge). Currently, managing failed payments, pausing subscriptions, or upgrading plans requires the owner to navigate technical dashboards or deal directly with Stripe.

  ## Research Report
  - **Competitor Analysis:**
    - **Shopify:** Requires third-party apps (ReCharge, Skio) which cost hundreds of dollars monthly, break native checkout flows, and introduce significant setup complexity.
    - **Wix/Squarespace:** Offer basic recurring payments, but lack intelligent lifecycle management (e.g., AI-driven recovery, natural language pauses/swaps).
    - **Stripe Billing:** Powerful API but highly technical. The dashboard is not designed for non-technical SMB operators on mobile devices.
  - **OHC Opportunity:** By natively integrating a `SubscriptionLedger` directly into our data model and hooking it into our agentic framework, OHC can make subscriptions a "zero-configuration" feature. The AI agents will handle dunning, paused states, and plan upgrades autonomously via natural language.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Natural Language Request] --> B(Customer Success Agent)
      B --> C{Intent Analysis}
      C -->|Pause/Cancel/Upgrade| D[Subscription Manager Agent]
      C -->|Billing Question| E[Finance Agent]
      D --> F[(OHC Postgres: Subscription Ledger)]
      D --> G[Stripe Billing API]
      F --> H[Operations Agent: Fulfillment Queue]
      G --> F
  ```

  ### Data Model & Invariants
  - **`subscriptions` table:** `id`, `tenant_id` (RLS), `customer_id`, `product_id`, `stripe_subscription_id`, `status` (active, past_due, canceled, paused), `current_period_end`.
  - **`subscription_events` table:** `id`, `tenant_id` (RLS), `subscription_id`, `event_type` (created, renewed, failed, paused), `agent_id` (if triggered by AI).
  - **Invariants:**
    - Strict Multi-tenant isolation using `tenant_id` and RLS on all subscription-related tables.
    - All state mutations must be synced bidirectionally with Stripe via webhooks to prevent ledger drift.

  ### Mobile UX Flow (375px)
  1. **Owner View:** "Revenue" tab shows a simple "Active Subscriptions" card with MRR. Tapping reveals a list of subscribers.
  2. **Product Setup:** A simple toggle on the Product Edit screen: "Make this a recurring subscription" -> Select frequency (Weekly/Monthly/Yearly). No complex pricing tiers or billing logic exposed.
  3. **Customer Interaction:** Customer texts the business: "Hey, can I pause my coffee delivery for 2 weeks while I'm on vacation?" The Customer Success Agent reads the intent, executes the pause via the Subscription Ledger, and replies: "All set! I've paused your coffee delivery until [Date]."

  ### AI Agent Integration
  - **Finance Agent:** Monitors Stripe webhooks for `invoice.payment_failed`. Automatically drafts a polite, personalized follow-up to the customer and creates a task for the owner if the customer doesn't update their payment method within 3 days.
  - **Customer Success Agent:** Equipped with a `ManageSubscription` tool (Pause, Resume, Cancel, Upgrade) that it can call when interacting with customers via Inbox/SMS.

  ## Implementation Prompt
  Implement the core `SubscriptionLedger` data model and the bidirectional sync with Stripe Billing.
  1. Create the necessary database migrations for the `subscriptions` and `subscription_events` tables, ensuring strict RLS is applied.
  2. Implement the Stripe webhook handlers to listen for subscription lifecycle events (`customer.subscription.created`, `updated`, `deleted`, `invoice.payment_failed`) and update the local ledger.
  3. Expose a unified internal service layer (`SubscriptionService`) that the AI agents can use to query and mutate subscription states securely. Do not build the full UI in this task; focus on the robust backend architecture and the agent integration layer.
  4. Ensure 100% unit test coverage for the service layer and webhook handlers.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
