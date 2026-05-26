issue_title: "Implement Offline-First Inventory Sync Mesh via CRDTs"
issue_description: |
  # Research Report: Offline-First Inventory Sync Mesh

  **Problem:** Mobile POS systems experience severe data corruption and overselling when returning online after taking offline transactions. Existing platforms (Square, Shopify) either block offline sales entirely or use naive syncs that require manual merchant intervention.
  **Opportunity:** OHC must guarantee data integrity utilizing an invisible CRDT-based synchronization mesh.

  **Design Architecture:**
  1. Embed local SQLite stores in the mobile POS app for zero-latency, local-first writes.
  2. Implement CRDT PN-Counters for inventory state to mathematically guarantee correct eventual consistency upon reconnecting.
  3. Deploy an OHC Cloud Sync Mesh API that merges state vectors.
  4. Integrate the Operations and Customer Service AI agents to automatically handle edge cases like post-merge negative inventory (overselling) by issuing refunds and sending personalized apologies.

  **Proposed Next Steps:**
  1. Design CRDT PN-Counter data structure in Rust backend.
  2. Implement bidirectional sync endpoint for local POS nodes.
  3. Wire the CS Agent to the negative-inventory exception event.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []