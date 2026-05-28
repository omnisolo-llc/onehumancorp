issue_title: "[Architecture] Autonomous Inventory Sync Engine"
issue_description: |
  # [Architecture] Autonomous Inventory Sync Engine

  ## Title
  Autonomous Inventory Sync Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) manage inventory across multiple channels—physical in-store sales and online storefronts. Keeping these in sync is manual and error-prone, leading to double-selling items or turning away customers unnecessarily. Existing platforms require complex POS setups or rely on delayed syncs. Priya needs an invisible, zero-config system where an in-store sale instantly and automatically updates her global online inventory, ensuring accurate stock levels everywhere.

  ## Research Report
  *   **Competitor Systems Audit**:
      *   **Shopify POS**: Offers good integration but requires expensive, proprietary hardware and a complex setup process.
      *   **Wix/Squarespace**: Often treat offline and online inventory as separate silos or require manual reconciliation, lacking true real-time hybrid synchronization.
      *   **Square**: Excellent at offline, but linking it tightly to a unified, multi-channel global catalog often requires third-party plugins.
  *   **OHC's Differentiation**: OHC's architecture uses a central `Universal Capacity Ledger` combined with edge caching. By building an autonomous sync engine directly into the mobile app's core POS capabilities, OHC can leverage CRDTs (Conflict-Free Replicated Data Types) to ensure that offline sales are queued and synced instantly when connectivity returns, with the AI Operations Agent handling any Edge vs. Origin reconciliation automatically.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE_POS ||--o{ OFFLINE_QUEUE : "Writes Offline Tx"
      ONLINE_STOREFRONT ||--o{ EDGE_CACHE : "Reads Stock"
      OFFLINE_QUEUE }|--|| SYNC_ENGINE : "Batches on Reconnect"
      SYNC_ENGINE ||--o{ CAPACITY_LEDGER : "Updates State securely"
      CAPACITY_LEDGER ||--o{ EDGE_CACHE : "Invalidates/Updates"
      SYNC_ENGINE ||--o{ AI_OPERATIONS_AGENT : "Alerts on Conflict"
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  *   **Visual Identity**: macOS-style Translucent Glass materials (`backdrop-filter: blur(20px)`) combined with clean Ubiquiti UniFi modular dashboard cards.
  *   **In-Store Sale (Offline)**: Priya processes a sale via Tap-to-Pay. The UI instantly shows a satisfying green checkmark. A subtle frosted glass pill at the top reads: "Saved offline. Will sync when connected." (Passes the Grandmother Test).
  *   **Sync Resolution**: When connectivity is restored, the app silently syncs. If a conflict occurs (e.g., the last item sold online while Priya was offline), the Operations Agent sends a plain-language push notification: "Priya, we double-sold the Vintage Silk Scarf. I've drafted an apology and a refund for the online buyer. Tap to approve."
  *   **Inventory Dashboard**: A clean, unified list showing global stock levels. No "Online" vs "In-Store" tabs, just the true available quantity.

  ### AI Agent Integration Points
  *   **Operations Department**: Monitors the `SYNC_ENGINE`. If there's a massive spike in offline transactions that fail to sync, the AI proactively investigates and temporarily flags high-risk inventory items. It also handles conflict resolution automatically by prioritizing in-store (offline) sales and handling online refunds/notifications.
  *   **Marketing Department**: Triggered automatically when the sync drops inventory below a threshold (e.g., "Only 1 Vintage Silk Scarf left"). It prepares a draft Instagram post: "Almost sold out!" for Priya to approve with 1 tap.

  ### Key Design Decisions (Why, not How)
  *   **Eventual Consistency with Conflict Resolution**: Designed around a CRDT or robust timestamp-based ledger to ensure offline sales never result in negative inventory online (or gracefully mark items "sold out online").
  *   **Offline-First Paradigm**: The UI must always assume success for reads and writes. Network requests are an asynchronous side-effect, not a blocker for the UI.
  *   **Zero Trust & Multi-Tenancy**: The sync engine authenticates via SPIFFE/SPIRE identity, ensuring the offline transactions are strictly bound to the specific merchant's tenant ID.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your objective is to architect the core logic and sync mechanisms for the "Autonomous Inventory Sync Engine."

  **Customer User Journey (CUJ):**
  Priya processes an offline transaction via her mobile POS. The system safely stores this transaction locally. When the network is restored, the system automatically synchronizes the inventory decrement with the global `Universal Capacity Ledger` and updates the edge caches for her online storefront. If a conflict occurs, the AI Operations agent handles it proactively.

  **Acceptance Criteria:**
  *   **Offline Queueing**: Implement a robust local queuing mechanism (using CRDTs or write-ahead logs) to safely store inventory events offline.
  *   **Idempotent Sync Engine**: Build the synchronization logic to ensure transactions are idempotent, preventing double-decrements if a network drop occurs during sync.
  *   **Edge Cache Invalidation**: The sync engine must trigger an event to update or invalidate the appropriate `EDGE_CACHE` keys when global inventory changes.
  *   **Conflict Resolution Hook**: Provide an interface for the Operations Agent to handle scenarios where eventual consistency results in an oversell.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
