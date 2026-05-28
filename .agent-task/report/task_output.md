issue_title: "[Architecture] Autonomous Multi-Location & Franchise Topology Engine"
issue_description: |
  # Multi-Location Topology Gap Analysis

  ## Findings
  Current small business platforms (Shopify, Wix, Square) lack an intuitive, mobile-first approach to scaling a business from a single location to multiple nodes. The friction of adding a second location, syncing inventory, sharing staff, and managing local tax compliance is a major pain point for our personas (Priya the boutique owner, Fatima the food cart operator).

  ## Proposed Next Steps
  We have detailed a new architectural design for an Autonomous Multi-Location & Franchise Topology Engine.
  This engine allows for 1-tap provisioning of new business nodes directly from the mobile app, orchestrated by the AI Operations Agent. It introduces a `LOCATION_NODE` entity while maintaining a `MASTER_CATALOG` at the tenant level, allowing for both aggregated "Empire" views and strict location-specific filtering and Zero-Trust isolation.

  The full design document, including Mermaid.js entity relationship diagrams and mobile UX flow, is available in `docs/research/[architecture]_autonomous_multi_location_and_franchise_topology_engine.md`.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
