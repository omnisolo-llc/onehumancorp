issue_title: "Automated Product Subscriptions & Replenishment"
issue_description: |
  # Automated Product Subscriptions & Replenishment

  ## Target Persona: Maya (Home Baker) & Local Coffee Roaster

  ## Problem Statement
  Small businesses selling consumable goods struggle to establish predictable recurring revenue. Setting up subscription models on traditional platforms (like Shopify) is highly technical, often requiring expensive third-party apps (like ReCharge or Skio) that break native checkout flows and introduce disjointed customer experiences. Customers forget to reorder, and business owners lose lifetime value (LTV).

  ## Research Report
  Our competitive research highlights a significant gap in the SMB commerce landscape:
  - **Shopify**: Relies heavily on the "App Tax." Merchants must pay hundreds of dollars monthly for subscription apps. These apps inject foreign code into the storefront and possess complex configuration dashboards that overwhelm non-technical users like Maya.
  - **Wix/Squarespace**: Offer basic subscription features but lack flexibility for customer self-management (e.g., "skip a month", "swap flavor"). They also lack proactive AI follow-ups.
  - **OHC Opportunity**: Subscriptions must be a native, one-click feature integrated directly into the core product catalog, leveraging Stripe Billing seamlessly. Crucially, OHC's AI agents will handle the heavy lifting: predicting when a customer is running low to prompt a reorder, or automatically managing the recurring charge, communication, and fulfillment workflow without the owner lifting a finger.

  ## Design Doc

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

      I[The Promoter Agent] -.->|Upsell One-Time Buyers| D
      J[The Ambassador Agent] -.->|Handle Modifications| E
  ```

  ### Mobile UX Flow (375px First)
  1. **Product Edit Screen**: A single toggle labeled "Enable Subscribe & Save".
  2. **Configuration**: If toggled on, simple options appear: "Delivery Frequency (e.g., Weekly, Monthly)" and "Discount %".
  3. **Customer Checkout**: Customer selects "Subscribe & Save X%" and completes the normal checkout flow.
  4. **Subscription Management**: Customer receives an email with a secure link to manage their subscription (pause, skip, cancel) via a clean mobile interface.

  ### AI Agent Integration Points
  - **The Promoter (Marketing/Sales)**: Analyzes purchase history to identify one-time buyers of consumable goods and sends an automated, personalized email offering a discount if they subscribe.
  - **The Ambassador (Customer Success)**: Handles customer requests to pause or modify subscriptions via natural language (e.g., customer texts "Can I skip my coffee delivery next week?", agent understands, updates the `CustomerSubscription`, and confirms).
  - **The Manager (Operations)**: Automatically injects the recurring orders into the standard daily fulfillment queue and alerts the owner of upcoming volume.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, I want to toggle "Subscribe & Save" on my coffee beans with a 10% discount. Customers should be able to subscribe at checkout, and I should see their recurring orders automatically appear in my daily fulfillment queue without configuring complex apps.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Implement the "Subscribe & Save" toggle on the Product Edit schema and UI, extending the data model to support subscription frequency and discount percentage.
  2. Integrate the checkout flow with Stripe Billing to create recurring `CustomerSubscription` objects when a subscription product is purchased.
  3. Create a simple customer-facing portal (or integration for The Ambassador agent) allowing customers to manage active subscriptions (pause, skip, cancel).
  4. Build an E2E Playwright test: A business owner toggles subscription on a product, a customer purchases it as a subscription, and the resulting recurring entity is verified in the backend.
  5. Do NOT prescribe specific database schema names or function signatures; focus on the seamless end-to-end integration and native Stripe Billing utilization.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
