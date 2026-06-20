issue_title: "Centralized Multi-Channel Inventory Sync & POS Redlock Architecture"
issue_description: |
  # Research Report: Centralized Multi-Channel Inventory Sync & POS Redlock Architecture

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) sell both online and in-store. They lack a unified inventory system that prevents double-booking items when simultaneous offline and online purchases occur. Existing tools (like Shopify POS or Square) either require expensive tier plans or lack agentic automation to handle conflict resolution gracefully without user intervention. OHC needs a centralized inventory architecture with robust distributed locking to guarantee consistency across multi-channel sales seamlessly.

  ## Target Persona
  **Priya (Boutique Operator, 35):** Sells clothing in-store and wants online. Needs inventory-aware offers, in-person tap-to-pay visibility, and no double-booking headaches. If an item sells in-store, her online store must immediately reflect that before another customer checks out.

  ## Research & Competitive Analysis
  - **Shopify POS:** Offers unified inventory, but requires higher-tier plans for full real-time sync capabilities, and its conflict resolution is purely reactive (cancellations).
  - **Square:** Great for in-store POS, but lacks strong agentic workflows to handle cross-platform inventory deduplication natively without manual reconciliation.
  - **OHC Opportunity:** Utilize a distributed lock (Redis Redlock) during the checkout process (both online and in-store via Stripe Terminal) to reserve inventory. Incorporate "The Manager" (Operations Agent) to automatically monitor these locks and resolve out-of-stock scenarios gracefully by notifying online customers, rather than failing silently or causing a double-book.

  ## Design Doc
  ### System Architecture
  ```mermaid
  graph TD
      A[Customer Online Checkout] -->|Initiate| B(Redlock Reservation Service)
      C[In-Store Tap-to-Pay POS] -->|Initiate| B
      B -->|Lock Acquired| D[PostgreSQL Central Ledger]
      B -->|Lock Denied| E[Operations Agent - The Manager]
      D -->|Update Inventory| F[Unified Storefront]
      E -->|Draft Graceful Notice| G[Action Required Queue / Customer Notification]
  ```

  ### Mobile UX Flow (375px First)
  - **POS Interface (Mobile):** The owner selects items in the POS. Upon tapping "Charge," the UI optimistic updates with a loading state while attempting the Redlock.
  - **Conflict State:** If the lock is denied (e.g., an online customer just checked out), the POS immediately displays a non-intrusive toast: "Item out of stock from online sale," preventing the charge.
  - **Online Customer Experience:** If the lock is denied (e.g., an in-store customer just checked out), the checkout screen updates via server-sent events to show "Just sold out!" with an option suggested by the Operations Agent to "Pre-order" or "Join Waitlist".

  ### AI Agent Integration
  - **Operations Agent (The Manager):** Subscribes to lock denial events. When a double-booking attempt is blocked, The Manager proactively drafts a restock order for Priya to approve in her feed and/or drafts a polite email to the online customer offering a similar item or waitlist.

  ### Key Design Decisions
  - **Redis Redlock:** Chosen for high-performance distributed locking across potentially geographically separated instances (e.g., edge storefront vs central POS API).
  - **Fail-Fast Reservation:** The checkout must fail *before* payment capture if the inventory lock cannot be acquired.
  - **Eventual Consistency Offline POS Sync:** If the POS loses connection, it relies on eventual consistency, but high-risk items (low stock) require online lock verification before proceeding to prevent overselling.

  ## Implementation Prompt
  **User-Facing Outcome:** When Priya processes a sale in-store, the exact item becomes unavailable online instantly. If a collision occurs, the system automatically prevents the sale and the AI agent offers a waitlist option, avoiding any manual cancellation or customer disappointment.

  **CUJ & Acceptance Criteria:**
  1. Setup a product with exactly 1 item in stock.
  2. Initiate an online checkout process (which acquires a 5-minute Redis Redlock).
  3. Attempt an in-store POS transaction for the same item.
  4. The POS transaction must be denied, and the POS UI must reflect the out-of-stock status.
  5. The Operations Agent must log the conflict and generate a "Restock Suggestion" card in the owner's mobile feed.
  6. Implement the `ohc:lock:{tenant_id}:inventory:{product_id}` Redlock logic in the checkout/POS service endpoints.
  7. Provide Playwright E2E tests verifying the lock acquisition and denial states using mocked simultaneous requests.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
