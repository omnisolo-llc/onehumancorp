issue_title: "Implement Offline-Tolerant Agentic Order Fulfillment for Food Carts"
issue_description: |
  # Research Report: Offline-Tolerant Agentic Order Fulfillment

  ## Problem Statement
  Small business operators in fast-paced or low-connectivity environments (like Fatima the Food Cart Operator) struggle with real-time order management. Current point-of-sale and order management systems rely heavily on continuous, stable internet connections. When the network drops, orders are lost, syncing fails, and the operator is left with angry customers and out-of-sync inventory. Traditional platforms do not prioritize offline-first data structures and require manual reconciliation once the connection is restored.

  ## Research Report
  - **Competitor Gaps**: Systems like Square or Toast have "offline modes," but they are often limited to simply caching credit card swipes for later processing. They do not maintain a fully functional, AI-assisted order queue that gracefully handles concurrent online pre-orders and local, offline walk-ups without causing inventory collisions when the network returns.
  - **OHC Opportunity**: OHC can differentiate by providing an offline-tolerant local ledger on the mobile client (using SQLite/IndexedDB) paired with an eventual consistency sync engine. More importantly, the *Operations Agent* should handle the conflict resolution automatically upon reconnection, rather than forcing the owner to resolve sync errors manually.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Client POS] -->|Local Write| B(Local SQLite/IndexedDB)
      A -.->|Network Restored| C{Sync Gateway}
      D[Online Pre-Orders] -->|Incoming| C
      C --> E[Central Postgres Ledger]
      C -->|Conflict Detected| F[Operations Agent]
      F -->|Agentic Resolution| E
      F -->|Push Notification| A
  ```

  ### Mobile UX Flow (375px)
  1. **Order Feed Screen**: A highly visible, high-contrast list of pending orders. A small indicator shows "Offline Mode: Caching 3 Orders."
  2. **Order Interaction**: Fatima taps an order to mark it "Ready." The UI updates instantly (optimistic update).
  3. **Reconnection Event**: The indicator turns green ("Synced"). If the Operations Agent detected that an offline order consumed the last of an ingredient that an online customer also tried to buy, Fatima receives a simple, non-blocking alert: "Agent paused online sales for Falafel due to low stock during offline period."

  ### AI Agent Integration
  - **Operations Agent**: Acts as the reconciliation engine. Instead of a hard failure when an offline transaction conflicts with an online transaction (e.g., both sold the last unit of inventory), the Agent applies business logic (e.g., prioritize the in-person transaction) and automatically updates the online storefront to reflect the true stock.

  ## Implementation Prompt
  **Feature Name**: Offline-Tolerant POS & Order Queue
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima can continuously take orders and mark them as ready even when her mobile data connection drops. When the connection returns, OHC syncs the local ledger with the central database, and the Operations Agent resolves any inventory conflicts automatically.

  **Next Actions**:
  1. Implement a local data store (e.g., local SQLite wrapper or IndexedDB for web) for caching POS transactions and order state changes.
  2. Build the Sync Gateway that attempts background reconciliation when network connectivity is restored.
  3. Update the Operations Agent logic to handle conflict resolution events emitted by the Sync Gateway, specifically managing inventory oversell scenarios gracefully.
  4. Design the mobile-first UI to clearly, but unobtrusively, indicate connection state and optimistic UI updates.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
