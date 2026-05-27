issue_title: "Implement Autonomous Generative Merchandising Engine"
issue_description: |
  **Research findings**: We investigated the gap in merchandising capabilities for SMBs on current platforms (Shopify, Wix). Competitors require manual configuration, rigid rules, and extensive time investment. Small business owners lack the time and expertise to actively manage storefront layouts based on context.

  **Proposed next steps**: We designed the `GenerativeUIEngine` which will interface with the `MarketingAgent` and `InventoryLedger` to dynamically adapt the storefront UI in real-time. It will adjust hero messaging, promote high-margin or contextually relevant items (e.g., morning coffee vs evening cakes), and hide out-of-stock items, completely autonomously. This will run at the edge for sub-100ms load times and enforce strict `tenant_id` isolation.

  Detailed architecture and design are available in `docs/research/[architecture]_autonomous_generative_merchandising_engine.md`.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
