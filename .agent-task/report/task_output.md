issue_title: "Research: High-Performance Multi-Tenant Data Sync for Offline-First Mobile POS"
issue_description: |
  **Problem Statement**
  Small business owners like Carlos (Handyman) and Fatima (Food Cart) operate in high-distraction environments and often experience poor or completely disconnected cellular data (e.g. inside basements or at crowded events). Currently, if the network drops, they cannot reliably take tap-to-pay payments, update inventory (sold-out toggles), or access critical client booking data. A major market gap was identified where competitors like Shopify provide queued card transactions for POS, but web-first systems like Wix fail offline. We need a robust offline-first edge-caching and sync architecture that allows critical operations to function autonomously offline and seamlessly sync via the Hybrid Architecture (OHC-HA) when connectivity is restored.

  **Research Report**
  *   **Competitor Landscape**:
      * **Shopify POS**: Offers an offline mode for cash and queued card transactions that process upon reconnection.
      * **Square**: The leader in offline POS payments; it queues and encrypts transactions with a 24-hour sync window.
      * **Wix/Squarespace**: Heavily dependent on constant internet connections and browser caching, leaving a gap for reliable offline point-of-sale functionality.
  *   **OHC Internal Gaps**: According to `[architecture]_mobile_first_review.md` and `[research]_smb_platform_market_gap.md`, OHC suffers from "Inventory Sync Paralysis" and lacks "Native POS Integration". OHC’s hybrid nature (Local SQLite / SIPDB) is perfectly positioned for "Offline Drafting" and local-first execution.
  *   **Technology & Pattern Analysis**: Integrating CRDTs (Conflict-free Replicated Data Types) or local-first database sync protocols (like PowerSync or RxDB) over the existing local SQLite footprint enables seamless offline-to-online state transitions without major schema rewrites.

  **Design Doc**

  *   **Architecture Diagram (Mental Model)**:
      1.  **Mobile Client (Flutter/PWA)**: Reads and writes strictly to a local SQLite database acting as a local SIPDB mirror. It maintains an `ActionQueue` table for mutating operations (e.g. queueing payments, deducting local stock).
      2.  **Service Worker (PWA)**: Actively caches static assets (Glassmorphism CSS, fonts, product WebP images) to ensure sub-1.5s LCP even on 2G/offline networks.
      3.  **Sync Engine (SyncDaemon)**: A background daemon monitoring network state. When online, it pushes the batched `ActionQueue` payloads to the OHC Backend (`/api/v1/sync/push`) and pulls the latest multi-tenant delta.
      4.  **OHC Backend (Go)**: Receives batched actions. Uses Redis Redlock for distributed locking to resolve conflicts (e.g. parallel offline stock depletion) before committing to PostgreSQL, enforcing tenant-isolation boundaries.

  *   **Mobile UX Flow (375px First)**:
      *   **Offline Indicator**: A premium, translucent glassmorphism pill smoothly animates at the top: "Offline - Changes Saved Locally".
      *   **Tap-to-Pay Offline**: The primary payment CTA remains active but dynamically updates to "Queue Payment (Offline)" when network drops.
      *   **Inventory Management**: Toggles for "Sold Out" react instantly with optimistic UI updates and display a subtle "Sync Pending" icon instead of blocking with a spinner. Touch targets remain $\geq$ 44x44px.

  *   **AI Agent Integration Points**:
      *   **Finance & Payments ("The Accountant")**: Proactively analyzes queued offline payments to flag potentially high-risk transactions once the sync engine processes them.
      *   **Operations ("The Manager")**: Monitors inventory reconciliation. If an offline conflict results in an oversell, it autonomously drafts an apology and refund proposition to the affected customer.

  *   **Key Design Decisions**:
      *   **Optimistic UI Over Blocking States**: No spinning loaders waiting for network requests; the application must perform as responsively as a native calculator app.
      *   **Action Queueing Model**: Mutating actions are serialized into the local SQLite `ActionQueue`.
      *   **Conflict Resolution Strategy**: We will utilize a Last-Write-Wins (LWW) strategy for basic profile fields, but specific deductive merging logic for critical operations like inventory counts.

  **Implementation Prompt**
  "Implement the core local-first database and sync engine for the Flutter mobile client. Start by integrating a local SQLite database and an `ActionQueue` table for tracking offline mutations. Create the background Sync Engine that monitors network connectivity, flushes queued actions to a new Go backend endpoint (`/api/v1/sync/push`), and pulls state updates. Update the POS and Inventory UI to support optimistic state updates, and display a clear 'Offline' indicator when disconnected. Develop unit tests for the sync logic and a Playwright E2E test that explicitly simulates a network disconnect, processes a local inventory update, and verifies successful synchronization upon reconnection."

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
