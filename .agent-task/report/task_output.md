issue_title: "Implement Automated Product Subscriptions & Replenishment"
issue_description: |
  **Title:** Automated Product Subscriptions & Replenishment

  **Problem Statement:**
  Small businesses selling consumable goods (like Maya the baker or a local coffee roaster) struggle to establish predictable recurring revenue. Setting up subscription models on traditional platforms is highly technical, often requiring expensive third-party apps that break native checkout flows. Customers forget to reorder, and business owners lose lifetime value (LTV).

  **Research Report:**
  - **Shopify:** Requires third-party apps like ReCharge or Skio. These apps cost hundreds of dollars a month, inject code into the storefront, and have complex configuration dashboards that overwhelm non-technical users.
  - **Wix/Squarespace:** Basic subscription features exist but lack flexibility for customer self-management (e.g., "skip a month", "swap flavor").
  - **OHC Opportunity:** Subscriptions should be a native, one-click feature integrated directly into the core product catalog and Stripe Billing. The AI agents should handle the heavy lifting: predicting when a customer is running low and prompting them, or automatically managing the recurring charge and fulfillment workflow.

  **Design Doc:**
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

  ### Mobile UX Flow
  1. **Product Edit Screen (375px):** A single toggle labeled "Enable Subscribe & Save".
  2. **Configuration:** If toggled on, simple options appear: "Delivery Frequency (e.g., Weekly, Monthly)" and "Discount %".
  3. **Customer Checkout:** Customer selects "Subscribe & Save X%" and completes the normal checkout flow.
  4. **Subscription Management:** Customer receives an email with a secure link to manage their subscription (pause, skip, cancel).

  ### AI Agent Integration Points
  - **The Promoter (Marketing/Sales):** Analyzes purchase history to identify one-time buyers of consumable goods and sends an automated, personalized email offering a discount if they subscribe.
  - **The Ambassador (Customer Success):** Handles customer requests to pause or modify subscriptions via natural language (e.g., customer texts "Can I skip my coffee delivery next week?", agent understands, updates the `CustomerSubscription`, and confirms).
  - **The Manager (Operations):** Automatically injects the recurring orders into the standard daily fulfillment queue.

  ### Key Design Decisions
  - **Native Integration:** Subscriptions must be a native feature, avoiding third-party apps to maintain simplicity and a cohesive user experience.
  - **Mobile-First UX:** The setup process must be completely manageable on a 375px screen with large touch targets and simplified forms.
  - **Agentic Automation:** Leverage AI agents for proactive marketing (The Promoter) and customer support (The Ambassador) to reduce manual work for the owner.

  **Implementation Prompt:**
  Implement the "Subscribe & Save" toggle on the Product Edit screen and connect it to Stripe Billing. The user should be able to enable subscriptions for a product, specify the frequency and discount, and customers should be able to subscribe during checkout. Create a simple customer portal for managing active subscriptions. Ensure the Critical User Journey (CUJ) is fully functional on mobile viewports (375px) and includes end-to-end testing with Playwright, verifying the flow from product setup to successful checkout and subscription management. Do not prescribe specific database schemas or API endpoints; design those during implementation to best fit the existing architecture.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []