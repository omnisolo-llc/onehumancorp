issue_title: "Research & Design: Proactive AI Autonomous Subscription Management"
issue_description: |
  # Research Report: Proactive AI Autonomous Subscription Management

  ## Problem Statement
  Small business owners managing recurring revenue models (e.g., subscription boxes, monthly service retainers) struggle with churn management and payment failures. Current tools (like Stripe Billing, Chargebee, or Shopify Subscriptions) are powerful but complex and highly reactive. When a payment fails or a customer wants to pause a subscription, the owner usually finds out after the fact or relies on rigid, impersonal automated dunning emails. There is no intelligent system that anticipates churn, proactively negotiates pauses or downgrades, and handles the operational overhead invisibly.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe Billing / Recharge (Shopify):** Offer excellent infrastructure for recurring billing and standard dunning rules (e.g., retry in 3 days, send email). However, the communication is static. They do not engage the customer in a dialogue to understand *why* the payment failed or if they want to adjust their plan.
  - **Chargebee:** Highly robust but overwhelmingly complex for a micro-SME (like Priya the boutique owner offering a monthly curated box).
  - **OHC Opportunity:** Leverage our AI agents to transform subscription management from a passive billing engine into an active, relationship-preserving service. Instead of a generic "Payment Failed" email, the Customer Success Agent can reach out via SMS or WhatsApp: "Hi Sarah, your card for the monthly box didn't go through. Do you want to update it, or would you like to pause this month's box?"

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Stripe Webhook: invoice.payment_failed] --> B(Event Mesh)
      B --> C{Subscription State Engine}
      C -->|Update Status| D[PostgreSQL: Subscription Record]
      C --> E[Finance Agent: The Accountant]
      E -->|Analyze Value/Risk| F[Customer Success Agent: The Ambassador]
      F -->|Query Context| G[Unified Customer Graph]
      F -->|Draft Proactive Message| H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|Owner Approves| J[Omnichannel Dispatcher]
      J --> K[SMS / WhatsApp to Customer]
      K -->|Customer Replies 'Pause'| L[Operations Agent]
      L -->|Update Stripe & DB| D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The owner sees an Action Card: "Subscription Risk: Sarah's payment failed. Approve outreach?"
  - **Interaction:** Tapping the card shows the drafted message. The message offers options based on the customer's history (e.g., offer a 1-month pause instead of immediate cancellation if they are a long-term customer).
  - **Action:** Primary button "Send Options", Secondary "Cancel Subscription", Tertiary "Edit Message".
  - **Visual Design:** Premium Translucent Glass styling, utilizing UniFi-style modular dashboard cards. Clear status indicators for active, paused, and past-due subscriptions.

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant):** Monitors Stripe webhooks for billing events, forecasts recurring revenue, and flags high-risk accounts.
  - **Customer Success Agent (The Ambassador):** Drafts personalized, omnichannel messages to handle failed payments, upgrade opportunities, or pause requests.
  - **Operations Agent (The Manager):** Executes the backend logic (pausing the subscription in Stripe, updating inventory forecasts).

  ### Key Design Decisions
  - **Proactive Churn Mitigation:** Shift from rigid dunning emails to conversational, multi-channel problem solving.
  - **Zero-Touch Execution:** The owner only needs to approve the strategy (e.g., "Offer a pause"). The agents handle the customer dialogue and the underlying Stripe API calls.
  - **Mobile-First Visibility:** Complex subscription metrics (MRR, churn rate) must be distilled into simple, plain-language insights on the mobile dashboard.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner offering a monthly service, when a customer's payment fails, my OHC app immediately drafts a polite message offering them the option to update their payment method or pause for a month. If they reply "pause", the system automatically pauses their subscription and updates my revenue forecast, without me touching a single setting.

  **Next Actions for Engineering:**
  1. Define the `Subscription` and `SubscriptionEvent` data models in PostgreSQL, ensuring alignment with Stripe's data structures and strict multi-tenant isolation.
  2. Implement the Stripe webhook listener for subscription lifecycle events (created, updated, deleted, payment_failed).
  3. Integrate the Finance and Customer Success agents to process these events and generate Action Cards for the mobile feed.
  4. Develop the Mobile UI for subscription management (list view, detail view, and action cards).

  **Priority:** P1
  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
