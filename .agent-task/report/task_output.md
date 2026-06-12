issue_title: "[Research] OHC Centralized Inventory & Distributed POS Sync Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (e.g., Priya the boutique owner) struggle with disjointed inventory systems when selling both online and offline. An item sold in-store via a point-of-sale (POS) terminal can simultaneously be purchased by an online customer, leading to double-bookings, overselling, and customer frustration. Existing platforms require expensive third-party tools to handle unified inventory sync effectively, placing an unnecessary burden on non-technical SMB operators.

  ## Research Report
  - **Market Context**: Platforms like Shopify offer POS hardware, but true, real-time inventory sync across online and offline channels often necessitates higher-tier plans or complex integrations. Basic builders like Wix and Squarespace provide e-commerce, but their offline POS integration lacks robust consistency guarantees during concurrent purchases.
  - **The OHC Opportunity**: OHC can differentiate itself by providing native, highly consistent inventory tracking through an architecture that uses a central ledger and distributed locking mechanisms. Crucially, the "Operations Agent" can proactively monitor stock levels and automatically manage conflicts or re-order suggestions without user intervention.
  - **Competitor Gaps**:
    - *Shopify*: Good ecosystem, but high "app tax" for comprehensive inventory management across complex setups.
    - *Square*: Excellent POS, but the online storefront features often lag behind dedicated e-commerce builders.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Checkout] --> B(Redis Redlock: Inventory Reservation)
      C[Mobile POS / Tap-to-Pay] --> B
      B --> D{Reservation Success?}
      D -->|Yes| E[Complete Payment via Stripe]
      D -->|No| F[Operations Agent: Trigger 'Item Sold Out' Flow]
      E --> G[PostgreSQL Central Ledger: Finalize Deduction]
      G --> H[Operations Agent: Monitor Low Stock]
  ```

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth for inventory counts. Updates must use row-level locking or optimistic concurrency control.
  - **Distributed Locks (Redis Redlock)**: Used during the checkout phase (both online and POS) to create a temporary hold on the inventory. This prevents the "double-booking" scenario. Lock keys should follow the pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline-First POS Capability**: The mobile POS interface caches the catalog locally. It must gracefully handle offline transactions and synchronize via an eventual consistency model with the central ledger when connectivity returns.

  ### AI Agent Coordination
  - **Operations Agent (The Manager)**:
    - Monitors real-time stock levels.
    - Resolves sync conflicts (e.g., if a lock fails, it notifies the user gracefully).
    - Sends push notifications for low-stock items: "Red Dress sold out. Draft a restock order?"
  - **Customer Success Agent (The Ambassador)**: Automatically updates online storefront availability and can draft apology emails if a race condition causes an online order to be canceled due to in-store purchase.

  ### Mobile UX Flow (375px)
  1.  **POS Mode**: Priya uses the OHC app (POS view). Large, touch-friendly product tiles. She taps "Red Dress" and proceeds to charge via a card reader.
  2.  **Immediate Reservation**: Behind the scenes, the system applies a short-lived Redis lock.
  3.  **Conflict Handling**: If an online customer attempts to buy the dress at the exact same moment, the online UI instantly updates to "Sold Out" via optimistic UI updates driven by the Operations Agent.

  ## Implementation Prompt
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: A unified inventory system where an in-store POS transaction instantly reserves stock, preventing double-bookings online.

  **Critical User Journey (CUJ)**:
  1.  Simulate concurrent purchases: One via the mobile POS interface and one via the online storefront.
  2.  Verify that the Redis Redlock mechanism successfully allows only one transaction to proceed, applying a temporary hold.
  3.  Verify the failed transaction receives a graceful "Sold Out" message.
  4.  Verify the successful transaction deducts the inventory in the PostgreSQL central ledger.
  5.  Trigger the Operations Agent to generate a low-stock notification.

  **Acceptance Criteria**:
  - Implementation of the Redis distributed lock for inventory.
  - Updates to the PostgreSQL schema to support precise inventory decrements.
  - Mobile UI (375px) for the POS checkout flow handling lock failures gracefully.
  - E2E Playwright test simulating concurrent purchases to prove lock efficacy.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
