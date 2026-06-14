issue_title: "Implement Autonomous Universal Subscription & Retainer Engine"
issue_description: |
  # Mission Queue Protocol: Autonomous Universal Subscription & Retainer Engine

  ## 1. Title
  Implement Autonomous Universal Subscription & Retainer Engine

  ## 2. Problem Statement
  Service providers like Leo (Music Tutor) and Nora (Agency Principal) struggle with managing recurring revenue. Current SMB platforms treat subscriptions as an afterthought, often requiring complex third-party billing portals. They need a system that natively handles lesson packages, monthly retainers, failed payment recovery, and usage tracking, integrated directly with the OHC Assistant so the AI can proactively manage expiring packages and follow-ups.

  ## 3. Research Report
  - **Market Context**: Shopify requires apps like Recharge for subscriptions, which adds cost and complexity. Wix subscriptions are basic and detached from service fulfillment. Stripe Billing is powerful but too complex for non-technical users to set up without developer help.
  - **The OHC Opportunity**: By building a native subscription and retainer engine, OHC can unify recurring revenue with service delivery (Operations Agent) and customer relationship management (Customer Success Agent).
  - **Competitor Gaps**:
    - *Shopify*: Subscriptions are bolted on; poor native service/booking integration.
    - *Wix*: Passive subscription management.
    - *Stripe Billing*: Too technical for Maya, Leo, or Nora to configure directly.

  ## 4. Design Doc
  ### Data Model & Invariants
  - `SubscriptionProduct`: Defines the recurring offer (e.g., "4 Lessons/Month", "Design Retainer").
  - `SubscriptionContract`: The agreement between the Customer and Tenant, tracking state (active, past_due, canceled), billing cycle, and usage limits.
  - `UsageLedger`: Tracks consumption of the subscription (e.g., 3/4 lessons used).
  - Strict multi-tenant isolation via `tenant_id` and RLS on all tables.

  ### AI Agent Coordination
  - **Sales & Revenue Agent ("The Accountant")**: Monitors billing cycles, handles Stripe Checkout Session creation for subscriptions, and retries failed payments natively.
  - **Operations Agent ("The Manager")**: Automatically refills usage quotas at the start of a billing cycle (e.g., adds 4 lesson credits) and coordinates with the Booking Engine.
  - **Customer Success Agent ("The Ambassador")**: Drafts proactive messages: "Hi Sarah, your card for the monthly music lessons is expiring soon. Update it here: [Link]".

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean customer portal to view active subscriptions, remaining credits, and update payment methods.
  2. **Owner View (Dashboard)**: A "Recurring Revenue" card on the mobile dashboard. Tap to view active subscribers, upcoming renewals, and AI-suggested actions for failed payments.

  ## 5. Implementation Prompt
  **Feature Name**: OHC Native Subscription & Retainer Engine
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer a "4 Lessons/Month" subscription. The system handles recurring Stripe payments, automatically adds 4 booking credits to the student's account each month, and the Assistant follows up on failed payments.

  **Next Actions**:
  1. Implement the core Data Models (`SubscriptionProduct`, `SubscriptionContract`, `UsageLedger`).
  2. Integrate with Stripe Billing with robust webhook handling for invoice events.
  3. Develop the usage ledger logic to refill credits upon successful recurring payment.
  4. Build the mobile-first (375px) Owner Dashboard view for managing subscriptions and the Customer Portal.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
