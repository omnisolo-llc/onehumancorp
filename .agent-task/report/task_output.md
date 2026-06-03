issue_title: "Implement Autonomous Multi-Channel Inventory & Fulfillment Engine"
issue_description: |
  **Problem Statement:**
  Small business owners selling physical goods across multiple channels face a persistent threat: double-selling. Priya (the boutique owner) sells clothing in her physical store, on her OHC online storefront, and sometimes at weekend pop-up markets. If she sells a specific blue dress (Size M) in person, but forgets to immediately update her online store, an online customer might buy the exact same dress. This leads to refunds, angry customers, and manual labor to keep systems in sync. Furthermore, anticipating when to restock items requires manual calculation. They need an invisible, real-time inventory synchronization engine that instantly updates stock across all channels (online, in-person, social) and uses AI to predict when they will run out of stock.

  **Research Findings:**
  Competitor Systems Audit:
  - **Shopify:** Excellent multi-channel inventory syncing, but often requires expensive third-party apps for advanced predictive restocking or seamless integration with non-Shopify in-person POS systems. The interface can be complex for non-technical users.
  - **Square:** The gold standard for physical POS inventory, but its e-commerce integration is sometimes clunky. Predictive analytics are available but often gated behind higher pricing tiers.
  - **Wix:** Basic inventory tracking is present, but lacks sophisticated real-time multi-channel sync without plugins. It does not offer robust AI-driven predictive restocking natively.

  **Design Doc:**
  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  graph TD;
      subgraph Sales Channels
          OnlineStore[OHC Storefront]
          TapToPay[Offline Tap-to-Pay POS]
          Social[Instagram/Social Sales]
      end

      OnlineStore -- Order Placed --> InventoryGateway[Inventory & Fulfillment API Gateway]
      TapToPay -- In-Person Sale --> InventoryGateway
      Social -- DM Order Approved --> InventoryGateway

      InventoryGateway --> ConflictResolver[CRDT Conflict Resolution Engine]
      ConflictResolver --> MasterLedger[(Cloud Postgres Inventory Ledger)]

      MasterLedger -- Real-Time Update --> OnlineStore
      MasterLedger -- Real-Time Update --> TapToPay

      MasterLedger --> Agents[AI Agent Swarm]

      subgraph Agent Departments
          Agents --> OpsAgent[Operations: Predictive Restock & Fulfillment]
          Agents --> CSAgent[Customer Success: Backorder Notifications]
      end
  ```

  **Mobile UX Flow (375px First)**
  1. **Inventory Overview:** Priya opens the OHC app and taps the "Inventory" tab. She sees a clean, Glassmorphism-styled list of her products with simple color-coded indicators: Green (In Stock), Yellow (Low Stock), Red (Sold Out).
  2. **AI Restock Alert:** At the top of the screen, an Operations AI card appears: "✨ Heads up Priya, your 'Blue Summer Dress (Size M)' has been selling fast this week. You have 2 left. Consider ordering more before Friday."
  3. **Product Detail:** Priya taps the dress. She sees the total stock (2), and a breakdown of where it is available (e.g., "Available Online & In-Store").
  4. **Auto-Sync in Action:** A customer buys the dress in the physical store using the OHC Tap-to-Pay POS. The app immediately updates the stock from 2 to 1. Simultaneously, the online storefront updates its stock level, preventing a double-sale.

  **AI Agent Integration Points**
  - **Operations Agent:** Constantly analyzes the `MasterLedger` for sales velocity. Uses machine learning models to predict stock depletion dates and generates proactive restock alerts. It also manages the fulfillment pipeline (e.g., printing shipping labels automatically when an online order is placed).
  - **Customer Success Agent:** If an item is oversold due to an edge-case network partition, the CS agent immediately drafts a sincere apology email to the customer, offering a refund or a backorder option, and presents it to Priya for approval.

  **Key Design Decisions**
  - **Strict Consistency via CRDTs:** The system uses Conflict-free Replicated Data Types (CRDTs) to handle concurrent sales (e.g., a customer buys the last item online at the exact same millisecond Priya sells it in-store). The CRDT engine deterministically resolves the conflict and ensures the ledger is consistent.
  - **Row-Level Security (RLS):** Every inventory item in the `MasterLedger` is secured by tenant ID using Postgres RLS, ensuring Priya can never accidentally view or modify another business's inventory.
  - **Offline Resilience:** The Tap-to-Pay POS maintains a local cache of inventory. If Priya loses internet, she can still sell items. Once reconnected, the `ConflictResolver` merges the offline sales into the `MasterLedger`.

  **Implementation Prompt**
  Implement the Autonomous Multi-Channel Inventory & Fulfillment Engine.
  - **User-Facing Outcome:** Business owners have a unified, real-time view of their inventory across all channels (online, POS, social) that automatically updates without manual intervention. AI agents predict stock-outs and alert the user proactively.
  - **CUJ:** Priya sells a dress in her physical store using the OHC POS. The system instantly deducts the item from the central inventory ledger and updates the online storefront to reflect the new stock level. The Operations Agent analyzes the remaining stock and sends Priya a push notification recommending a restock.
  - **Acceptance Criteria:**
    - Build the `InventoryGateway` and `ConflictResolver` to handle concurrent sales across channels.
    - Implement CRDTs for offline POS sync resilience.
    - Develop the Operations AI Agent logic to calculate sales velocity and trigger predictive restock alerts.
    - Ensure the UI adheres to the 375px baseline, using the Translucent Glass design system.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []