issue_title: "Integration Research: Third-party Accounting and Billing (QuickBooks/Xero)"
issue_description: |
  # Mission Queue Protocol: Tool Integration Research

  ## Title
  Accounting and Billing Integration: QuickBooks Online & Xero

  ## Problem Statement
  OneHumanCorp's operating mandate centers around helping owners operate a sustainable business, capturing tasks like collecting/reconciling money and tracking profitability. The platform natively provides `api/invoice.rs`, basic ledger primitives (`payment_ledger.rs`, `ledger.rs`), and a Stripe connection (`api/tool_integrations.rs`) for collecting payments. However, most established and scaling small businesses (particularly those relying on physical goods or multi-channel retail) do not operate their core financial books within their workflow tool or POS alone; they rely on specialized platforms like QuickBooks Online or Xero to capture bills, manage payables, process payroll, file taxes, and act as their system of record for accounting (often mandated by their CPAs).

  Currently, we have placeholder folders for `quickbooks` and `xero` in `src/server/integrations/` with boilerplate Cargo and `provider.rs` structures, but no working auth, synchronization, or workflows. For an AI agent to reliably "Know cash and profitability" and correctly reconcile payments (as prescribed in the Active business-capability map), it needs to push native invoices/payments to the accounting system, pull expense and bill data, and reconcile clearing accounts without inventing a duplicate ERP.

  ## Research Report

  ### Target integrations
  - **QuickBooks Online (QBO)**
  - **Xero**

  ### Business Needs & Personas
  - **Solo service operators & digital merchants:** Need seamless syncing of paid Stripe invoices into QBO/Xero so their accountant doesn’t have to manually match Stripe deposits to sales.
  - **The "Know cash and profitability" loop:** Currently, `RESEARCH.md` tasks the agent to understand profitability. True profit cannot be determined merely by adding up Stripe invoices; it requires deducting operating expenses (software, inventory, contractors) which usually live in the core accounting ledger, not OHC.

  ### Integration Feasibility
  - **OAuth2 Flow:** Both QBO and Xero require OAuth 2.0. This fits cleanly into our emerging connection vault model (`connection_vault.rs` / `tool_integrations.rs`), but since these are user-delegated connections, they require redirect-based OAuth flows rather than static API keys (which `connection_vault.rs` currently focuses on storing, e.g. for OpenAI or Resend).
  - **Data Synchronisation Boundaries:**
    - **One-way sync (Initial scope):** OHC acts as the source of truth for Sales (Invoices, Proposals, Stripe Checkouts) and pushes them to QBO/Xero.
    - **Two-way read:** OHC reads Chart of Accounts, Tax Rates, and Expense totals to inform the AI agent’s profitability reports without replicating the entire ledger.
  - **Security & Authorization:** We must use the `connection_vault.rs` to store the OAuth Refresh tokens securely. The connection UI needs to handle the OAuth redirect callback securely.

  ### Pricing & Business Model Compatibility
  - These tools are customer-funded (BYOK/BYO Account). The customer pays for their QBO/Xero subscription. OHC simply orchestrates the data on their behalf using standard API quotas.

  ## Design Doc

  ### Integration with OHC Core Flow
  1. **Connection Vault Enhancement:** Expand `tool_integrations.rs` and `connection_vault.rs` to support OAuth 2.0 authorization code flow, not just static secrets. Store access tokens and refresh tokens per tenant.
  2. **Data Mapping Layer:** Create a canonical internal model for "Invoice" and "Payment" that can be mapped to both QBO and Xero schemas (which have different shapes).
  3. **Event-Driven Sync:** Hook into the existing `InvoiceServiceImpl` and webhook handlers. When an invoice transitions to "paid" via Stripe, dispatch an asynchronous job (using `nats` or `task_scheduler` already present in the codebase) to create the corresponding invoice/payment in the accounting ledger.
  4. **Agent Action Exfiltration:** Add MCP tools or directly accessible agent functions like `sync_accounting`, `get_profit_summary`, allowing the AI to answer: "What were our expenses last month?" by querying the external ledger.

  ## Implementation Prompt
  Extend the `tool_integrations` and `connection_vault` modules to securely support OAuth2 refresh token flows for accounting providers. Implement the QuickBooks and Xero providers in `src/server/integrations/quickbooks` and `src/server/integrations/xero`. Expose an interface that the Invoice service can call to asynchronously push paid invoices to the connected accounting platform. Do not build a full two-way sync; focus strictly on pushing closed sales and reading high-level profitability metrics. Ensure the tenant's connection state is accurately reflected in the `GetIntegrationsResponse`.

  ## Priority
  P2 (Medium)

  ## Estimated Scope
  Large

  ## Execution Evidence
  - Superpowers skill loaded: `using-superpowers` and `brainstorming`
  - Superpowers revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Codebase checks performed: `make test` (failed due to Next.js missing), codebase read via `cat` and `grep` commands
  - Outcome: Issue mapped to existing capabilities and structured for implementation
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
