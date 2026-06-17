issue_title: "Research: Add comprehensive Stripe integration for Omnichannel payments"
issue_description: |
  # Research Report: Add comprehensive Stripe integration for Omnichannel payments

  ## Problem Statement
  Based on the global SMB market research, payment gateway confusion is a significant pain point for small business owners. OHC aims to simplify operations, but without seamless, native Stripe integration, business owners are forced to navigate complex third-party setups to accept payments. This friction contradicts OHC's mission of providing an invisible, autonomous work assistant.

  ## Research Report
  The current OHC system has traces of Stripe integration (e.g., in UI tests and legacy frontend components), but lacks a robust, unified backend integration that handles Stripe Checkout Sessions, Payment Links, Payment Intents, Terminal SDK interactions, and webhook processing resiliently. Competitors like Shopify and Wix provide tightly coupled payment ecosystems. To achieve parity and surpass them with agentic workflows (e.g., AI auto-generating payment links via DMs), OHC must formalize its Stripe integration architecture.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[OHC Mobile/Web Client] --> B(OHC API Layer)
      B --> C{Stripe Integration Service}
      C --> D[Stripe API]
      D -.-> E(Stripe Webhooks)
      E --> F{Webhook Handler}
      F --> G[(PostgreSQL Ledger)]
      C --> G
  ```
  ### Mobile UX Flow (375px first)
  1.  **Checkout Card**: A streamlined, transparent glass-styled card displays the order summary and a prominent "Pay with Stripe" button.
  2.  **Native Experience**: Tap invokes a native-feeling Stripe UI or deeply integrated Terminal SDK flow for in-person payments.
  3.  **Confirmation**: A success state updates the UI instantly, confirming payment without page reloads.

  ### AI Agent Integration
  -   **Sales Assistant**: Can automatically draft payment links in response to DMs (e.g., "Ready to order? Here's the link: [Stripe Link]").
  -   **Finance Assistant**: Reconciles Stripe payouts and flags anomalies in the daily digest.

  ### Key Design Decisions
  -   **Idempotency**: All Stripe API calls must use idempotency keys.
  -   **Webhook Resiliency**: Webhook handlers must verify signatures and implement retry mechanisms with exponential backoff.
  -   **Multi-tenancy**: Strict isolation of Stripe credentials and customer data per tenant.

  ## Implementation Prompt
  **User Persona**: Carlos (Field Service Owner) needs to accept a deposit on-site.
  **CUJ**:
  1. Carlos creates a service quote in the OHC app.
  2. He taps "Request Deposit."
  3. The system generates a Stripe Payment Link and drafts an SMS/Email to the customer.
  4. The customer pays via the link.
  5. The OHC system receives the Stripe webhook, updates the quote status to "Deposit Paid," and notifies Carlos via his Agent Feed.

  **Acceptance Criteria**:
  - Implement the Stripe Service layer in the Go backend.
  - Create secure endpoints for generating Payment Links/Intents.
  - Implement a robust webhook listener that updates the database state based on payment success/failure.
  - Ensure all new API routes and database updates respect tenant isolation boundaries.
  - Add comprehensive E2E tests validating the full checkout and webhook flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
