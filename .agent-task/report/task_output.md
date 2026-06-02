issue_title: "[architecture]_autonomous_subscription_box_fulfillment_engine"
issue_description: |
  # Research Report: Autonomous Subscription Box & Membership Engine

  ## Problem Statement
  Small business owners running recurring revenue models (like monthly subscription boxes, membership clubs, or recurring service retainers) face immense operational friction. Managing subscription billing is only half the battle; the real complexity lies in coordinating recurring fulfillment, tracking active member perks, managing churn/failed payments automatically, and keeping inventory synced across single-purchase vs. subscription stock. Maya (baker) wanting a "Cake of the Month" club or Leo (tutor) wanting a "Monthly Masterclass" tier currently need a complex integration of Stripe Billing + Shippo + manual spreadsheets. This is too hard for non-technical users.

  ## Research Report
  - **Current Market Gap**:
    - **Shopify**: Requires expensive third-party apps (e.g., Recharge, Skio) which add $99/mo + transaction fees. Setup is complex and breaks native checkout flows often.
    - **Wix/Squarespace**: Basic recurring billing exists, but lacks deep fulfillment tracking (e.g., "batch print all labels for this month's box").
    - **Patreon/Substack**: Good for digital content, poor for physical/hybrid boxes.
  - **OHC Opportunity**: Treat subscriptions not just as recurring billing, but as a holistic *Lifecycle* entity managed by the AI Departments. The platform should natively link a Stripe Subscription to an OHC Fulfillment Batch and notify the Operations Agent.

  ## Design Doc
  ### Mobile UX Flow (375px)
  1. **Creation**: Owner taps "Add Product" -> Selects "Subscription Box" -> Sets price, frequency (e.g., Monthly), and cut-off dates (e.g., "Ship on the 5th").
  2. **Dashboard Card**: A clean, translucent glass module on the home dashboard shows "Active Subscribers: 42" and "Upcoming Fulfillment: 42 boxes on Jan 5th".
  3. **Fulfillment Batching**: On the 5th, the Operations Agent sends a push notification: "Your 42 labels for January are ready to print." One tap prints all labels.
  4. **Churn Recovery**: The Customer Success Agent handles failed payments autonomously, sending friendly, plain-language reminders ("Hey, looks like your card expired! Update it here to get this month's box.") without the owner lifting a finger.

  ### AI Agent Integration
  - **Operations Agent**: Auto-generates fulfillment batches. Predicts inventory needs ("You have 42 active subscribers, order more flour by Monday").
  - **Finance Agent**: Manages Stripe Billing webhooks, prorations, and tax calculations automatically.
  - **Customer Success Agent**: Executes dunning (failed payment recovery) workflows using friendly, brand-aligned messaging.

  ### Implementation Prompt
  **User Facing Outcome:** A business owner can launch a subscription product in 2 minutes. The platform automatically handles recurring charges, generates batch fulfillment tasks, and recovers failed payments via AI.
  **CUJ (Critical User Journey):**
  1. Owner creates a "Monthly Coffee Bean" subscription product.
  2. Customer subscribes via the storefront.
  3. System creates a Subscription record and schedules the next Fulfillment Batch.
  4. At the billing cycle, System captures payment via Stripe Billing.
  5. Owner receives a consolidated list of shipping labels to print for the batch.
  **Acceptance Criteria:**
  - Create database entities for `SubscriptionPlan`, `Subscriber`, and `FulfillmentBatch` with strict multi-tenant isolation.
  - Integrate Stripe Billing API for plan creation and recurring webhooks.
  - Create a 375px-optimized UI for the owner to view active subscribers and batch-print labels.
  - Add AI Agent triggers for failed payment recovery.
  - Add Playwright E2E test covering the full subscription lifecycle.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
