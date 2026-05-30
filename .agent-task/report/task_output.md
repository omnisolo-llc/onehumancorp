issue_title: "Implement Automated Subscription & Membership Billing Engine"
issue_description: |
  # [Architecture] Automated Subscription & Membership Billing Engine

  ## Problem Statement
  Small business owners like Leo (a music tutor offering weekly lessons) and Priya (a boutique owner launching a monthly "curated box" membership) need reliable recurring revenue to sustain their businesses. Currently, setting up subscriptions requires integrating complex third-party tools (like Patreon or specialized Shopify apps) that don't talk directly to their core scheduling or inventory systems. If Leo wants to offer a "4 lessons a month" package, he has to manually track if the student paid their monthly fee before confirming calendar slots. They need a native, fully automated subscription and membership engine accessible right from their mobile device, where billing, failed payment retries (dunning), and perk access (like unlocking calendar slots or digital downloads) are handled invisibly by AI.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify Subscriptions / ReCharge:** Highly capable but often requires expensive external app integrations. The setup is complex, involving strict product configurations that aren't ideal for service-based businesses like tutoring.
  - **Patreon / Substack:** Excellent for digital creators, but entirely siloed from a business owner's primary inventory, physical POS, or service booking calendar.
  - **Stripe Billing:** Powerful API for subscriptions, but the dashboard is designed for developers and financial teams. It is not intuitive for a non-technical solopreneur trying to manage memberships on a 375px mobile screen.

  **Gaps Identified:**
  OHC lacks a native recurring billing and membership management engine. We need a system that securely orchestrates recurring payments via a unified gateway while seamlessly triggering cross-department AI agents. For example, when a subscription payment succeeds, the system should automatically unlock calendar access for the service provider or decrement recurring inventory.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> SubUI[Membership & Subscription Manager];
          SubUI --> LocalCRDT[(Local Cache)];
      end

      App -- "Configure Subscription Tier" --> Gateway[OHC API Gateway];

      Gateway --> BillingEngine[Subscription Billing Engine];
      BillingEngine --> MainDB[(Cloud Postgres Ledger)];
      BillingEngine --> Stripe[Stripe Billing API];

      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Unlock Calendar/Inventory];
          Agents --> CSAgent[Customer Success: Dunning & Churn Win-back];
          Agents --> FinanceAgent[Finance: Recurring Revenue Forecasting];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Tier Creation:** Leo opens the OHC app and navigates to "Products & Services". He taps a new floating action button designed with a sleek Glassmorphism effect: "Create Subscription".
  2. **Simplified Pricing:** A simple, card-based UI asks "What are you offering?" (e.g., "Weekly Guitar Lessons"). He sets the price ($200/month) and selects the perks (unlocks 4 calendar slots). No complex "billing interval" dropdowns; the AI infers the standard setup from plain text or simple toggles.
  3. **Customer View:** The student sees a clean, one-tap checkout on Leo's mobile storefront to subscribe via Apple Pay or credit card.
  4. **Dashboard Analytics:** On his home screen, Leo sees a unified "Monthly Recurring Revenue (MRR)" metric cleanly integrated into his standard daily sales chart.

  ### AI Integration Points
  - **Customer Success (CS) Agent (Dunning & Churn):** If a student's credit card fails, the CS Agent autonomously drafts and sends a polite, personalized SMS with a secure link to update their payment method, without Leo having to intervene. If a user cancels, the CS Agent can offer a personalized "win-back" discount.
  - **Operations Agent:** Immediately upon successful recurring payment, this agent updates the student's permissions, unlocking exactly 4 new slots in Leo's booking calendar.
  - **Finance Agent:** Aggregates upcoming recurring payments to provide cash-flow forecasting in the mobile dashboard.

  ### Key Design Decisions & Security
  - **Unified Engine:** We wrap Stripe Billing behind our K8s/LangGraph orchestrated Swarm. The business owner never touches the Stripe Dashboard directly.
  - **Zero-Trust Multi-Tenancy:** Subscription states and PII (like customer payment profiles) are strictly isolated using SPIFFE SVIDs, ensuring Leo cannot accidentally query Priya's subscribers.
  - **Abstracted Complexity:** "Dunning", "Proration", and "MRR" are translated into simple, human-readable concepts like "Payment Failed Reminders", "Partial Month Charges", and "Guaranteed Monthly Income".

  ## Implementation Prompt
  Implement the Automated Subscription & Membership Billing Engine.
  - **User-Facing Outcome:** Business owners can create and manage recurring subscription products (services, physical boxes, or digital memberships) directly from their mobile app. Customers can subscribe seamlessly, and the system handles billing, retries, and perk access automatically.
  - **CUJ:** A user creates a "$50/mo Premium Tier". A customer subscribes. Next month, the customer's card fails. The AI CS Agent texts them a link to update it. They update the card, the payment clears, and the AI Ops Agent immediately restores their access to the premium content or booking calendar.
  - **Acceptance Criteria:**
    - Strict adherence to the 375px mobile UI standard, using Translucent Glass and modular card layouts.
    - Integration with Stripe Billing (or equivalent) masked behind OHC's unified API.
    - The AI CS Agent must correctly intercept webhook events for `payment_failed` to execute the dunning workflow.
    - The feature must completely hide developer terminology (no mentions of webhooks, dunning, or CRDTs to the user).
  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
