issue_title: "Implement Agentic Quote-to-Cash & Dynamic Deposit System for Custom Orders"
issue_description: |
  ## Problem Statement
  Custom orders (like Maya's specialized cakes or Carlos's home repairs) require conversational negotiation, customized pricing, and deposit collection before work begins. Legacy platforms (Shopify, Wix) assume static catalogs and fixed prices. Custom quotes require complex, disconnected form-builder plugins and manual invoicing. SMB owners lose leads because they are too busy executing work to reply, draft quotes, and collect deposits in real-time. They need an assistant that handles the end-to-end "Quote-to-Cash" flow autonomously from chat.

  ## Research Report
  - **Competitor Analysis**: Shopify excels at static SKUs but struggles with custom quoting without expensive third-party apps like "Globo Form Builder." Square provides robust manual invoicing but lacks conversational AI to negotiate the quote dynamically via SMS or Instagram DMs.
  - **Market Gap**: There is a missing link between Conversational AI (e.g., ChatGPT) and Commerce ledgers. SMBs need an AI that not only talks to the customer but is authorized to generate a binding quote, issue a secure payment link for a deposit, and lock a calendar slot, all within the chat interface.
  - **Persona Focus**: Maya (Baker) and Carlos (Handyman). Both operate primarily from mobile and rely heavily on word-of-mouth or social media DMs for inbound leads.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **The Lead**: Customer messages Maya on Instagram: "I need a 2-tier vegan chocolate cake for next Saturday. How much?"
  2. **Agent Triage**: OHC Omni-Inbox receives the webhook. The "Negotiator Agent" checks Maya's availability calendar and recipe pricing guidelines.
  3. **Drafted Quote**: The Agent drafts a reply and a proposed Quote Card (Deposit: $50, Total: $150). It appears on Maya's OHC 375px feed.
  4. **One-Tap Approval**: Maya taps "Approve & Send".
  5. **Payment Collection**: The Agent replies to the customer with a native Stripe Payment Link. Upon payment, the Operations Agent schedules the task.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant MetaWebhook as Omni-Channel Webhook
      participant Inbox as Inbox Service
      participant Negotiator as Negotiator Agent (LLM)
      participant Quoting as Quoting & Ledger Service
      participant Stripe as Stripe Integration
      participant OwnerFeed as Owner Mobile Feed

      Customer->>MetaWebhook: "Need a quote for..."
      MetaWebhook->>Inbox: Ingest Message
      Inbox->>Negotiator: Analyze Intent & Context
      Negotiator->>Quoting: Draft Quote & Deposit terms
      Quoting-->>Negotiator: Quote ID
      Negotiator->>OwnerFeed: Push Actionable Card (Approve Quote)
      OwnerFeed->>Quoting: Owner taps "Approve"
      Quoting->>Stripe: Generate Payment Link (Idempotent)
      Stripe-->>Quoting: URL
      Quoting->>Inbox: Dispatch to Customer
      Inbox->>Customer: "Here is your quote: [Link]"
  ```

  ### AI Agent Integration Points
  - **Negotiator Agent**: Hooked into `omni_inbox_webhook.rs`. Needs tools to call `CreateInvoiceDraft` and `GeneratePaymentLink`.
  - **Operations Agent**: Listens for the `PaymentEvent` from Stripe to transition the order from "Draft" to "Scheduled" and block calendar time.

  ### Key Design Decisions
  - **Idempotency**: All Stripe Payment Link generation must use an idempotency key tied to `QuoteID + RevisionNumber` to prevent double-billing if the network is flaky.
  - **State Machine**: Quotes have strict state transitions: `Draft -> Pending Approval -> Sent -> Deposit Paid -> Completed`.
  - **Zero Trust Isolation**: Tenant isolation on the quoting tables using row-level security or application-level `tenant_id` checks.

  ## Implementation Prompt
  **User-Facing Outcome**: Maya receives actionable "Quote Approval" cards in her feed for incoming DMs.

  **CUJ**:
  1. A new DM arrives asking for a custom service.
  2. The system auto-generates a Quote Draft based on business context.
  3. The owner clicks "Approve" on the mobile feed.
  4. The system replies to the DM with a checkout link for the deposit.

  **Acceptance Criteria**:
  - Implement a `Quote` data model and repository (must handle deposits/splits).
  - Implement the Negotiator Agent logic to output structured Quote tool calls.
  - Expose an endpoint for the mobile frontend to "Approve" and dispatch the quote.
  - Ensure 100% unit test coverage for the Quote state machine.
  - Add Playwright E2E tests simulating the owner approving a quote in the mobile view.

  **Top 5 Codebase Quirks to Fix Later**:
  1. `src/server/api/ledger.rs` uses floating-point `f64` for currency math (`amount = unit_price * quantity as f64`). This must be refactored to use integer cents or a precise decimal library to prevent rounding errors.
  2. `src/server/domain/repository/ledger_repo.rs` contains a duplicated test `test_apply_payment_with_split`.
  3. `src/server/ohc/mod.rs` includes proto files via `tonic::include_proto!` without explicit directory mapping, which can be fragile.
  4. Missing `bazel` / `bazelisk` binaries in the default environment despite instructions mandating their use.
  5. Inconsistent use of UUID strings vs strongly typed IDs in domain models.

  ## Priority & Scope
  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
