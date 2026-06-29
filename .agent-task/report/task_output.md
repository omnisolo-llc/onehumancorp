issue_title: "Implement Autonomous AI Dispute and Chargeback Resolution Engine"
issue_description: |
  **Title**: Implement Autonomous AI Dispute and Chargeback Resolution Engine

  **Problem Statement**:
  When small business owners (like Maya the Baker or Carlos the Handyman) receive chargebacks or payment disputes, they often miss the notification, fail to gather compelling evidence (receipts, delivery confirmation, customer communication), and lose revenue by default. They lack the time and technical expertise to navigate complex Stripe or PayPal dispute resolution portals, leading to an automatic loss of funds and increased chargeback fees.

  **Research Report**:
  - **Market Context**: Platforms like Stripe provide automated dispute evidence submission via API, but require the merchant to actively manage and format the data. Shopify has "Fraud Protect" but it mainly covers unauthorized transactions, leaving service providers and custom order businesses exposed to "product not received" or "unacceptable quality" claims.
  - **Competitor Gap**: Square and Wix alert merchants of disputes but require manual evidence compilation.
  - **OHC Opportunity**: OHC inherently has access to the full customer interaction history (omnichannel inbox), proof of delivery/service (calendar/booking), and transaction details. A dedicated AI Finance/Legal Agent could autonomously detect disputes, compile a comprehensive evidence package, and submit it on the owner's behalf, transforming a high-stress manual process into a simple 1-tap approval.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
        participant Stripe
        participant DisputeWebhookHandler
        participant AIFinanceAgent
        participant OmnichannelDatabase
        participant MobileApp (Owner)
        Stripe->>DisputeWebhookHandler: Webhook: charge.dispute.created
        DisputeWebhookHandler->>AIFinanceAgent: Trigger Dispute Review Task
        AIFinanceAgent->>OmnichannelDatabase: Query Customer DMs, Invoices, Delivery Logs
        AIFinanceAgent->>AIFinanceAgent: Draft Evidence Package
        AIFinanceAgent->>MobileApp (Owner): Push Notification: "Dispute Received - Evidence Ready"
        MobileApp (Owner)->>AIFinanceAgent: 1-Tap Approve
        AIFinanceAgent->>Stripe: Submit Evidence via API
    ```
  - **Mobile UX Flow**:
    1. A push notification appears on the 375px mobile screen: "🚨 Chargeback Alert: $150 from John Doe."
    2. Tapping opens a clean card view summarizing the claim (e.g., "Product Not Received").
    3. Below, the AI agent presents a compiled "Evidence Package" (showing delivery date, John's SMS confirming receipt, and the signed invoice).
    4. A primary call-to-action button: "Submit Evidence to Bank".
    5. Success screen with a tracking timeline.
  - **AI Agent Integration**: The AI Finance Agent will use an LLM to read the dispute reason code, query the memory/omnichannel inbox for the specific `customer_id` and `transaction_id`, and format the findings into the specific evidence format required by the payment gateway API.
  - **Key Design Decisions**:
    - Evidence submission must require manual owner approval (1-tap) to ensure the business owner retains final authority.
    - Uses existing tenant-isolated data retrieval patterns.

  **Implementation Prompt**:
  Implement the Autonomous AI Dispute Resolution Engine.
  1. Create a secure webhook handler for `charge.dispute.created` events from payment providers (e.g., Stripe) in the backend.
  2. Implement an AI job queue worker that fetches relevant customer interactions, booking completions, and digital receipts from the tenant's database to compile an evidence package.
  3. Design a mobile-first (375px) Flutter UI card for the "Owner Feed" that clearly explains the dispute and presents the compiled evidence for 1-tap approval.
  4. Ensure all database interactions strictly enforce PostgreSQL row-level security (`tenant_id`).
  5. Include E2E Playwright tests simulating a dispute webhook, the resulting UI state, and the approval action.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
