issue_title: "Architectural Gap & Scaling Discovery: Agentic Offline-Resilient Tap-to-Pay POS & Zero-Trust Sync"
issue_description: |
  ## Title
  Agentic Offline-Resilient Tap-to-Pay POS Architecture & Zero-Trust Sync

  ## Problem Statement
  Small business owners and operators (like Fatima the food cart owner or Carlos the handyman) frequently operate in environments with poor or zero cell service (farmers markets, basements, remote client sites). Currently, OHC's Terminal API requires an active network connection to process payments or sync inventory. When the network drops, operations halt. Competitors like Square offer robust offline modes, but they lack autonomous agentic recovery (e.g., automatically resolving inventory conflicts or following up on failed offline payments once reconnected). OHC needs an offline-first POS architecture that securely queues transactions and uses an Operations Agent to resolve state conflicts invisibly upon reconnection.

  ## Research Report
  **Market Gap:**
  - **Square:** Offers robust offline payments but leaves conflict resolution (e.g., overselling inventory) to the user via manual dashboards.
  - **Shopify POS:** Good offline capabilities, but heavy on the desktop/tablet paradigm and less suited for a 375px mobile-first flow without dedicated hardware.
  - **OHC Opportunity:** Implement an offline-first event-sourcing model on the mobile client (using SQLite/PowerSync). When a payment or order is taken offline, it's cryptographically signed and queued. Upon reconnection, the Operations Agent automatically resolves inventory conflicts, triggers The Ambassador Agent to email receipts for any delayed payments, and handles failed payment logic without owner intervention.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px] -->|Local Write| B(Local SQLite + PowerSync)
      A -->|Offline Tap-to-Pay| C[Stripe Terminal SDK Offline Mode]
      B -->|Background Sync| D[OHC API Gateway]
      D --> E{Zero-Trust Identity Verifier - SPIFFE/SPIRE}
      E --> F[Event Mesh / Redis Queue]
      F --> G[Universal Ledger DB]
      F --> H[Operations Agent]
      H -->|Resolve Inventory Conflicts| G
      H -->|Draft Recovery Emails| I[Customer Success Agent]
      I --> J[Agent Feed]
  ```

  ### Mobile UX Flow (375px First)
  - **Normal Operation:** Standard cart and "Tap to Pay" button visible.
  - **Offline State:** Network indicator turns amber ("Offline - Saving locally"). The "Tap to Pay" button remains active.
  - **Transaction:** User taps card. Payment is authorized offline via Stripe Terminal SDK. App shows a green "Payment Queued" checkmark.
  - **Reconnection:** Once back online, the amber indicator turns green ("Syncing...").
  - **Agent Intervention:** If a card declines after offline capture syncs, the Customer Success Agent automatically drafts an SMS/Email to the customer ("Hi, your card at Fatima's Food Cart couldn't be processed later. Here's a secure link to update payment.") and surfaces a 1-tap approval card to Fatima's Agent Feed.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the sync queue. If an offline order causes inventory to go negative (overselling), the agent automatically creates a backorder record and queues a supplier reorder draft.
  - **Customer Success Agent (The Ambassador):** Triggered by the Operations Agent if there are payment capture failures or necessary apologies for delayed fulfillment due to offline overselling.

  ### Key Design Decisions & Multi-Tenancy
  - **Event Sourcing:** The mobile app stores events (`OrderCreated`, `PaymentAuthorizedOffline`), not just end states.
  - **Zero-Trust Sync:** Each offline payload must be cryptographically signed using the device's secure enclave key, verified by the OHC API gateway to ensure tenant isolation (Row Level Security via `tenant_id`).
  - **Conflict-Free Replicated Data Types (CRDTs):** Use CRDTs for inventory counters locally to minimize hard conflicts during background sync.

  ## Implementation Prompt
  **User-Facing Outcome:** As an operator running a food cart in a dead zone, I can continue taking tap-to-pay orders seamlessly. I never see technical error messages. If a payment fails later, my AI assistant drafts an SMS to the customer that I just tap to approve.

  **CUJ & Acceptance Criteria:**
  1. The mobile UI allows completing a transaction while simulating offline mode (no network connectivity).
  2. The transaction is stored locally using the PowerSync/SQLite adapter.
  3. Upon simulating network reconnection, the offline queue syncs securely to the OHC backend.
  4. The backend verifies the device signature and processes the payment intent.
  5. If the payment is simulated to fail during sync, the Customer Success Agent automatically generates a draft message in the tenant's Agent Feed for review.
  6. **Playwright E2E Test:** Load the POS view in Playwright, disable network routing, complete a checkout flow, re-enable network routing, and verify the order appears in the backend ledger and Agent Feed without manual retries. Do NOT mock internal API calls; use the real stack.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []