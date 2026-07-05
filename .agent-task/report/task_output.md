issue_title: "Implement Offline-First AI Agent Work Queue & Sync Protocol for Mobile POS"
issue_description: |
  ## Title: Implement Offline-First AI Agent Work Queue & Sync Protocol for Mobile POS

  ## Problem Statement
  For operators in low-connectivity environments (e.g., Fatima the Food Cart Operator), relying on a strictly online-only connection to the central PostgreSQL and Redis cluster causes critical transaction failures. If the connection drops, she cannot process offline cash/tap sales or queue agent tasks (like sending a receipt or alerting the kitchen). The current architecture lacks a local-first caching and eventual-consistency queue on the mobile client that syncs transparently with the AI job queue when the connection is restored. This leads to lost sales and desynced inventory.

  ## Research Report
  - **Codebase Audit:** Currently, OHC heavily relies on direct PostgreSQL transactions and Redis Redlock (`ohc:lock:{tenant_id}`) for online booking and inventory. Data models assume consistent network availability and synchronous API responses.
  - **Competitor Systems Audit:** Shopify POS and Square Terminal cache catalog data locally and write transactions to a local SQLite/IndexedDB queue. When network connectivity is restored, they flush the queue in the background.
  - **The OHC Opportunity:** By extending the local-first queue to include "Agent Task Intents" (e.g., "Draft a follow-up for this customer"), OHC can allow the Operations and Sales agents to process tasks asynchronously without blocking the local checkout flow, maintaining the AI-assistant experience even in flaky network zones.

  ## Design Doc
  - **Architecture Diagram (Mental Model):**
    - Mobile Client (Flutter/PWA) -> Local SQLite / IndexedDB (Queue + Cache).
    - Network Restored -> Background Sync Worker -> gRPC / REST API -> Backend Sync Endpoint.
    - API Layer -> Postgres `SKIP LOCKED` Agent Job Queue.
    - AI Workers process jobs (e.g., Inventory deduction, SMS receipts, AI follow-ups).
  - **Mobile UX Flow (375px):**
    - The UI remains completely unblocked.
    - A subtle top-bar indicator shows "Offline - Changes saved locally" with a cloud-sync icon.
    - When tap-to-pay or cash is recorded, the button provides an instant optimistic success state ("Order #142 Saved").
  - **AI Agent Integration Points:**
    - The "Operations Agent" receives a batch of offline-sync events once the device is online.
    - If there's an inventory conflict (e.g., sold offline but also sold online), the Operations Agent drafts a resolution proposal for the owner instead of failing the transaction silently.
  - **Key Design Decisions:**
    - Favor optimistic UI updates for local POS transactions.
    - Guarantee Zero Trust and Multi-Tenancy by signing offline payloads locally and validating against the `tenant_id` at the edge API before enqueuing to the AI job queue.

  ## Implementation Prompt
  **User-Facing Outcome:** Fatima can continue to process pre-orders, cash sales, and update her menu availability while her phone is temporarily offline or in a 3G dead zone. The app feels instant, and AI tasks queue in the background to execute when connectivity returns.

  **CUJ & Acceptance Criteria:**
  1. The user logs into the app and the initial catalog data is fetched and cached.
  2. The network connection drops (offline mode). The user creates a new POS cash transaction. The UI must show success instantly with a local offline indicator.
  3. The network connection is restored. The client must automatically sync the transaction to the backend without user intervention.
  4. The backend API must validate and enqueue the synced transaction into the PostgreSQL job queue.
  5. The Operations Agent must process the transaction, deduct inventory, and the UI must update the sync indicator to "All systems go".

  **Constraints:**
  - Let the implementer design the exact schemas for the `OfflineSyncQueue` tables.
  - Implement at least five Playwright E2E tests simulating offline/online transitions and sync edge cases.
  - Ensure ZERO mock data is used; utilize real PostgreSQL schemas and local queue logic.

  ## PR Experience Evidence
  - **Persona:** Fatima (Food Cart Operator)
  - **Browser/Playwright Flow Tried:** Launched the UI using local docker compose stack. Opened the POS checkout view and toggled the browser network throttle to "Offline". Attempted to ring up a cash transaction.
  - **Observed CUJ Gap:** The UI threw a network error banner, blocked the transaction, and the user was unable to proceed. Real owners in fast-paced or low-connectivity environments cannot afford transaction blocks.
  - **User-Experience Reason for Fix:** Owners need guaranteed, unblocked order capture regardless of network state. Delaying the backend sync is acceptable; blocking the customer flow is not.
  - **Post-Fix Verification Plan:** Toggle offline mode, complete the transaction instantly in the UI, toggle online mode, and verify the background sync correctly persists the transaction to the backend database.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
