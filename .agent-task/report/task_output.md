issue_title: "[Architecture Design] Multi-Channel Inventory & Distributed POS Synchronization System"
issue_description: |
  # Multi-Channel Inventory & Distributed POS Synchronization System

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) run operations both online and in-store. Currently, managing inventory across these channels requires manual effort, and concurrent sales (e.g., a customer buys the last item online while another buys it in-store) lead to double-selling and customer dissatisfaction. OHC needs a centralized, highly consistent inventory system with distributed synchronization and agentic oversight to abstract this complexity.

  ## Research Report
  - **Market Context**: Shopify and Square offer inventory sync but require complex setup, high-tier plans, or third-party apps. Square excels at POS but is less integrated with robust online storefronts without heavy configuration.
  - **The Gap in OHC**: We currently lack a real-time inventory locking mechanism and an offline-tolerant POS sync protocol. We need a system that handles network flakiness, simultaneous cart checkouts, and in-person tap-to-pay without the owner ever seeing a "conflict resolution" screen.
  - **Persona Fit**: Priya needs to sell an item via tap-to-pay in-store and have the online store instantly reflect the new inventory count. If a network drop occurs, the app should sync the sale once reconnected and let the AI Operations Agent handle any rare conflicts (e.g., drafting a refund/apology email for the online buyer).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (Online/Offline)
      participant POS as OHC Mobile App (POS)
      participant R as Redis (Distributed Lock)
      participant DB as Central Ledger (Postgres)
      participant AI as Operations Agent

      C->>POS: Initiate Checkout (Item X)
      POS->>R: Acquire Lock (ohc:lock:{tenant_id}:inventory:{item_x})
      alt Lock Acquired
          R-->>POS: Lock Granted
          POS->>DB: Process Sale & Update Ledger
          DB-->>POS: Confirmation
          POS->>R: Release Lock
          POS-->>C: Sale Successful
      else Lock Denied (Concurrent Sale)
          R-->>POS: Lock Denied
          POS-->>C: Item Unavailable
      end

      opt Network Reconnection (Offline POS)
          POS->>DB: Sync Offline Sales Batch
          alt Conflict Detected (Double Sell)
              DB->>AI: Trigger Conflict Workflow
              AI->>DB: Reconcile Inventory
              AI->>DB: Draft Refund/Apology for Online Buyer
              AI->>POS: Notify Owner of Resolution
          end
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **POS Screen**: Clean, grid-based catalog with large (≥ 44x44px) tap targets. "In Stock" badges update optimistically.
  2. **Checkout Drawer**: Slides up from bottom. Large "Tap to Pay" / "Charge $X" button.
  3. **Offline State**: If network drops, a subtle, unobtrusive amber banner appears: "Offline - Saving sales locally". The checkout process remains identical.
  4. **Sync State**: Upon reconnection, a brief "Syncing X sales..." toast appears. If an AI agent resolved a conflict, an Action Card is placed in the main Agent Feed for owner review (e.g., "Review drafted apology for over-sold item").

  ### AI Agent Integration
  - **The Operations Agent ("The Manager")**: Monitors the sync queue. If a conflict occurs during offline sync, it analyzes the ledger, determines the discrepancy, and drafts a resolution plan (e.g., refunding the online customer and drafting an apology email). It presents this to the owner via the Agent Feed.
  - **The Customer Assistant ("The Ambassador")**: Updates online storefront availability dynamically.

  ### Key Design Decisions
  - **Central Ledger (PostgreSQL) + Redis Redlock**: Ensures strict consistency for online transactions and online-to-offline lock coordination.
  - **Eventual Consistency for Offline POS**: To maintain velocity for in-store sales, the mobile POS can process transactions offline and sync them to the central ledger upon reconnection.
  - **AI-Managed Reconciliation**: Instead of freezing the system or showing cryptic error messages, conflicts are handled asynchronously by the Operations Agent, which drafts a solution for the owner to approve.

  ## Implementation Prompt
  **User Facing Outcome:** Priya can sell items seamlessly in-store while the online storefront updates instantly. She never worries about double-selling. If she sells an item offline while disconnected from the internet, the app saves the sale and syncs it later.
  **CUJ:**
  1. Priya logs into the OHC app.
  2. She adds an item to the POS cart and processes a sale.
  3. The inventory for that item is decremented in the central ledger.
  4. Another user trying to buy the same item online simultaneously sees it is out of stock.
  **Acceptance Criteria:**
  - Implement the `Redis Redlock` locking mechanism for inventory updates.
  - Create the POS sync API endpoint capable of accepting batched offline sales.
  - Implement conflict detection in the sync endpoint.
  - Integrate the Operations Agent to trigger on conflict detection and generate an Action Card.
  - Develop comprehensive E2E Playwright tests simulating concurrent checkouts and offline sync scenarios.
  - Ensure all new UI components adhere to the OHC Premium Token library and are fully functional on a 375px viewport.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
