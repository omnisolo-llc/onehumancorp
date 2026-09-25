issue_title: "OHC-01: True Provider Invoicing and Reconciliation"
issue_description: |
  **Title**: OHC-01: True Provider Invoicing and Reconciliation

  **Problem Statement**:
  Currently, when OmniSolo generates an invoice, it fakes the payment link by returning a UUID-based URL without actually establishing a payment session with Stripe (or another provider). To support a real business, the AI team must create a valid checkout session, correctly record payment intent, and only mark the invoice as "paid" after verifiable reconciliation of provider webhooks, preventing fabricated success.

  **Research Report**:
  - **Baseline Investigation**: Code audit of the invoice API reveals that the codebase manually creates a `checkout-looking UUID` rather than establishing an actual checkout URL. Conversely, proposal intake does properly invoke the payment provider client.
  - **Competitive Comparison**: Products like HoneyBook and Jobber establish payment sessions and await verified webhook outcomes before showing the task as complete. Currently, OmniSolo has disjoint logic between proposals and invoices.
  - **Market Evidence**: As discussed in the business capability audit (OHC-01 target ID), an invoice must not claim payment success until actual funds are reconciled.
  - **Sources**:
    - [HoneyBook Pricing and Automations](https://www.honeybook.com/pricing) (Accessed 2026-09-18)
    - [Jobber Feature Overview](https://www.getjobber.com/features/) (Accessed 2026-09-18)
    - [Stripe Checkout Fulfillment Docs](https://docs.stripe.com/checkout/fulfillment) (Accessed 2026-09-18)
    - [Federal Reserve 2025 Report on Nonemployer Firms](https://www.fedsmallbusiness.org/reports/survey/2025/2025-report-on-nonemployer-firms) (Accessed 2026-09-18)
    - [OECD Generative AI and SME Workforce](https://www.oecd.org/en/publications/generative-ai-and-the-sme-workforce_2d08b99d-en.html) (Accessed 2026-09-18)

  **Comparative Feature Matrix**:
  | Feature | OmniSolo (Current) | HoneyBook | Jobber | OmniSolo (Target) |
  |---------|--------------------|-----------|--------|-------------------|
  | Proposal Payment Integration | Yes | Yes | Yes | Yes |
  | Invoice Checkout URL | Fake UUID | Real Provider URL | Real Provider URL | Real Provider URL |
  | True Webhook Reconciliation | No | Yes | Yes | Yes |
  | Unpaid Status Persistence | No | Yes | Yes | Yes |

  **Persona-Specific User Journey Narrative**:
  Nora is a solo web/design/marketing professional. When she creates an invoice for a completed milestone, she expects her AI team to handle the billing smoothly. She describes the completed work to OmniSolo, which generates a real payment request that the client can immediately pay using an integrated provider (like Stripe). OmniSolo tracks this invoice as "unpaid" and waits. When the client pays, a secure webhook confirms the payment. OmniSolo then automatically updates the invoice to "paid", records the transaction in the ledger, and notifies Nora that she got paid—all without Nora manually checking her bank account.

  **Design Doc**:
  - **Entities**: Invoice, PaymentSession, ProviderWebhookEvent.
  - **Key Relationships**:
    - Invoices link to exactly one PaymentSession (e.g., Checkout Session ID).
    - ProviderWebhookEvent must deduplicate events per PaymentSession.
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
        actor User as Nora (Owner)
        actor Client as End Client
        participant OHC as OmniSolo Backend
        participant Provider as Payment Provider

        User->>OHC: Create Invoice for Milestone
        OHC->>Provider: Request Checkout Session
        Provider-->>OHC: Return Checkout URL & Session ID
        OHC->>OHC: Persist Invoice (Status: Unpaid, Store URL)
        OHC-->>Client: Send Invoice Link
        Client->>Provider: Pays via Checkout
        Provider-->>OHC: Webhook Event (Payment Success)
        OHC->>OHC: Deduplicate Event & Verify Signature
        OHC->>OHC: Update Invoice (Status: Paid)
        OHC-->>User: Notification: Invoice Paid
    ```
  - **UI Flow**: User generates an invoice -> OmniSolo fetches real provider checkout URL -> UI displays the link. Payment completion relies on backend webhook reception to transition the invoice to a "Paid" state. Mobile view will show a pending indicator until the webhook fires.
  - **AI Integration**: The agent feeds should not report an invoice as "paid" simply because it was sent. The verifier agent will monitor the invoice status table and only log a successful outcome when the status updates via the payment provider hook.

  **Implementation Prompt**:
  Refactor the invoice creation logic to replace the UUID-based fake checkout URL with a real call to the payment provider client to create an idempotent checkout session. The backend must persist the true provider checkout URL and checkout session ID in the invoice database table. Ensure that the invoice status remains "unpaid" until an explicit webhook event reconciles it. Do NOT prescribe specific database schemas, API contracts, or function signatures. The success criteria include testing with test mode to verify that the URL is usable and that the webhook can transition the invoice state to "paid".

  **Priority**: P0

  **Estimated Scope**: Medium

  **Strategy Admission**: OHC target ID OHC-01; Run stage; observed codebase gap; evidence level: code audit; baseline: fake URLs; non-goals: inventing a new payment provider; authority class: routine external work under standing policy; cost plan: use test mode; happy-path checks: checkout URL is valid; failure-path checks: provider failure results in explicit unavailable state.

  **Workflow Provenance**:
  - Loaded skills: `skills/using-superpowers/SKILL.md`, `skills/brainstorming/SKILL.md`
  - Upstream repo: https://github.com/obra/superpowers/
  - Revision: 5bf4e78011075bcfc0dc295f0724994cd123ee71
  - Completed checks: Code audit, file verification (`git status`, `git diff --cached`).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
