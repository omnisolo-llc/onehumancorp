issue_title: "[Architectural Gap] Introduce High-Performance Edge-Cached Tap-to-Pay POS Synchronization"
issue_description: |
  # Mission Queue Protocol: Introduce High-Performance Edge-Cached Tap-to-Pay POS Synchronization

  ## 1. Problem Statement
  Currently, OneHumanCorp lacks a resilient, high-performance architecture for in-store Point-of-Sale (POS) operations that seamlessly integrates with our central inventory and multi-channel backend.
  Small business owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) struggle with slow transactions and unreliable internet connectivity, which hurts the customer experience and leads to lost sales. They need an instant, reliable tap-to-pay capability that works even under flaky network conditions while still maintaining inventory accuracy across online and offline channels.

  ## 2. Research Report
  - **Market Context**: Legacy systems often decouple online storefronts from physical POS terminals, leading to out-of-sync inventory and reconciliation nightmares. Modern unified commerce platforms (like Square, Shopify POS, Stripe Terminal) provide real-time or near-real-time synchronization.
  - **Identified Gap**: OHC requires an architectural foundation to support "always-on" offline-tolerant in-store transactions that securely cache critical product/pricing data at the edge (on the mobile device) and seamlessly synchronize state changes to the central backend.
  - **Key Competitive Insight**: The most critical capability for operators like Fatima is speed of transaction. A delay of seconds during peak hours is unacceptable. The architecture must decouple the local payment capture and receipt generation from the backend inventory sync.

  ## 3. Design Doc
  ### Architectural Overview
  The proposed architecture introduces an "Edge-Cached Local Ledger" model for OHC mobile clients:
  1.  **Edge Data Cache (Mobile)**: The OHC mobile app will maintain a local cache (e.g., SQLite or similar embedded DB) of the active product catalog, pricing, and tax rules for the current location.
  2.  **Offline-Tolerant Transaction Queue**: In-store sales (tap-to-pay) are recorded to a local transactional queue.
  3.  **Background Sync Engine**: A background worker on the device synchronizes the local queue to the OHC core backend asynchronously using exponential backoff.
  4.  **Backend Reconciliation**: The OHC core backend processes the incoming sync queue, updating central inventory, triggering AI agent workflows (e.g., low-stock alerts, accounting updates), and persisting to the master database.

  ### Mobile UX Flow (375px First)
  - **State**: The user opens the POS view. The top bar clearly indicates network status ("Online" or "Offline Mode").
  - **Action**: User selects items (from the cached local catalog) and taps "Charge".
  - **Feedback**: The local payment intent is initiated (via Stripe Terminal SDK or similar). Upon local success, an instant confirmation screen is shown.
  - **Background**: The transaction is added to the sync queue. A subtle, non-blocking indicator shows "Syncing X transactions..." in the background.

  ### AI Agent Integration Points
  - **Inventory Agent**: Monitors the backend sync queue and automatically drafts purchase orders or alerts the owner when stock drops below thresholds due to in-store sales.
  - **Finance Agent**: Reconciles the async POS transactions with the daily payout reports, highlighting any discrepancies.

  ## 4. Implementation Prompt
  **To the Implementer:**
  Implement the foundational backend data models and API endpoints to support asynchronous POS transaction synchronization for an offline-tolerant mobile client.
  - Define the data model for `POS_Transaction_Queue` to securely ingest and deduplicate offline sales.
  - Expose a secure, multi-tenant API endpoint (e.g., `/api/v1/pos/sync`) that accepts batched transaction payloads from the mobile client.
  - Ensure the ingestion process is robust, utilizing idempotent keys to prevent double-counting.
  - Integrate a background job (using our existing job queue) to process the ingested transactions and update the central inventory.
  - **Acceptance Criteria**:
    - The `/api/v1/pos/sync` endpoint successfully ingests valid transaction payloads.
    - Idempotency is proven: duplicate payloads do not result in duplicate inventory deductions.
    - Unit tests cover the ingestion and deduplication logic completely.
    - Create an E2E Playwright test simulating an owner using a mocked offline-to-online sync flow, ensuring inventory updates correctly on the dashboard.

  ## 5. Scope & Priority
  - **Priority**: P1 (High - unblocks critical physical retail persona)
  - **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
