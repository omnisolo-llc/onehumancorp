issue_title: "Implement Unified Multi-Tenant Subscription and Replenishment Architecture"
issue_description: |
  # Research Report: Unified Multi-Tenant Subscription and Replenishment Architecture

  ## Problem Statement
  Small businesses selling consumable goods (like Maya the baker, or a local coffee roaster) struggle to establish predictable recurring revenue. Setting up subscription models on traditional platforms is highly technical, often requiring expensive third-party apps that break native checkout flows. Customers forget to reorder, and business owners lose lifetime value (LTV) while managing complex third-party tools that do not integrate natively with their core operations.

  ## Research Report
  - **Competitor Analysis:**
    - **Shopify:** Requires third-party apps (e.g., ReCharge, Skio). These apps cost hundreds of dollars a month, inject code into the storefront, and have complex configuration dashboards that overwhelm non-technical users.
    - **Wix/Squarespace:** Offer basic subscription features but lack flexibility for customer self-management (e.g., "skip a month", "swap flavor").
  - **OHC Opportunity:** Subscriptions must be a native, one-click feature integrated directly into the core product catalog and Stripe Billing. AI agents should handle the heavy lifting: predicting when a customer is running low, prompting them to reorder, or automatically managing recurring billing and fulfillment workflows.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Product Settings] -->|Enable Subscription Toggle| B(Stripe Billing Integration)
      B --> C[Subscription Plan Configuration]
      C --> D{Customer Checkout Flow}
      D --> E[Stripe Customer Subscription]
      E --> F[Recurring Billing Event / Webhook]
      F --> G[Fulfillment Queue]
      G --> H[Operations Agent: Alert Owner / Manage exceptions]
  ```

  ### Data Model & Invariants (PostgreSQL)
  - `SubscriptionPlan`: Ties to a product with frequency (weekly, monthly) and discount settings. Tenant-scoped.
  - `CustomerSubscription`: Links a customer to a `SubscriptionPlan`, tracking status (active, paused, cancelled) and next billing date. Tenant-scoped.
  - **Invariants:** Row-level tenant isolation (RLS) is strictly enforced based on `tenant_id`. All subscription interactions flow through a secure server-side API integrating with Stripe Billing.

  ### Mobile UX Flow (375px)
  1. **Product Edit Screen:** A single toggle labeled "Enable Subscribe & Save".
  2. **Configuration:** If toggled on, simple options appear: "Delivery Frequency" (e.g., Weekly, Monthly) and "Discount %".
  3. **Customer Checkout:** The customer selects "Subscribe & Save X%" and completes the normal checkout flow directly within the mobile layout.
  4. **Owner Dashboard:** A unified feed displays new subscriptions, upcoming fulfillments, and AI-driven alerts for potential churn or re-engagement opportunities.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the subscription fulfillment queue, alerting the owner to upcoming necessary actions or inventory shortages.
  - **Sales/Customer Success Agent:** Automatically identifies customers at risk of churn or those who might want to modify their subscription (e.g., "swap flavor") and drafts re-engagement messages.

  ## Implementation Prompt
  **Target Persona:** Maya the baker
  **Outcome:** Maya can enable "Subscribe & Save" for her weekly bread boxes with a single toggle. Customers can subscribe seamlessly at checkout. The OHC system natively handles recurring billing via Stripe and adds weekly orders to her fulfillment queue without any manual intervention.

  **Acceptance Criteria:**
  1. **Database:** Implement `SubscriptionPlan` and `CustomerSubscription` models with strict RLS (`tenant_id`).
  2. **API:** Create secure endpoints for managing plans and processing customer subscriptions via Stripe Billing.
  3. **UI (Owner):** Add the "Enable Subscribe & Save" toggle to the mobile-first Product Edit screen, integrating smoothly into the existing form layout.
  4. **UI (Customer):** Add subscription options to the checkout flow, ensuring seamless mobile (375px) functionality.
  5. **Verification:** Provide Playwright E2E tests validating the end-to-end flow from owner setup to customer checkout and subscription activation.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
