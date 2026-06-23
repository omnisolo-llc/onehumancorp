issue_title: "Agentic Centralized Inventory & Distributed POS Synchronization"
issue_description: |
  # Mission Queue Protocol: Agentic Centralized Inventory & Distributed POS Synchronization

  ## Problem Statement
  Priya, a boutique owner, sells inventory both in-store via a point-of-sale system and online through her web storefront. Currently, OneHumanCorp (OHC) lacks real-time inventory locking and synchronization. When a customer in-store attempts to buy the last "Red Dress" at the same time an online customer puts it in their cart, the system fails to reconcile this competition, leading to double-booking, oversold inventory, and a manual, frustrating refund process for Priya.

  ## Research Report
  Our competitive analysis (see `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`) highlights that legacy platforms like Shopify require higher-tier plans or expensive third-party apps for seamless POS and online inventory sync, often leading to a fragmented experience for micro-SMEs. Square and Stripe Terminal provide robust hardware but lack the integrated agentic workflow automation necessary to unify business operations effortlessly. The gap is clear: OHC needs a centralized ledger with a distributed lock mechanism, managed invisibly by our AI Operations Agent.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant POS as Mobile POS Client
      participant Web as Online Storefront
      participant Lock as Redis Redlock
      participant DB as Central Ledger (PostgreSQL)
      participant Agent as Operations Agent

      Note over POS,Web: Customer views "Red Dress"
      POS->>Lock: Request Lock (15s TTL)
      Lock-->>POS: Lock Granted
      Web->>Lock: Request Lock
      Lock-->>Web: Lock Denied
      Web-->>Web: Optimistic UI: "Sold Out"
      POS->>DB: Finalize Sale & Deduct
      DB-->>Agent: Inventory Trigger (Sold Out)
      Agent-->>POS: Notification: "Red Dress sold out. Draft restock order?"
  ```

  ### Mobile UX Flow
  1.  **375px First:** The POS interface ensures touch targets are at least 44x44px.
  2.  **In-Store Action:** Priya taps "Charge" on the mobile app.
  3.  **Invisible Sync:** A Redis lock is instantly acquired.
  4.  **Online View:** The online storefront updates optimistically to show "Sold Out".
  5.  **Agent Notification:** Upon successful charge, a push notification offers to draft a restock order.

  ### AI Agent Integration
  -   **Operations Agent:** Actively monitors inventory levels. It listens for sold-out events and conflicts, proactively notifying the owner with actionable steps (e.g., restocking).
  -   **Customer Success Agent:** Can automatically draft a message or update availability on the storefront.

  ### Key Design Decisions
  -   **Redis Redlock:** Chosen for its speed in distributed environments to handle rapid, simultaneous checkout attempts.
  -   **Central Ledger:** PostgreSQL remains the source of truth, updated after the lock secures the transaction.

  ## Implementation Prompt
  **Outcome:** Implement a distributed inventory locking system that prevents double-booking between in-store POS and online sales, managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1.  Priya initiates an in-store sale for the last item using the POS interface.
  2.  An online customer attempts to checkout the same item simultaneously.
  3.  The online customer receives a graceful "Sold Out" message because the item was locked by the POS transaction.
  4.  The POS transaction finalizes, deducting the inventory.
  5.  The Operations Agent sends Priya a notification offering to draft a restock order.

  **Acceptance Criteria:**
  -   Implement a Redis-based lock for inventory items during checkout.
  -   Ensure the lock prevents simultaneous checkout of the same limited-quantity item.
  -   Implement optimistic UI updates on the storefront reflecting the locked state.
  -   The Operations Agent must detect the stock-out and trigger a notification.
  -   Add comprehensive E2E tests covering the double-booking scenario.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
