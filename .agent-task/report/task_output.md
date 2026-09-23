issue_title: "✍️ Scribe: Documentation gap in connected reporting and invoice reconciliation"
issue_description: |
  # Documentation Research Report: Invoice Reconciliation and Connected Reporting

  **Problem Statement:** Owners currently lack clear visibility into how invoices transition from draft to fully reconciled and collected states, and how this data feeds into connected reporting. The current system can produce a checkout draft (F08) and log a reminder (F09), but full provider payment event reconciliation and actual reminder delivery are not fully documented or verified. This gap leaves the owner unable to trace a payment confidently.

  **Research Report:**
  According to the `business_capability_and_usage_economics_audit.md` (accessed 2026-09-18) and `native_migration_and_remediation.md`:
  - **F08**: Invoice creation returns a draft without a real Stripe ID/URL. Full provider sandbox replay and payment event reconciliation are pending.
  - **F09**: Receivables code historically logged drafts before implementing delivery. Drafting is verified, but sending, delivery, and stopping on payment/cancellation are open.
  - **Business Responsibility Gap**: "Collect and reconcile money" requires provider IDs and receipts to agree with invoice balance and the ledger, with retries not double-charging.

  **Design Doc:**
  - **Mobile UX Flow**:
    1. Owner views a pending invoice on their dashboard.
    2. The status clearly indicates "Draft", "Sent", "Paid", or "Overdue".
    3. Tooltips explain that "Sent" means an email was delivered, not just drafted.
  - **AI Agent Integration Points**: The agent can surface overdue invoices and ask the owner if a reminder should be sent (if not fully autonomous), but the agent must read from a truthful status, not just a log entry.

  **Mermaid.js architecture diagram:**
  ```mermaid
  %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#ffcccc', 'edgeLabelBackground':'#ffffff', 'tertiaryColor': '#f0f0f0'}}}%%
  graph TD
      A[Invoice Creation] --> B(Draft Saved in DB)
      B --> C{Send Trigger}
      C -->|Authorized| D[Provider API Call - e.g. Stripe]
      D --> E[Status: Sent]
      E -.-> F[Webhook / Receipt]
      F --> G[Status: Paid & Reconciled]
  ```

  **Implementation Prompt:** Provide plain-language tooltips and a help center article explaining the difference between an invoice draft and a sent invoice, how to authorize a reminder, and where to verify that a client's payment has settled in the OHC ledger.

  **Priority:** P0 (Blocks clear understanding of cash collection)

  **Estimated Scope:** Moderate (Requires verifying the actual state flow and adding tooltips/docs)

issue_priority: "P0"
issue_category: "documentation"
issue_type: "research"
issue_label: "documentation"
assignees: []
