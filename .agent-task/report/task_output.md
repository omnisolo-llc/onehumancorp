issue_title: "[Architecture] Implement Hybrid-First Offline Sync Engine for Terminal & POS"
issue_description: |
  # OHC Architectural Deep Dive: Hybrid-First Offline Sync Engine

  ## Problem Statement (Persona Context: Fatima & Priya)
  Fatima runs a food cart and relies on low-end Android hardware with flaky or non-existent 4G connectivity depending on where her cart is parked. Priya runs a clothing boutique and sometimes deals with network dropouts at her local mall while checking out customers via a point-of-sale terminal.

  Currently, OneHumanCorp (OHC) lacks a robust, hybrid-first synchronization protocol for edge devices. When a network connection drops, offline transactions cannot be reliably cached, sequenced, or resolved against the central ledger upon reconnection. This results in lost sales (Priya cannot process tap-to-pay offline), inventory double-booking (Fatima accepts pre-orders while POS sync is down), and ultimately loss of trust.

  A true "owner work assistant" must shield the owner from network transient failures. We need a robust architecture to handle offline-first caching, local conflict resolution, and background sync without the owner doing anything manual.

  ## Market Research & Competitor Landscape
  - **Shopify POS:** Relies on partial local storage but struggles deeply if the connection drops mid-transaction without their high-tier advanced caching mechanisms.
  - **Square:** The gold standard for offline mode. Square caches swipes locally, encrypts them, and syncs asynchronously when the connection restores within 24 hours.
  - **OHC Opportunity:** Implement an SQLite/IndexedDB-based local ledger on the Flutter edge client that queues mutations (transactions, inventory decrements) as immutable CRDT-like events, while the backend implements a deterministic, idempotent sync receiver capable of handling eventual consistency for multi-channel sales (online + offline).

  ## Architectural Design (System Deep Dive)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Mobile as OHC Mobile App (Flutter)
      participant Cache as Local SQLite (Edge Ledger)
      participant API as OHC Backend API (Rust/Go)
      participant Q as Background Job Queue (PostgreSQL SKIP LOCKED)
      participant DB as OHC Central Ledger (PostgreSQL)

      Note over Mobile,Cache: Offline State
      Mobile->>Cache: Create Transaction (Intent)
      Cache-->>Mobile: Local Success (Inventory Soft-locked)

      Note over Mobile,API: Network Restored
      Mobile->>API: Sync Offline Batch (Events)
      API->>Q: Enqueue Events with Idempotency Key
      Q->>DB: Apply CRDT / Resolve Inventory Conflicts
      DB-->>API: Reconciliation Result
      API-->>Mobile: Sync Success & Update Local State
  ```

  ### Data Model & Invariants
  1.  **TerminalSession (Edge & Central):** Requires strong state machine mapping: `INIT` -> `READY` -> `OFFLINE_MODE` -> `SYNCING` -> `RECONCILED` -> `CLOSED`.
  2.  **SyncEvent (Edge):** Immutable log of actions.
      - `id`: UUID (v7 for time-sorting)
      - `tenant_id`: UUID
      - `entity_type`: (e.g., "transaction", "inventory_reservation")
      - `entity_id`: UUID
      - `mutation_payload`: JSONB
      - `idempotency_key`: string
      - `timestamp`: UTC datetime
  3.  **Conflict Resolution Protocol (Server):**
      - Last-Write-Wins (LWW) is insufficient for inventory.
      - Use additive/subtractive deltas for inventory (e.g., `-1 item X`) instead of absolute values (`set item X to 4`).
      - In case of negative stock due to online/offline collision, the backend must flag the anomaly for the **Decision Assistant AI** to present an actionable resolution to the owner (e.g., "Refund order or substitute item?").

  ### AI Department Integration
  - **Operations Assistant:** Invisibly monitors the sync queue. If an offline transaction conflicts with an online order, it flags the issue.
  - **Finance Assistant:** Reconciles the final synced amounts into the daily ledger view without showing raw syncing data to the user.

  ### Mobile UX Flow (375px First)
  1.  **Header Indicator:** Subtle amber icon indicates "Offline Mode - Operating Locally".
  2.  **Checkout Flow:** Functions identically to online mode. A "Payment Saved Locally" toast replaces the standard success screen.
  3.  **Queue Visibility (Advanced):** Under a "Sync Status" settings panel, the owner can see "3 orders pending sync".
  4.  **Auto-Recovery:** When connectivity returns, the amber icon turns green briefly and vanishes. Pending syncs resolve silently in the background.

  ## Implementation Prompt (For Implementer Agent)
  **Objective:** Implement the backend synchronization API endpoints and the corresponding database schema (PostgreSQL) to ingest and process batched offline transactions.

  **CUJ (Critical User Journey):**
  1. An owner (Fatima) completes 3 offline sales.
  2. Network restores.
  3. The mobile app posts a batch of `SyncEvent` records to the backend.
  4. The backend securely ingests these events, respecting multi-tenant isolation.
  5. The backend enqueues them for background processing or processes them synchronously using idempotency keys to ensure no double-charging or double-inventory deduction occurs.

  **Acceptance Criteria:**
  - Create the `SyncEvent` (or similar) table in PostgreSQL with strict RLS (Row Level Security) by `tenant_id`.
  - Expose a new gRPC/REST API endpoint (e.g., `POST /api/v1/pos/sync`) to receive batches of offline events.
  - The endpoint must use idempotency keys. Repeated sync attempts with the same key must return success without mutating data twice.
  - Write comprehensive backend unit tests verifying idempotency and multi-tenant isolation.
  - Update any necessary Protobuf/API definitions.
  - *Note: Do not build the Flutter UI, focus entirely on the robust backend ingestion layer that the mobile app will eventually call.*

  ## Priority & Scope
  **Priority:** P1
  **Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
