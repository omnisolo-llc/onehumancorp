issue_title: "[Feature] Mobile-First Omnichannel Sync (Tap-to-Pay)"
issue_description: |
  # Research Report: Mobile-First Omnichannel Sync (Tap-to-Pay)

  ## Title
  Mobile-First Omnichannel Sync Engine for Tap-to-Pay

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) need to seamlessly take in-person payments ("Tap-to-Pay") using their smartphones, while maintaining real-time inventory synchronization with their online storefronts. Currently, the mobile Tap-to-Pay infrastructure is isolated from the global multi-tenant inventory ledger, causing friction, out-of-sync inventory, and a risk of double-selling. Furthermore, businesses operating in areas with spotty cell service (like food carts) need an offline-first POS experience that gracefully syncs when connectivity is restored.

  ## Research Report
  - **Market Context:** Competitors like Shopify bundle this functionality through separate Point-of-Sale (POS) apps, while Square requires hardware dongles or treats e-commerce as secondary.
  - **OHC Differentiation:** OHC must provide a zero-config, invisible management experience. The smartphone is the entire operating system. Native Apple/Google Tap-to-Pay eliminates the need for external hardware.
  - **Architectural Gap:** OHC lacks a unified integration layer between the mobile terminal sessions (Tap-to-Pay SDK) and the real-time global multi-tenant database cache. We need an Optimistic Mutation Engine (local-first SQLite/CRDTs) that queues transactions offline and syncs them (< 500ms latency) to the global ledger when online, securely enforcing multi-tenant isolation.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph "Mobile Device (375px UI)"
          App[OHC App] --> Tap[Native Tap-to-Pay SDK]
          App --> LocalCache[(Local SQLite / CRDT Queue)]
          Tap --> LocalCache
      end

      subgraph "Edge & Cloud Platform"
          LocalCache -- "Background Sync (gRPC/Websockets)" --> Gateway[API Gateway]
          Gateway --> Ledger[Inventory Ledger Service]
          Ledger --> GlobalDB[(Global Inventory & POS DB)]
          GlobalDB -- "Webhook/Event" --> AIOps[AI Operations Dept]
      end
  ```

  ### Mobile UX Flow (375px baseline)
  - **Checkout:** User taps products from a visual catalog. App is instantly responsive (reading from local CRDT).
  - **Payment:** Taps "Charge $X" -> Bottom sheet for "Tap to Pay" or "Cash" -> Native Tap-to-Pay UI appears.
  - **Success/Offline:** Success toast ("Paid. Inventory updated.") or Offline toast ("Offline - Syncing later...").

  ### AI Agent Integration Points
  - **AI Ops:** Monitors the offline sync queue. Resolves ledger anomalies and flags high-risk inventory items during sync spikes.
  - **AI Marketing:** Triggers low-stock alerts and drafts "Almost sold out!" social posts when Tap-to-Pay deducts inventory below thresholds.

  ## Implementation Prompt
  **User-Facing Outcome:** Users can open the OHC mobile app, process an in-person payment via Tap-to-Pay, and see their global online inventory decrement instantly. The app works flawlessly offline, queuing sales and syncing automatically without exposing technical errors.

  **CUJ & Acceptance Criteria:**
  1. A transaction authorized via the mobile Tap-to-Pay SDK successfully deducts the corresponding SKU's inventory in the global `InventoryDB`.
  2. Implement an offline-first action queue (e.g., CRDT or local SQLite) so that if the network drops during checkout, the transaction is cached locally and synced securely to the global ledger when the network returns.
  3. The end-to-end sync must occur in < 500ms under standard network conditions.
  4. Multi-tenant boundaries must be strictly enforced at the Ledger level (Tenant ID validated on every sync request).
  5. The AI Operations queue must be notified via event bridge upon sync to trigger relevant automations (e.g., low stock).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
