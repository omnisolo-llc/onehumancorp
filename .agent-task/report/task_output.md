issue_title: "Implement Real-Time Multilingual KDS & Pre-Order Engine"
issue_description: |
  # Research Report & Findings

  Based on our analysis of competitor architectures (Square, Toast, Shopify POS) and the specific pain points of our non-technical food/beverage personas (e.g., Fatima, food cart operator), we have identified a critical gap: existing Kitchen Display Systems (KDS) require a constant internet connection and rely heavily on complex English-centric interfaces.

  Our proposed solution, the **Real-Time Multilingual KDS & Pre-Order Engine**, solves this by moving state management locally (SQLite) and leveraging a background sync daemon over a Hybrid Event Mesh. This guarantees sub-100ms UI responsiveness (optimistic updates) even in areas with poor cellular service, and enables native UI translation without round-tripping to the cloud.

  ## Next Steps
  1. Implement the local SQLite queuing mechanism on the mobile client.
  2. Implement the Hybrid Event Mesh sync daemon to push/pull pre-orders securely via SPIFFE/SPIRE.
  3. Build the highly legible, multi-lingual KDS view (375px first), focusing on massive touch targets and local optimistic updates.
  4. Ensure AI Agent integrations (Operations & Marketing) trigger correctly on state changes like "Sold Out".

  For complete architectural and UI flow details, see `docs/research/[architecture]_realtime_multilingual_kds_preorder_engine.md`.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, kds, offline-first]
assignees: []
