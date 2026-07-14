issue_title: "OHC Inventory Local-First Offline Sync architecture"
issue_description: |
  # Research Report: Local-First Offline Inventory Sync & Conflict Resolution

  ## Problem Statement
  Small business owners relying on POS systems (like Priya the Boutique Owner or Fatima the Food Cart Operator) frequently experience network instability. When network connectivity drops, standard cloud-based POS systems fail to record sales or check inventory correctly, leading to lost sales, overselling (when the system is back online), and frustrated customers. A local-first POS system must be able to continue functioning (reading catalog, writing orders) while offline and seamlessly synchronize and reconcile inventory counts with the centralized ledger when connectivity is restored, resolving conflicts intelligently without confusing the owner.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square POS:** Strong offline mode capabilities allowing card payments (queued) and cash sales. However, complex inventory conflicts (e.g., selling the last item offline while it was sold online simultaneously) often require manual reconciliation or simply result in negative inventory.
  - **Shopify POS:** Offers offline cash transactions but lacks robust real-time synchronization strategies when offline sales conflict with online orders, particularly for low-stock items.
  - **Modern Local-First Architectures (Linear, Superhuman, CRDTs):** These apps use optimistic UI updates and local data stores (like IndexedDB/SQLite) to provide instant interactions. Changes are captured as an event log or CRDTs (Conflict-Free Replicated Data Types) and synced to the cloud.
  - **OHC Opportunity:** Implement a robust offline-first POS architecture for OHC using local caching (SQLite/IndexedDB via Flutter), operation intents (event sourcing), and CRDTs for inventory counters. When offline, POS transactions are recorded locally. Upon reconnection, the Operations Agent ("The Manager") handles reconciliation automatically, notifying the owner only if critical action (like cancelling an online order or restocking) is required due to simultaneous online/offline sales.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client - Flutter] -->|Reads/Writes| B(Local SQLite/IndexedDB)
      A -->|Network Up| C[API Gateway]
      C --> D[Event Sourcing / Sync Engine]
      D --> E{Conflict Resolution Engine - CRDTs}
      E -->|Resolves| F[Central Ledger DB - PostgreSQL]
      E -->|Escalates| G[Operations Agent]
      G -->|Push Notification| H[Owner Feed - Mobile]
      G -->|Auto-Reply| I[Online Customer / Storefront]
  ```

  ### Mobile UX Flow (375px First)
  - **Offline Indicator:** A subtle, non-intrusive status pill at the top of the POS screen indicates "Offline - Changes saved locally".
  - **Transaction Flow:** Processing a cash or queued card payment works instantly as if online. Inventory is deducted locally.
  - **Sync & Reconciliation:** Upon reconnect, a "Syncing..." spinner appears briefly.
  - **Conflict Notification:** If a conflict occurred (e.g., last item sold in-store and online while offline), an Action Card appears in the Agent Feed: "Conflict Resolved: 'Red Dress' was sold in-store while offline, but also purchased online. Online order #123 has been paused. Would you like me to email the online customer with a refund or backorder option?"

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Acts as the final arbiter for business-logic conflicts that CRDTs cannot solve alone (e.g., two people bought the exact same unique item). It decides which order takes priority (e.g., in-store possession usually wins) and drafts the resolution strategy.
  - **Customer Success Agent (The Ambassador):** If an online order must be cancelled due to a conflict, The Ambassador drafts a sincere apology and alternative offer for the online customer.

  ### Key Design Decisions
  - **Local-First Data Store:** Flutter app must use a robust local database to store catalog, pricing, and the queue of offline operation intents.
  - **CRDTs for Inventory (PN-Counters):** Inventory levels should be modeled as Positive-Negative (PN) Counters (increments/decrements) rather than absolute values to ensure eventual consistency without simple overwrite conflicts.
  - **Agentic Escalation:** Technical conflicts (database locks) are handled by the system. Business conflicts (overselling) are handled by the AI Agents drafting a resolution for the owner, never by showing database error codes.

  ## Implementation Prompt
  **User-Facing Outcome:** As Priya, when my boutique's Wi-Fi drops, I can still ring up a customer for a dress. When the Wi-Fi returns, the system silently syncs. If I sold the last dress in-store while someone bought it online, OHC automatically reserves the dress for the in-store customer and drafts an email apologizing to the online customer, presenting it to me for a 1-tap approval.
  **CUJ & Acceptance Criteria:**
  1. Define the required Flutter local database schema for offline operation intents (e.g., `offline_transactions` queue).
  2. Implement a CRDT-based inventory update endpoint in the backend that accepts a delta (e.g., `-1 Red Dress`) rather than an absolute value (`0 Red Dresses`).
  3. Create an E2E Playwright test simulating an offline POS sale: Disconnect network, process sale, reconnect network, verify sync to backend.
  4. Create an E2E Playwright test simulating a conflict: Item stock = 1. Process online sale (stock=0). Process offline POS sale (local stock=0). Sync offline POS sale. Verify backend detects oversell, sets stock to -1, and triggers the Operations Agent to create an Action Card in the feed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
