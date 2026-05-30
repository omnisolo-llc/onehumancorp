issue_title: "[Feature] Implement Omnichannel Tap-to-Pay and Inventory Sync Engine"
issue_description: |
  # Research Report: The Omnichannel Sync Engine & Tap-to-Pay

  ## Problem Statement
  Small business owners such as Priya (Boutique Owner) and Fatima (Food Cart Operator) use OneHumanCorp (OHC) to run both digital storefronts and physical point-of-sale operations. Currently, our system lacks an integrated Tap-to-Pay terminal session handler that communicates with the global multi-tenant inventory ledger in real-time. This forces merchants to manually reconcile online stock after physical sales, which leads to double-selling and breaks the OHC core promise of "invisible management." Moreover, the physical POS must remain functional offline (e.g. food carts in poor cell service areas) with seamless eventual consistency upon network restoration.

  ## Research Findings
  - **Shopify & Square:** Both offer POS solutions. Square is deeply integrated but heavily relies on physical hardware dongles. Shopify POS requires a separate app. Neither provides the "invisible AI teammate" layer OHC aims for.
  - **OHC Technical Gap:** We currently have isolated components (`StripeClient` for online checkout, `MySyncService` for CRDT deltas, and basic inventory tracking via `inventory_count` in products). However, there is no end-to-end mechanism bridging a mobile Tap-to-Pay event directly to the global `InventoryDB` in under 500ms, nor is there a unified event bridge to the AI Operations Queue to trigger low-stock alerts.
  - **Proposed Solution:** Implement an Omnichannel Sync Engine that securely digests offline-first local cache transactions (CRDT sync) from the OHC Mobile App, processes the payment intent via Stripe Terminal API (or MercadoPago/Razorpay), and atomically updates the `InventoryDB`.

  ## Architecture & Design
  ```mermaid
  graph TD
      subgraph "Mobile Device"
          App[OHC Mobile App - 375px UI] --> Tap[Native Tap-to-Pay SDK]
          Tap --> LocalDB[(Local SQLite / CRDT)]
      end

      LocalDB -->|Background Sync (gRPC)| Gateway[API Gateway / SyncService]

      subgraph "Core OHC Platform"
          Gateway --> Ledger[Inventory Ledger Service]
          Ledger --> InventoryDB[(Global PostgreSQL DB)]
          InventoryDB -->|Webhook/Event| AI_Ops[AI Ops Dept: Low Stock Check]
      end
  ```

  ### Mobile UX Flow (375px baseline)
  1. Priya taps "New Sale" in her OHC app dashboard.
  2. She selects items from her catalog. The app calculates the total.
  3. She taps "Charge $XX.XX" -> "Tap to Pay". The native OS Tap-to-Pay interface intercepts the customer's card.
  4. On success, a toast notification confirms "Paid. Online inventory updated."
  5. If offline, the sale is queued, and the UI gently notes "Syncing when online...".

  ### Multi-Tenancy & Zero Trust
  All API sync requests must carry `x-spiffe-id` tokens, validating tenant isolation at the `SyncService` gateway before hitting the inventory ledger.

  ## Implementation Prompt (For Implementer Swarm)
  **Objective:** Implement the backend components of the Omnichannel Sync Engine, specifically the integration between terminal session processing, inventory deduction, and offline CRDT sync.

  **Acceptance Criteria:**
  1. Ensure the `SyncService` can process a new payload type representing a `PosTransaction`.
  2. Upon receiving a `PosTransaction`, the service must decrement the `inventory_count` in the `products` table for the corresponding `tenant_id`.
  3. Trigger an AI Operations job (via `sub_agent_queue`) if the new `inventory_count` falls below a threshold (e.g., 5).
  4. Ensure all database updates respect multi-tenant row-level boundaries (`tenant_id` validation).
  5. The logic must handle idempotency to ensure CRDT replays or duplicate syncs do not double-deduct inventory.
  6. Unit and E2E tests must be strictly implemented and pass `bazel test //...`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
