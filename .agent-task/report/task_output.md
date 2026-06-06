issue_title: "Research: Automated Product Subscriptions & Replenishment"
issue_description: |
  # Research Report: Automated Product Subscriptions & Replenishment

  ## Problem Statement
  Small businesses selling consumable goods (like Maya the baker or a local coffee roaster) struggle to establish predictable recurring revenue. Setting up subscription models on traditional platforms is highly technical, often requiring expensive third-party apps that break native checkout flows. Customers forget to reorder, and business owners lose lifetime value (LTV).

  ## Research Findings
  - **Shopify:** Requires third-party apps like ReCharge or Skio. These apps cost hundreds of dollars a month, inject code into the storefront, and have complex configuration dashboards that overwhelm non-technical users.
  - **Wix/Squarespace:** Basic subscription features exist but lack flexibility for customer self-management (e.g., "skip a month", "swap flavor").
  - **OHC Opportunity:** Subscriptions should be a native, one-click feature integrated directly into the core product catalog and Stripe Billing. The AI agents should handle the heavy lifting: predicting when a customer is running low and prompting them, or automatically managing the recurring charge and fulfillment workflow.

  ## Proposed Architecture
  ### Data Model
  - `SubscriptionPlan`: Attached to a `Product`, defining frequency (e.g., weekly, monthly) and discount logic.
  - `CustomerSubscription`: Tracks the active subscription state, linked to a `Customer` and a Stripe `Subscription`.
  - `FulfillmentSchedule`: Generated automatically based on the subscription interval to queue up Operations tasks.

  ### AI Agent Integration
  - **The Promoter (Marketing/Sales):** Analyzes purchase history to identify one-time buyers of consumable goods and sends an automated, personalized email offering a discount if they subscribe.
  - **The Ambassador (Customer Success):** Handles customer requests to pause or modify subscriptions via natural language (e.g., customer texts "Can I skip my coffee delivery next week?", agent understands, updates the `CustomerSubscription`, and confirms).
  - **The Manager (Operations):** Automatically injects the recurring orders into the standard daily fulfillment queue.

  ## Next Steps
  Design the `SubscriptionPlan` schema and the Stripe Billing integration layer. Create the UI for business owners to enable "Subscribe & Save" with a single toggle on the product creation screen.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
