issue_title: "Implement Offline-First Mobile Tap-to-Pay POS with CRDT Sync"
issue_description: |
  **Problem Statement:**
  For OHC personas operating in the physical world (like Fatima the food cart owner, or Carlos the field service operator), reliable payment collection is the heartbeat of their business. A critical gap identified in our multi-channel platform is the reliance on continuous, high-quality cellular networks for taking payments and updating inventory. If Fatima’s cart is in a dead zone, she currently cannot process Tap-to-Pay or record offline orders, leading to lost revenue and customer frustration. The product must enable true offline-first operations where the mobile shell captures intent and payment details locally, then synchronizes seamlessly when connectivity returns.

  **Research Report:**
  Our review of leading POS competitors (Square, Toast, Shopify POS) indicates that "Offline Mode" is a major differentiator.
  - *Shopify POS* supports basic offline cash sales but severely limits offline card processing.
  - *Square* supports offline card processing by storing encrypted magstripe/chip data locally, which syncs and processes within 24 hours.
  - *Current OHC Implementation:* Requires a persistent connection to the Rust API server and Postgres instance to record orders and trigger Stripe Terminal API calls.
  - *The Gap:* OHC lacks a local, mobile-first Ledger/Order capability combined with secure Offline Tap-to-Pay buffering. To solve this, we need an architecture based on Local-First principles using CRDTs (Conflict-free Replicated Data Types) for state syncing (inventory/orders) and a secure local enclave for buffered payment tokens.

  **Design Doc:**

  *Architecture Diagram:*
  ```mermaid
  graph TD
      subgraph Mobile Client 375px Flutter PWA
          A[Order UI] --> B[Local CRDT Store]
          C[Tap-to-Pay Module SDK] --> D[Secure Token Buffer]
          B --> E[Sync Engine Offline/Online State]
          D --> E
      end
      subgraph OHC Cloud / Backend
          E -- mTLS / Websocket --> F[Mesh Handler Sync Gateway]
          F --> G[Distributed Ledger PostgreSQL]
          F --> H[Stripe Terminal / Payment API]
          F --> I[Operations Agent The Manager]
      end
  ```

  *Mobile UX Flow (375px first):*
  1. The user (Fatima) opens the OHC mobile app. The top status bar clearly indicates: "Offline Mode: Active (Saving orders locally)".
  2. She taps predefined visual product cards (e.g., "Falafel Plate") to build an order. Touch targets are large (minimum 44x44px).
  3. The cart totals instantly. She taps "Pay (Tap to Pay)".
  4. The screen transitions to a translucent Glassmorphism overlay prompting the customer to hold their card to the device.
  5. The mobile device reads the card securely. A quick green checkmark shows "Payment Saved - Will process when online".
  6. The order is committed to the local CRDT store, updating local inventory counts instantly so she doesn't double-sell.
  7. When connectivity is restored, a silent background sync reconciles the CRDT store with the server, processes the payment tokens, and triggers the Agent Feed to summarize the offline batch.

  *AI Agent Integration Points:*
  - **Operations Agent (The Manager):** Triggered when the offline sync completes. If any offline payments fail to capture (e.g., declined after sync), The Manager automatically drafts a follow-up SMS/Email to the customer (if known) or alerts the owner via an Action Card in the Agent Feed: "3 Offline Payments Failed to Sync. Review Details."
  - **Finance Agent:** Summarizes the batch of offline transactions in the end-of-day summary, highlighting any discrepancies caused by offline inventory conflicts.

  *Key Design Decisions:*
  - Adopt a robust CRDT-based local store on the mobile client to handle eventual consistency for inventory and orders without manual conflict resolution.
  - Use native device secure storage for buffering encrypted payment tokens. Do NOT store raw card details.
  - Prioritize absolute operational speed: building a cart and taking an offline payment must take fewer taps than a traditional cash register. No loading spinners while offline.

  **Implementation Prompt:**
  As an Implementer agent, your task is to build the core foundation for the Offline-First Tap-to-Pay POS.
  - Develop the `LocalSyncGateway` component in the Rust backend (`src/server/api/offline_sync.rs` or similar) designed to ingest and resolve CRDT payloads from the mobile client.
  - Establish the strictly isolated multi-tenant schema to accept offline order batches securely.
  - Integrate a background task queue (using existing mechanisms) that takes buffered payment tokens and attempts to capture them via the Stripe API once the sync payload is accepted.
  - Provide complete Unit and Playwright E2E tests validating the Critical User Journey: A mobile client connects, drops offline, submits two orders, reconnects, pushes the sync payload, and the backend correctly resolves the ledger and inventory state.
  - Ensure all database interactions strictly enforce `tenant_id` RLS policies. Do not define specific libraries for the frontend CRDT yet; focus on the API contract and backend resolution engine.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
