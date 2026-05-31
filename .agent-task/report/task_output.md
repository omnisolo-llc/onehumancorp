issue_title: "[Architecture] Edge-Caching Local-First Terminal Session Architecture"
issue_description: |
  # Architecture Research & Design: Edge-Caching Local-First Terminal Sessions

  ## Problem Statement
  Small business owners need reliable payment processing regardless of internet connectivity. Currently, OneHumanCorp (OHC) relies entirely on cloud connectivity for payment processing and inventory syncing. If the network drops, businesses halt. We need an edge-caching, local-first synchronization architecture to enable uninterrupted Tap-to-Pay and Point-of-Sale (POS) operations, guaranteeing that offline transactions are securely queued and instantly synchronized once connectivity returns. This enables true mobile-first autonomy for our core personas like Maya (baker at farmer's markets) and Carlos (handyman in basements).

  ## Research Report & Competitor Analysis
  Leading platforms like Square and Shopify POS utilize local-first architectures with robust sync queues to handle intermittent connectivity.
  - **Square:** Uses encrypted local SQLite with background synchronization, assuming the risk for offline authorizations up to 24h.
  - **Shopify POS:** Heavy reliance on React Native with local caching, though some discounting/inventory checks mandate a connection.
  - **OHC's Differentiation:** We will build a continuous local-first edge database (e.g., SQLite/IndexedDB) with CRDT-based conflict-free synchronization to OHC cloud via our AI Operations Agent. This enables seamless offline queuing with Zero-Touch configuration for the end user.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    subgraph Mobile Device
      UI[Mobile App 375px UI]
      LocalDB[(Local Edge DB SQLite/IndexedDB)]
      SyncEngine[Local Sync Engine]
    end
    subgraph Cloud Backend
      API[OHC API Gateway]
      Queue[(PostgreSQL Sync Queue)]
      Ledger[(Global Ledger)]
    end

    UI -->|Read/Write| LocalDB
    LocalDB --> SyncEngine
    SyncEngine -->|Background Sync when Online| API
    API --> Queue
    Queue --> Ledger
  ```

  ### UX & Mobile-First Flow (375px)
  - **Offline Indicator:** A subtle, premium glassmorphism pill at the top of the 375px viewport indicating "Offline - Saving Payments locally."
  - **Checkout Flow:** Uninterrupted Tap-to-Pay flow. The user taps, the app instantly shows "Payment Captured," and stores the encrypted payload locally.
  - **Syncing:** Upon network restoration, an invisible background task processes the queue. The offline pill fades out, and the ledger updates.

  ### AI Agent Integration
  - **Operations Agent:** Monitors the background sync queue. If a conflict arises (e.g., inventory oversold while offline), the agent automatically drafts a resolution strategy and notifies the owner via the Unified Inbox.
  - **Finance Agent:** Reconciles the batched offline transactions with the global ledger upon successful sync.

  ## Implementation Prompt
  Implement the Edge-Caching Local-First Terminal Session architecture.
  1. Define the Data Model and Invariants for the local-first queue and synchronization.
  2. Implement the background sync engine to push queued transactions from the local edge DB to the cloud API once connectivity is restored.
  3. Integrate the Operations Agent to handle conflict resolution automatically.
  4. Ensure all UX updates strictly adhere to the 375px mobile-first Glassmorphism design tokens.

  **Acceptance Criteria:**
  - A transaction can be successfully captured while the application simulates an offline state.
  - The transaction is persisted locally and automatically synced to the backend upon simulated network restoration.
  - Full end-to-end Playwright tests verifying the offline-to-online recovery flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
