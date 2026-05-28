issue_title: "[Architecture] Autonomous Subscription & Retention Engine"
issue_description: |
  # [Architecture] Autonomous Subscription & Retention Engine

  ## Problem Statement
  Small business owners like Leo (a music tutor offering weekly lesson packages) and Priya (a boutique owner launching a monthly "curated box" subscription) face massive friction when transitioning from one-off sales to recurring revenue. Currently, managing subscriptions requires setting up external billing systems (like Stripe Billing or Recharge), tracking failed payments manually, and awkwardly emailing customers whose cards have declined (dunning). If Leo has to spend 3 hours a week chasing down failed payments, his business growth stalls. They need a system that automatically handles recurring billing, failed payment retries, and customer retention seamlessly from their phone, with AI agents invisibly handling all the awkward financial conversations.

  ## Research Report
  **Competitive Analysis:**
  - **Stripe Billing:** Industry standard for developers, extremely powerful but complex. The dashboard is intimidating for non-technical solopreneurs. Dunning exists but relies on basic email templates.
  - **Recharge (Shopify):** Great for e-commerce, but expensive and heavily tied to physical product flows. Doesn't serve service providers like Leo well.
  - **Patreon / Substack:** Good for digital creators, but they take a huge cut (up to 10%) and don't integrate with the user's broader business operations (like Priya's physical inventory).

  **Market Needs:**
  Solopreneurs need a subscription engine that is native to their business OS. It must support both physical products (Priya's boxes) and service bookings (Leo's lessons). Crucially, the "dunning" (chasing failed payments) must be handled gracefully by AI to preserve the business owner's relationship with the customer.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> SubManagerUI[Subscription Manager UI];
          SubManagerUI --> LocalDB[(Local Cache CRDT)];
      end

      App -- "Manage Plans" --> Gateway[OHC API Gateway];

      Gateway --> BillingEngine[Subscription & Dunning Engine];
      BillingEngine --> MainDB[(Cloud Postgres Ledger)];
      BillingEngine --> PaymentProvider[Stripe/Local Payment Gateways];

      Gateway --> KAIROS[KAIROS Orchestrator];
      KAIROS --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> FinanceAgent[Finance: Auto-Retry & Reconciliation];
          Agents --> CSAgent[Customer Success: AI Dunning & Retention];
          Agents --> OpsAgent[Ops: Schedule Recurring Deliveries/Bookings];
      end

      BillingEngine -- "Webhook: Payment Failed" --> KAIROS;
  ```

  ### Mobile UX Flow (375px First)
  1. **Plan Creation:** Leo opens the app and taps "New Subscription Plan". He sets the price ($200/mo) and selects "Weekly Music Lesson". The UI uses sleek Glassmorphism cards. No mention of "Stripe" or "Webhooks".
  2. **Customer Subscription:** A student signs up via Leo's OHC link. Leo gets a push notification with a celebratory animation.
  3. **The "Awkward Conversation" Avoidance:** A student's card declines on month 3. The AI Customer Success Agent automatically detects this via KAIROS. It sends a polite, personalized SMS to the student (e.g., "Hey Sarah, Leo's AI assistant here! Looks like your card expired. Here's a secure link to update it so we don't miss next week's lesson!"). Leo simply sees a subtle "Action Required: Payment Pending" badge on the student's profile, but doesn't have to lift a finger.
  4. **Retention Analytics:** Priya views her dashboard and sees a simple chart: "MRR (Monthly Recurring Revenue)" and "Churn Rate". All complex SaaS metrics are simplified for a non-technical user.

  ### AI Agent Integration Points
  - **Customer Success (CS) Agent:** Acts as the automated dunning manager. When a webhook fires for a failed payment, the KAIROS orchestrator wakes the CS Agent. It drafts and sends highly personalized recovery emails or SMS messages based on the customer's history.
  - **Finance Agent:** Handles the complex logic of prorating upgrades/downgrades and reconciling the recurring payouts into the business owner's central ledger.
  - **Operations (Ops) Agent:** For physical goods (Priya), it automatically decrements inventory and generates a shipping label each month. For services (Leo), it automatically blocks out the calendar slot for the recurring lesson.

  ### Key Design Decisions
  - **Unified Engine:** We do not rely on a 3rd party UI for subscriptions. OHC handles the state, and only uses the payment gateway as a dumb pipe. This ensures we control the UX end-to-end.
  - **Graceful Degradation:** The subscription logic is tracked in the cloud Postgres database, but the mobile app's local CRDT cache ensures Leo can view his active subscribers and MRR even while offline.
  - **Zero-Trust & Security:** Financial webhooks are processed through SPIFFE-authenticated KAIROS workflows to ensure malicious actors cannot spoof payment success events.

  ## Implementation Prompt
  Implement the Autonomous Subscription & Retention Engine.
  - **User-Facing Outcome:** Users can create recurring subscription plans for products or services. Customers can subscribe, and the system automatically bills them on schedule. Failed payments trigger an AI-driven, personalized dunning sequence via SMS/Email to recover the payment without the business owner's manual intervention.
  - **CUJ:** User creates a "$50/mo Consulting" plan. Customer subscribes. Month 2 payment fails. The OHC system catches the webhook, the CS Agent sends a friendly recovery SMS with an update link. Customer updates the card, payment succeeds, and the user receives a push notification that the subscription is active again.
  - **Acceptance Criteria:**
    - Create the subscription management UI adhering to the 375px mobile-first Glassmorphism design system.
    - Implement the backend Subscription & Dunning Engine that listens for payment webhooks.
    - Integrate KAIROS orchestration to trigger the CS, Finance, and Ops agents upon subscription lifecycle events (created, renewed, failed, canceled).
    - Ensure all metrics and terminology are simplified for non-technical users (no developer jargon).

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []