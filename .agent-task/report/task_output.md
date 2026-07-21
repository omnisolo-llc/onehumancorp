issue_title: "Design Agentic Automated Subscription Replenishment System"
issue_description: |
  # Research Report: Automated Product Subscriptions & Replenishment

  ## 1. Problem Statement
  Small businesses selling consumable goods (like Maya the baker or a local coffee roaster) struggle to establish predictable recurring revenue. Setting up subscription models on traditional platforms is highly technical, often requiring expensive third-party apps that break native checkout flows. Customers forget to reorder, and business owners lose lifetime value (LTV).

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps like ReCharge or Skio. These apps cost hundreds of dollars a month, inject code into the storefront, and have complex configuration dashboards that overwhelm non-technical users. Wix/Squarespace have basic subscription features but lack flexibility for customer self-management (e.g., "skip a month", "swap flavor").
  - **The OHC Opportunity**: Subscriptions should be a native, one-click feature integrated directly into the core product catalog and Stripe Billing. The AI agents handle the heavy lifting: predicting when a customer is running low and prompting them, or automatically managing the recurring charge and fulfillment workflow.
  - **Competitor Gaps**:
    - *Shopify*: Expensive third-party ecosystem for subscriptions.
    - *Wix*: Basic, inflexible subscription flows.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Product Settings] -->|Enable Subscription Toggle| B(Stripe Billing Integration)
      B --> C[Subscription Plan]
      C --> D{Customer Checkout}
      D --> E[Stripe Customer Subscription]
      E --> F[Recurring Billing Event]
      F --> G[Fulfillment Queue]
      G --> H[Operations Agent: Alert Owner]
  ```

  ### Mobile UX Flow (375px)
  1. **Product Edit Screen**: A single toggle labeled "Enable Subscribe & Save".
  2. **Configuration**: If toggled on, simple options appear: "Delivery Frequency (e.g., Weekly, Monthly)" and "Discount %".
  3. **Customer Checkout**: Customer selects "Subscribe & Save X%" and completes the normal checkout flow.
  4. **Subscription Management**: Customer receives an email with a secure link to manage their subscription (pause, skip, cancel).

  ### AI Agent Integration
  - **The Promoter (Marketing/Sales)**: Analyzes purchase history to identify one-time buyers of consumable goods and sends an automated, personalized email offering a discount if they subscribe.
  - **The Ambassador (Customer Success)**: Handles customer requests to pause or modify subscriptions via natural language (e.g., customer texts "Can I skip my coffee delivery next week?", agent understands, updates the `CustomerSubscription`, and confirms).
  - **The Manager (Operations)**: Automatically injects the recurring orders into the standard daily fulfillment queue.

  ## 4. Implementation Prompt
  **Feature Name**: Native Subscribe & Save
  **Target Persona**: Maya the Baker
  **Outcome**: Maya can easily offer weekly cake or cookie subscriptions directly from her product pages without installing complex apps. The Operations agent automatically lines up the orders for her each week.

  **Next Actions**:
  1. Implement the "Subscribe & Save" toggle on the Product Edit UI and connect it to Stripe Billing.
  2. The user should be able to enable subscriptions for a product, specify the frequency and discount.
  3. Customers should be able to select the subscription option during checkout.
  4. Create a simple, mobile-friendly customer portal for managing active subscriptions.
  5. Do NOT prescribe specific database schemas; focus on the integration between the UI, Stripe, and the Agent's ability to trigger the fulfillment workflow.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
