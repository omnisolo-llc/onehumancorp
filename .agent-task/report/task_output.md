issue_title: "[Research] Offline-Tolerant Mobile POS & Agentic Inventory Sync Architecture"
issue_description: |
  # Research Report: Offline-Tolerant Mobile POS & Agentic Inventory Sync Architecture

  ## Problem Statement
  Small business operators like **Priya (Boutique Operator)** and **Fatima (Food Cart Operator)** manage inventory that is sold simultaneously online and in-person. Current OHC architecture lacks a robust, offline-tolerant mechanism to synchronize inventory states. When an in-store transaction occurs, especially in environments with spotty mobile data (e.g., street fairs, pop-ups, basements), POS terminals can fall out of sync with the online storefront. This leads to double-booking, selling out-of-stock items, and manual reconciliation nightmares. They need an architecture that seamlessly locks inventory across both channels in real-time when online, and queues/resolves offline POS transactions gracefully without technical intervention.

  ## Research Report & Gap Analysis
  **Competitive Landscape:**
  - **Shopify POS:** Offers excellent omnichannel sync but requires a persistent internet connection. Offline mode is available only on Pro plans ($89/mo) and often results in negative inventory balances that the owner must manually resolve.
  - **Square:** Strong offline payment queuing, but the inventory sync is delayed until the device reconnects. It lacks an AI agent to proactively notify online customers or automatically adjust online listings when stock is suspected to be low based on offline queued data.
  - **Wix / Squarespace:** Very weak offline capabilities; mostly rely on real-time webhooks.

  **OHC Gap:** OHC currently lacks an edge-capable local cache combined with an eventual consistency protocol for offline POS operations. Furthermore, there is no AI agent coordination to handle the fallout of a sync conflict (e.g., an item sold offline is simultaneously bought online).

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  sequenceDiagram
      autonumber
      actor BuyerOnline as Online Customer
      actor Priya as Priya (Mobile POS)
      participant POSClient as Flutter POS Client (Local DB)
      participant OHCAPI as OHC API Gateway
      participant Redis as Redis Redlock (Distributed Lock)
      participant Postgres as PostgreSQL (Central Ledger)
      participant OpsAgent as Operations AI Agent

      Priya->>POSClient: Add "Blue Dress" to Cart (Offline Mode)
      POSClient->>POSClient: Optimistic Local Lock & UI Update
      Priya->>POSClient: Tap-to-Pay & Finalize
      POSClient->>POSClient: Store Offline Transaction in Local Queue

      Note over POSClient, OHCAPI: Network Restored
      POSClient->>OHCAPI: Sync Offline Queue (Transaction payload)
      OHCAPI->>Redis: Attempt Lock on "Blue Dress"

      alt Lock Available (No Online Sale)
          Redis-->>OHCAPI: Lock Acquired
          OHCAPI->>Postgres: Deduct Inventory & Record Sale
          OHCAPI-->>POSClient: Sync Success
      else Lock Failed / Out of Stock (Sold Online)
          Redis-->>OHCAPI: Lock Denied / Negative Balance
          OHCAPI->>Postgres: Record Sync Conflict
          OHCAPI->>OpsAgent: Trigger Conflict Resolution Workflow
          OpsAgent->>Priya: Push Notification: "Inventory Conflict on Blue Dress. Online order takes priority. Shall I issue a refund for the POS sale or draft an apology to the online buyer?"
      end
  ```

  ### 2. UI Wireframes & Mobile UX Flow (375px First)
  **Screen 1: POS Checkout (375px)**
  - **Header:** Sticky top bar with a network status indicator. If offline, a subtle yellow icon "Offline - Changes will sync later".
  - **Body:** Large, tap-friendly product catalog cards (min 44x44px touch targets). Premium Apple/Ubiquiti translucent glass materials.
  - **Action:** Full-width bottom sticky button "Charge $50.00".

  **Screen 2: Sync Conflict Resolution (Triggered by Ops Agent)**
  - **Context:** If a double-booking occurs upon reconnection.
  - **UI Layout:** A clean UniFi-style modal.
  - **Headline:** "Inventory Conflict Detected"
  - **Body text:** "You sold a Blue Dress offline, but it was purchased online at 2:00 PM."
  - **AI Suggestions (Cards):**
    - Option A: "Refund in-store customer (Requires card)"
    - Option B: "Cancel & refund online order (Agent will draft apology email)"
  - **Footer:** "Decide Later" ghost button.

  ### 3. AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Listens to the `inventory.sync.conflict` event. Evaluates the timestamp of the offline POS transaction vs. the online transaction. Drafts a resolution strategy and pushes it to the owner's Work Triage feed.
  - **Customer Relationship Agent:** If the online order is canceled, this agent automatically drafts an apologetic email, optionally offering a discount code, and queues it for the owner's approval.

  ### 4. Key Design Decisions
  - **CRDTs / Eventual Consistency:** The mobile client (Flutter) will use a local SQLite/Drift database to store the catalog and queue transactions. It will push a transaction log rather than absolute inventory numbers to prevent overwriting online sales.
  - **Zero Technical Jargon:** The owner never sees terms like "Sync Error" or "Database Conflict". The Ops Agent translates technical conflicts into business decisions ("We oversold this item. Who gets the refund?").

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the offline-tolerant POS checkout flow and the backend synchronization protocol for OHC.

  **Acceptance Criteria:**
  1. Build a Flutter mobile UI (tested at 375px width) that allows adding items to a cart and completing a mock tap-to-pay transaction while disconnected from the internet. The UI must clearly but non-intrusively indicate offline status.
  2. Implement a local queuing mechanism in the client to store the completed transaction.
  3. Implement a backend gRPC/REST sync endpoint that processes the queued transaction upon reconnection.
  4. Integrate Redis Redlock in the backend to ensure inventory isn't double-booked during the sync.
  5. If the inventory was sold online while the POS was offline, generate an `inventory.sync.conflict` event.
  6. Extend the Operations Agent to listen for this event and create an actionable task in the owner's Triage Feed asking how to resolve it.

  Please ensure you write a Playwright E2E test simulating a network drop during a POS transaction, restoring the network, and verifying the sync behavior. Do not prescribe specific DB schemas; design them optimally for multi-tenancy.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
