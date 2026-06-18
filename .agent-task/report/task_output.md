issue_title: "Implement 'Subscribe & Save' Functionality for Consumable Products"
issue_description: |
  ## Problem Statement
  Small businesses selling consumable goods (e.g., Maya the baker, a local coffee roaster) struggle to establish predictable recurring revenue. Setting up subscription models on traditional platforms is highly technical, often requiring expensive third-party apps that break native checkout flows and overwhelm non-technical users. Customers forget to reorder, leading to lost lifetime value (LTV).

  ## Research Report
  - **Competitor Analysis:** Shopify relies on third-party apps (ReCharge, Skio) costing hundreds of dollars monthly and injecting complex code. Wix and Squarespace offer basic subscription features but lack flexibility for customer self-management (e.g., "skip a month").
  - **OHC Opportunity:** Subscriptions should be a native, one-click feature integrated directly into the core product catalog and Stripe Billing. AI agents should handle predictions for low stock and automatically manage recurring charges and fulfillment workflows.
  - **Target Persona:** Maya the Baker (or similar consumable goods sellers).

  ## Design Doc
  ### Architecture
  - **Data Model:** Extending existing Product settings to include a `is_subscribable` toggle, `subscription_frequency` (e.g., weekly, monthly), and `subscription_discount_percent`.
  - **Integration:** Connect product configuration directly to Stripe Billing to generate Subscription Plans. Customers checking out with a subscription will create a Stripe Customer Subscription.
  - **Fulfillment:** Recurring billing events from Stripe will feed into a Fulfillment Queue managed by the Operations Agent.

  ### Mobile UX Flow (375px Target)
  1. **Product Edit Screen:** A simple toggle labeled "Enable Subscribe & Save".
  2. **Configuration Options:** When toggled on, simple dropdowns for Delivery Frequency and Discount % appear.
  3. **Checkout Experience:** Customers see a clear option to "Subscribe & Save X%" alongside standard checkout.
  4. **Customer Portal:** A secure, mobile-friendly view for customers to manage (pause, skip, cancel) their subscriptions.

  ### AI Agent Integration
  - **The Promoter:** Analyzes purchase history and identifies one-time buyers of consumables, sending automated, personalized emails offering a discount to subscribe.
  - **The Ambassador:** Handles customer requests to pause or modify subscriptions via natural language SMS or chat.
  - **The Manager:** Automatically injects recurring orders into the daily fulfillment queue.

  ## Implementation Prompt
  Implement the "Subscribe & Save" feature natively within OHC.
  1. Add UI toggles on the Product Edit screen (mobile-first) to enable subscriptions, set frequency, and discount.
  2. Update the backend data model to support these product settings and integrate with the Stripe Billing service.
  3. Ensure the customer checkout flow natively supports selecting the subscription option.
  4. Build the foundation for a simple customer portal to manage active subscriptions.
  The feature must feel like a natural extension of the product catalog, requiring zero technical setup from the business owner. Ensure thorough automated E2E testing using Playwright.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
