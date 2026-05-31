issue_title: "[Architecture] Offline-First Mobile Sync Engine for SMB Resilience"
issue_description: |
  **Problem Statement**: Small business owners (e.g., Fatima running a food cart on a 3G network) need zero-latency, offline-capable mobile access to manage orders, inventory, and point-of-sale functionality. Current web-first platforms fail in poor connectivity environments, disrupting critical business operations.

  **Research Report**:
  - Competitors like Shopify and Wix require stable connections for core operations and treat mobile apps as secondary views of desktop capabilities.
  - OHC requires a mobile-first (375px viewport baseline) architecture.
  - Research highlights the need for an optimistic UI powered by a Local-First Sync Engine (e.g., SQLite/Isar on client syncing via gRPC with delta updates to PostgreSQL).

  **Proposed Next Steps**:
  - Implement a `HybridSyncEngine` in the backend to accept batched client deltas and resolve conflicts.
  - Design the data model for CRDTs or Last-Write-Wins timestamps.
  - Integrate with the existing multi-tenant PostgreSQL schema and AI job queue.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
