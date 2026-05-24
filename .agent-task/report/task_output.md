issue_title: "Implement Invisible Multi-Party Split Payments Ledger"
issue_description: |
  # Research Report: Invisible Multi-Party Split Payments Ledger

  ## Problem Statement
  For small business owners like Priya (boutique owner) who sells consignment items, and Carlos (handyman) who frequently sub-contracts tasks, managing multi-party payments is an administrative nightmare. Currently, they must take the full payment, manually calculate the split percentage, keep track of who owes what on a notepad or complex spreadsheet, and remember to manually send payouts. This causes tax confusion, delayed payouts, and endless friction.

  ## Findings & Analysis
  - **Market Gap**: Competitors like Shopify require complex third-party apps for splits, while Wix/Squarespace lack native split capabilities altogether, forcing offline workflows.
  - **OHC Differentiator**: Leverage the existing Hybrid AI OS and Teammate Mesh to build an invisible ledger that orchestrates split logic at checkout and automatically processes partner payouts. The Finance AI agent ("The Treasurer") handles ledger calculations asynchronously, keeping the checkout fast.

  ## Proposed Architecture
  - **Ledger Engine**: Add split allocations within the product data model and integrate splitting logic via events (`payment.captured`) handled by the Finance agent.
  - **Mobile-First UX**: A bottom sheet modal during product/invoice creation to easily define percentage splits with tagged contacts ("Split this payment").
  - **Action Feed Integration**: Notifications confirming automated splits ("Payment Received: $100. $70 routed to Sarah. Your cut ($30) is ready.").

  ## Implementation Plan
  - Read existing database schemas (`products`, `orders`) to introduce split metadata.
  - Update `FinanceAgent` (The Treasurer) to process split logic upon checkout events.
  - Expose API endpoints for configuring splits.
  - Ensure 100% test coverage and E2E validation for the Split Configurator on mobile UI and the background ledger settlement.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
