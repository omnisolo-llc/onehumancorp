issue_title: "Implement Zero-Touch Autonomous Bookkeeping and Tax Engine"
issue_description: |
  # Research Report: Zero-Touch Autonomous Bookkeeping and Tax Engine

  ## Findings
  Current small business platforms (Shopify, Wix) ignore back-office operations like bookkeeping and tax provisioning, forcing users into complex secondary tools like QuickBooks. Small business owners (like Maya the baker or Carlos the handyman) lack the time and expertise to manage chart of accounts, reconcile bank feeds, or calculate estimated tax withholding.

  ## Opportunity
  OHC has a strategic opportunity to implement an invisible bookkeeping engine powered by The Treasurer (Finance AI Agent) and the Universal Multi-Tenant Ledger. By integrating with bank feeds (e.g., Plaid) and automatically prompting users to snap receipts for ambiguous transactions, the system can categorize expenses and automatically reserve a portion of income into a "Tax Reserve" ledger without manual intervention.

  ## Proposed Next Steps
  1. Review the detailed design document at `docs/research/[architecture]_zero_touch_autonomous_bookkeeping_and_tax_engine.md`.
  2. Implement the Universal Ledger integration to support automated tax withholding on income events.
  3. Develop the Mobile-First UI (375px) for the "Daily Financial Brief" and "Receipt Scanner".
  4. Orchestrate The Treasurer AI Agent to analyze `bank.transaction.synced` events and categorize them or prompt the user for confirmation.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []