issue_title: "F08: Real Provider Payment Sessions and Status Reconciliation"
issue_description: |
  **Title**: Reliable Provider Checkout & Payment Reconciliation

  **Problem Statement**:
  Currently, the app generates fabricated checkout-looking URLs (e.g., in proposals and bookings) instead of linking to real payment provider sessions. From an owner/operator's perspective, this creates a false sense of completion—customers might think they paid when they haven't, or the owner might deliver a service without securing the deposit. Small business owners need absolute trust that a "Paid" status means money is actually in their account, and a "Pending" status means the customer has a real way to pay.

  **Research Report**:
  - **Gap**: As noted in remediation finding F08, current code fabricates checkout URLs rather than bridging to a real provider session.
  - **Competitive Analysis**: Platforms like Shopify and Squarespace never guess payment states; they rely entirely on verified provider webhooks and sync sessions.
  - **Strategy Admission**:
    - **Stable OHC target ID**: OHC-03 (booking/deposit)
    - **Selected customer/stage**: Service business owners (e.g., Nora the agency principal, Maya the baker) at the Days 15–45 Launch stage.
    - **Evidence level**: Documented code gap (F08).
    - **Current behavior**: System generates fake checkout URLs, leading to false completion states.
    - **Business result**: Zero dropped deposits and 100% accurate payment status mapping to real funds.
    - **Metric/denominator**: Percentage of proposals with a true checkout session successfully created and reconciled.
    - **Dependencies/reuse**: Reuse existing payment connector and provider sandbox mode for testing.
    - **Non-goals**: Building a custom payment gateway or managing credit card data directly.
    - **Authority class**: Owner-delegated billing action (requires explicit API key and integration approval).
    - **Cost impact**: API polling and webhook ingestion costs; negligible compared to deposit value.
    - **Happy/failure-path checks**: Happy path creates a real provider session and sets state to "Pending", updating to "Paid" on receipt. Failure path sets explicit "Unavailable" state if the provider is down.
    - **Superpowers workflow**: used executing-plans and requesting-code-review skills; revision unknown due to truncation.
    - **Validation**: make lint and cargo clippy completed.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
      participant Customer
      participant OHC UI
      participant OHC Backend
      participant Payment Provider (Stripe)

      Customer->>OHC UI: Accept Proposal
      OHC UI->>OHC Backend: Request Checkout Session
      OHC Backend->>Payment Provider: Create Real Session (Idempotent)
      Payment Provider-->>OHC Backend: Session URL & ID
      OHC Backend-->>OHC UI: Return Real Checkout URL
      Customer->>Payment Provider: Complete Payment
      Payment Provider->>OHC Backend: Webhook (Payment Succeeded)
      OHC Backend->>OHC Backend: Reconcile & Mark Paid
    ```
  - **Mobile UX Flow**:
    1. Owner views proposal status on a 375px viewport (status badge shows "Pending Checkout").
    2. Customer views proposal and taps a prominent "Pay Deposit" button (styled with OHC design tokens, 16px border radius).
    3. Customer is redirected to the real provider checkout page.
    4. Upon return, the OHC dashboard updates immediately via SSE/WebSockets to show "Paid" with a timestamp.
  - **Zero Trust & Security**: Tenant-isolated API keys; provider requests must include the tenant's exact provider credential.
  - **Performance Targets**: Checkout session generation under 1500ms; offline UI degrades gracefully by hiding the payment button if disconnected.
  - **AI Agent Integration Points**: The Billing/CFO agent can monitor webhook events and notify the owner via the unified feed when a deposit clears, preventing the need for manual refreshing.

  **Implementation Prompt**:
  Implementer: Please replace the mocked checkout URL generation in the proposal and booking workflows with real payment provider session initialization.
  1. Integrate the existing payment provider connector to generate a real checkout session when a proposal is approved.
  2. If the provider is unconfigured or unavailable, fallback to an empty string or `None` and display a clear "Payment Setup Required" state in the UI.
  3. Ensure all money fields are strictly validated before creating the session.
  4. Ensure idempotent retries are supported so a network blip doesn't create duplicate provider sessions for the same proposal.
  5. DO NOT hardcode dummy provider URLs like `cs_test_` in the domain logic.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
