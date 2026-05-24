issue_title: "Invisible Dynamic Pop-Up & Geo-Commerce Orchestration Engine"
issue_description: |
  # Research Report: Invisible Dynamic Pop-Up & Geo-Commerce Orchestration Engine

  We conducted research to identify structural gaps holding back our core personas (like Priya, the boutique owner, and Fatima, the food cart operator) who frequently operate in temporary, offline, and dynamic locations (pop-ups, farmer's markets, festivals).

  ## Findings
  - **Market Gap**: Current platforms (Shopify POS, Square) treat pop-ups as permanent locations, adding administrative overhead to configure taxes, inventory allocation, and discoverability. They fail to support ephemeral, geo-fenced commerce elegantly.
  - **The OHC Opportunity**: Create a "Geo-Commerce Orchestration Engine" to instantly launch a temporary storefront node. This involves 1-tap inventory splitting, AI-driven location broadcasting (social media/website banner), localized tax handling, and an offline-resilient POS mode.

  ## Proposed Next Steps
  - Implement `PopupSession` and `InventoryAllocation` entities with strict multi-tenant isolation.
  - Build the "Inventory Splitter" logic to securely ring-fence stock during a pop-up.
  - Develop a mobile-first (375px) "Start Pop-Up" UX Flow: location auto-detect, 1-tap inventory selection, auto-social broadcast, and offline POS mode.
  - Coordinate the Marketing, Operations, and Finance AI agents to automate location promotion, inventory reconciliation, and local tax compliance.

  Detailed design document and Mermaid diagrams have been generated in `docs/research/[architecture]_invisible_dynamic_pop_up_orchestration_engine.md`.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
