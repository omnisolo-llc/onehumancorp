issue_title: "[Architecture] Universal Multi-Tenant Offline-First Mobile Tap-to-Pay and POS"
issue_description: |
  ## Problem Statement
  Small business operators like Priya (Boutique Operator), Fatima (Food Cart Operator), and Carlos (Field Service Owner) frequently transact in person. They struggle with fragmented systems where their in-person Point-of-Sale (POS) is disconnected from their online inventory and booking systems. Furthermore, poor network connectivity at events, food cart locations, or in the field disrupts payments. Existing solutions (Square, Shopify POS) often require dedicated hardware or separate apps, creating operational silos that non-technical owners must manually reconcile. OHC lacks a unified, offline-tolerant mobile Tap-to-Pay POS architecture that natively syncs with our centralized AI inventory and operations mesh.

  ## Research Report
  - **Market Landscape:** Competitors like Square, Stripe Terminal, and Shopify POS dominate the in-person payment space. However, they are either isolated payment processors or require heavy ecosystem lock-in. Stripe Terminal provides Tap to Pay on iPhone/Android SDKs, which allows regular smartphones to act as POS terminals without extra hardware.
  - **The Gap:** There is a gap for an "Invisible AI Automation" POS. Existing POS systems don't have autonomous agents that immediately reconcile offline payments, update centralized inventory, draft follow-up review requests (Customer Success Agent), and adjust financial ledgers (Finance Agent) the moment connectivity is restored.
  - **Target Personas:**
    - *Fatima (Food Cart):* Needs offline-tolerant order capture and payment processing when cell service is overloaded during lunch rushes.
    - *Priya (Boutique):* Needs in-store tap-to-pay that instantly deducts from the same inventory pool as her online storefront to prevent double-selling.
    - *Carlos (Field Service):* Needs to take final payment on his Android phone in the field, triggering automatic invoice marking and next-job routing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      subgraph Mobile Client (Flutter PWA / Native)
          UI[375px POS Interface]
          LocalDB[(Local SQLite/Isar Cache)]
          Sync[Optimistic Sync Engine]
          TTP[Stripe Tap-to-Pay SDK]
          UI <--> LocalDB
          UI <--> TTP
          LocalDB <--> Sync
      end

      subgraph OHC Backend
          API[gRPC / REST API]
          Queue[PostgreSQL Job Queue]
          DB[(PostgreSQL - Multi-Tenant)]
          StripeWebhooks[Stripe Webhook Handler]
      end

      subgraph AI Departments
          Ops[Operations - Inventory Sync]
          Fin[Finance - Ledger Update]
          CS[Customer Success - Receipts/Follow-ups]
      end

      Sync <-->|Background Sync via WiFi/Cell| API
      API --> Queue
      Queue --> DB
      Queue --> Ops
      Queue --> Fin
      Queue --> CS
      StripeWebhooks --> API
  ```

  ### Mobile UX Flow
  1. **Cart & Checkout (Offline-Tolerant):** The user taps items or enters a custom amount on their phone (375px mobile-first layout). The UI is clean, utilizing OHC Premium translucent glass materials.
  2. **Payment Selection:** Taps "Charge". The system checks network status. If offline, it allows recording a cash transaction or queuing an offline-approved transaction (if risk profile allows). If online, it launches the native Tap-to-Pay interface.
  3. **Tap to Pay:** The customer taps their NFC card/phone to the owner's device.
  4. **Optimistic Success:** UI instantly shows a green success checkmark and offers to send a digital receipt via SMS/Email.
  5. **Background Sync:** The Sync Engine queues the transaction locally and flushes it to the OHC Backend when connectivity is restored, transparent to the user.

  ### AI Agent Integration Points
  - **Finance (The Accountant):** Automatically categorizes the in-person sale and updates the daily revenue summary once the sync completes.
  - **Operations (The Manager):** Instantly deducts sold variants from the global multi-tenant inventory to prevent online overselling.
  - **Customer Success (The Ambassador):** Uses the customer's digital receipt contact info to proactively send a "Thank you" and a request for a review, without owner intervention.

  ### Key Design Decisions
  - **Local-First Data Model:** Use a local database (SQLite/Isar) in the Flutter client for the product catalog and cart state, enabling uninterrupted use without a network.
  - **Optimistic Mutation:** Treat the local device as the source of truth for POS state until synchronized, using UUIDs generated on the client to avoid collision.
  - **Stripe Terminal Integration:** Utilize Stripe's SDKs to handle the secure element and PCI compliance of Tap-to-Pay, keeping sensitive data out of the OHC backend.

  ## Implementation Prompt
  **Task:** Implement the foundational Offline-First Mobile POS and Tap-to-Pay architecture for OHC.
  **CUJ:** Fatima (Food Cart Operator) is at a crowded event with spotty cell service. She selects a "Falafel Wrap" from her catalog on her Android phone, taps "Charge", and the customer pays via Tap-to-Pay. The UI instantly registers success. When Fatima gets back to a good connection, the app silently syncs the order to the backend. The Operations Agent updates inventory, and the Finance Agent updates her daily summary.
  **Acceptance Criteria:**
  - Implement the local SQLite/Isar caching mechanism in the Flutter client for the product catalog and offline transaction queue.
  - Build the background sync engine that pushes queued transactions to the backend API when network connectivity is restored.
  - Define the API endpoints for POS synchronization, ensuring multi-tenant isolation (`tenant_id` validation).
  - Add backend event triggers so that synced POS orders dispatch jobs to the Operations and Finance AI Agents.
  - Playwright/Browser UI tests must simulate the offline state (disconnecting network), placing an order, and verifying the optimistic UI success, followed by reconnecting and verifying the backend sync.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
