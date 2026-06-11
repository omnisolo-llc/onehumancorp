issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (e.g., boutique owners like Priya) struggle with disjointed inventory management across online and in-store (POS) sales channels. Double-booking and out-of-stock scenarios occur during simultaneous purchases because OHC currently lacks a real-time, strongly consistent inventory locking mechanism and a robust distributed sync protocol.

  ## Research Report
  Based on competitive analysis and the internal research report (`docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`):
  - Competitors like Shopify dominate e-commerce but fail micro-SMEs due to complexity. Their online and in-store inventory frequently falls out of sync without expensive add-ons.
  - Square and Stripe Terminal provide robust POS hardware but lack integrated, agentic workflow automation.
  - **OHC Opportunity**: Implement a seamless, real-time inventory system using Redis distributed locks for online checkout and tap-to-pay, paired with an offline-first POS sync mechanism. Operations Agents handle exceptions (like low stock or conflict resolution) autonomously, notifying the owner via the Agent Feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Checkout] -->|Reserve Inventory| B(Redis Redlock: ohc:lock:{tenant}:{product})
      C[Stripe POS Terminal] -->|Reserve Inventory| B
      D[Offline POS Client] -->|Sync on Reconnect| E(pos_offline_transactions)
      B -->|Commit on Payment| F[(PostgreSQL Central Ledger)]
      E -->|Reconcile & Update| F
      F -->|Trigger Event| G[Operations Agent]
      G -->|Low Stock / Oversell| H(Agent Feed Action Card)
      H -->|Owner Approvs Restock| I[Supplier Email / PO]
  ```

  ### Data Model (PostgreSQL)
  - `pos_terminal_sessions`: Tracks active POS devices and their offline sync state.
  - `pos_offline_transactions`: Stores transactions processed while the POS device lacked connectivity.
  - `agent_feed_items`: The unified inbox for agent-generated proposals (replaces legacy `agent_action_requests`).

  ### Agent Integration
  - **Operations Agent**: Subscribes to `POS_SALE_COMPLETED` and `LowStockAlert` events. It generates Action Cards in the `agent_feed_items` table, prompting the owner to approve a restock order when inventory drops below a threshold.

  ## Implementation Prompt
  1. **Redlock Integration**: Ensure `InventoryService.reserve_inventory` accurately uses Redis to lock stock across all checkout flows (Web, API, Terminal).
  2. **Offline POS Sync**: Finalize the reconciliation logic for `pos_offline_transactions` to properly deduct stock in the central ledger and handle oversell conflicts gracefully.
  3. **Operations Agent Integration**: Update the inventory commit logic to publish events that the `OperationsAgent` catches. The agent must create rich Action Cards in `agent_feed_items` (not just `agent_action_requests`) for low stock scenarios, allowing the owner to 1-tap approve a restock order.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
