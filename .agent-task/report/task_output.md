issue_title: "OHC Edge Sync & Optimistic POS Architecture"
issue_description: |
  **Mission Queue Protocol Report**

  **Problem Statement:**
  Small business operators (like Priya, the boutique owner, or Carlos, the field service owner) need an inventory and point-of-sale (POS) system that remains highly responsive, even when networks are flaky. Double-booking must be prevented without making the user wait for slow network calls on every action.

  **Research Report:**
  We identified a critical gap: when the network drops, traditional web-based POS systems freeze. Conversely, fully offline POS systems create massive reconciliation headaches (double-selling) when they reconnect. OHC needs an architecture that handles edge caching, optimistic UI updates, and intelligent background synchronization, managed invisibly by the Operations Agent.

  **Design Doc:**
  - Architecture:
    - Implement a Redis-backed Distributed Lock Manager (`RedisRedlock`) for reserving inventory items during checkout.
    - Create a local IndexedDB/SQLite cache for the mobile Flutter client to store the active product catalog and offline-queued transactions.
    - Build a Sync Gateway in the Rust backend to handle reconciliation of offline transactions when the client reconnects.
  - Mobile UX Flow:
    - 375px viewport optimized POS screen.
    - Large 44x44px touch targets.
    - When offline, a subtle indicator appears, but the checkout button remains active. Transactions are queued locally.
  - AI Agent Integration:
    - The Operations Agent ("The Manager") receives conflict events from the Sync Gateway (e.g., item sold out online while an offline POS transaction was queued). It drafts an alert and proposes a resolution (e.g., "Cancel online order" or "Refund POS order") for the owner to approve via an Action Card.

  **Implementation Prompt:**
  1. Build the Redis Redlock reservation mechanism in the Rust backend for inventory items.
  2. Implement the Sync Gateway endpoint to receive offline-queued POS transactions.
  3. Wire the Operations Agent to handle inventory conflict events and generate Action Cards.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
