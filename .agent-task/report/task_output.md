issue_title: "Implement Multi-Channel Inventory Sync & Distributed POS Architecture"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Small business owners like Priya (Boutique Owner) and Carlos (Field Service Owner) struggle with double-booking and out-of-sync inventory when operating across multiple channels (online and in-person). They lack the technical expertise to integrate complex ERP systems. A seamless, real-time centralized inventory and distributed POS synchronization architecture is required to ensure online and offline sales never conflict.

  ## Research Report
  Our competitive analysis indicates that platforms like Shopify offer extensive POS integrations, but their implementation requires either higher-tier plans or expensive third-party plugins. OHC aims to capture the micro-SME market by providing an invisible, AI-driven automation experience. This gap in real-time, cross-channel inventory synchronization causes pain point #4 (Inventory Sync Across Channels), identified in our market research. By implementing a Redis-based distributed lock for checkout reservation (both online and tap-to-pay), we can guarantee consistency.

  ## Design Doc
  ### Architecture Design
  - **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth for all inventory counts.
  - **Distributed Locks (Redis Redlock)**: Used to reserve inventory during checkout to prevent double-booking. The lock key pattern will be `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client**: Mobile POS client caches catalog data locally, with eventual consistency for syncing offline sales when the network is restored.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      actor Customer
      participant POS Client
      participant Redis
      participant Database
      participant Agent

      Customer->>POS Client: Initiate Tap-to-Pay for "Item X"
      POS Client->>Redis: Request Redlock (`ohc:lock:{tenant_id}:inventory:{item_id}`)
      alt Lock Granted
          Redis-->>POS Client: Lock Acquired (15s TTL)
          POS Client->>Database: Finalize Transaction & Deduct Inventory
          Database-->>POS Client: Success
          POS Client->>Redis: Release Lock
          POS Client->>Agent: Notify of Sale
      else Lock Denied (Item reserved online)
          Redis-->>POS Client: Lock Denied
          POS Client->>Customer: "Item just sold out online"
      end
  ```

  ### Mobile UX Flow
  1. Priya processes an in-store tap-to-pay sale for the last item.
  2. The POS interface (optimized for 375px viewport) displays a rapid, optimistic checkout screen.
  3. Behind the scenes, a 15-second Redis lock reserves the item.
  4. Once finalized, Priya receives an Agent Feed notification (Action Card) if the item is out of stock, prompting a restock order with "Approve", "Edit", or "Discard" actions.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors stock levels and resolves sync conflicts. Triggers low-stock alerts and sends restock Action Cards to the user's mobile feed.
  - **Customer Success Agent**: Updates online storefront availability instantly based on in-store sales to prevent concurrent purchases.

  ## Implementation Prompt
  Implement the backend services for multi-channel inventory synchronization.
  1. Integrate Redis Redlock to handle inventory reservation during the checkout flow (both POS and online cart).
  2. Refine the backend checkout and transaction processing to update the central PostgreSQL ledger securely, releasing locks upon completion.
  3. Ensure the Operations Agent is hooked into the transaction success event to monitor stock levels and dispatch "Low Stock" Action Cards via the Agent Feed if necessary.
  Do not prescribe specific database schemas; focus on the distributed lock logic, checkout integration, and agent event triggering.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
