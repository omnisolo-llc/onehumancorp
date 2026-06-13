issue_title: "[Architecture] Invisible Multi-Party Split Payments Ledger"
issue_description: |
  **Problem Statement**
  Small business owners often act as intermediaries coordinating contractors, suppliers, and client funds. Nora (the agency principal) needs to collect an initial deposit, pay a designer a sub-contract fee, and retain her margin, but traditional platforms require her to manually receive all funds, track obligations on spreadsheets, pay out contractors separately, and reconcile bank statements. This multi-step process limits scaling and creates cash-flow friction. There is an architectural gap for an invisible, automated split-payment ledger.

  **Research Report**
  Current platforms like Shopify or simple Stripe checkouts assume a single seller receiving the entire payment. While Stripe Connect exists, integrating its complex standard/custom flows requires significant development and often shifts operational complexity back to the owner. OHC requires a seamless ledger layer that can abstract multi-party payments (e.g., deposits, contractor payouts, platform fees) invisibly behind a "Single Quote." The ledger must support immediate transaction splitting, escrow-like holds, and multi-tenant isolation, scaling across both fiat and potential multi-currency/crypto rails.

  **Design Doc**
  - **Architecture Diagram (Mermaid.js)**
    ```mermaid
    erDiagram
        LEDGER_ENTRY {
            uuid id
            uuid tenant_id
            uuid transaction_group_id
            string entry_type
            decimal amount
            string currency
            uuid source_party_id
            uuid destination_party_id
            string status
            timestamp created_at
        }
        TRANSACTION_GROUP {
            uuid id
            uuid tenant_id
            string reference_type
            uuid reference_id
            string status
        }
        PAYMENT_ROUTING_RULE {
            uuid id
            uuid tenant_id
            uuid product_service_id
            decimal split_percentage
            uuid destination_party_id
        }
        TRANSACTION_GROUP ||--o{ LEDGER_ENTRY : "contains"
        PAYMENT_ROUTING_RULE ||--o{ TRANSACTION_GROUP : "defines"
    ```
  - **Mobile UX Flow**
    1. Nora creates a project quote on her phone. She adds a "Design Work" line item and assigns the contractor "Alex" to it.
    2. She configures a rule (hidden under Advanced Settings): 70% to Alex, 30% retained.
    3. The client pays via an OHC payment link.
    4. The Finance Assistant instantly updates the daily summary: "Client paid $1,000. $700 queued for Alex, $300 retained." The complexity is entirely handled by the background ledger.
  - **AI Agent Integration Points**
    - **Finance Assistant**: Reads the ledger to generate simple daily cash-flow summaries for the owner.
    - **Sales Assistant**: Proposes routing rules based on past project setups when drafting new quotes.
    - **Operations Assistant**: Triggers task completion events that release held funds.
  - **Key Design Decisions**
    - **Multi-Tenant Isolation**: Enforced strictly on `tenant_id` at the database row level.
    - **Immutable Ledger**: Entries are append-only. Corrections require compensating entries.
    - **Abstracted Routing**: Rules are decoupled from the payment gateway to allow future multi-processor support.

  **Implementation Prompt**
  Implement the backend data models and service layer for the Invisible Multi-Party Split Payments Ledger. Create the database schemas for ledger entries, transaction groups, and routing rules with strict tenant isolation. Expose internal APIs (gRPC/REST) to create routing rules and record transaction groups. The implementation MUST ensure append-only ledger behavior and provide a method to query a tenant's current balance and pending obligations. Add unit tests covering standard split scenarios and tenant isolation edge cases.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
