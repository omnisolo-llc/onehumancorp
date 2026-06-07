issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (e.g., Priya the boutique owner) struggle with keeping inventory in sync between their online store and in-person sales (Point-of-Sale). Existing solutions on platforms like Shopify or Wix are often too complex, require expensive third-party apps, or lack real-time synchronization, leading to double-booking or selling out-of-stock items. There is a need for a seamless, strongly consistent, and real-time inventory management system that works flawlessly on mobile devices (375px first) and seamlessly merges online and offline operations.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Robust POS and inventory systems but complex for micro-SMEs, often requiring paid apps for complete synchronization across multiple channels. Setup takes considerable time and effort.
  - **Wix/Squarespace:** Simpler setups but weaker in-person POS capabilities compared to Shopify or dedicated POS systems like Square.
  - **Square/Stripe Terminal:** Excellent hardware and in-person POS, but often fragmented from the primary online storefront unless tightly integrated by developers.
  - **OHC Opportunity:** Provide a unified, centralized inventory ledger powered by AI agents that automatically handles sync, reservations, and conflict resolution across online carts and offline tap-to-pay, all manageable from a simple mobile app without technical setup.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Process| B(Inventory Reservation System)
      C[Mobile POS App / Terminal] -->|Rapid Checkout| B
      B -->|Redis Redlock| D{Distributed Lock}
      D -->|Acquired| E[Central PostgreSQL Ledger]
      E -->|Row-level Lock| F[Update Inventory Count]
      F -->|Event/Webhook| G(The Manager Agent)
      G -->|Low Stock Alert / Conflict Resolution| H[Owner Dashboard / Notifications]
      G -->|Sync Cache| I[Redis Cache]
      C -->|Offline Mode| J[Local Cache]
      J -->|Network Restored| E
  ```

  ### Mobile UX Flow (375px First)
  - **Home Screen:** Shows current stock levels and alerts for low inventory.
  - **In-Store Checkout (POS):** A clear, large-button interface (touch targets >= 44x44px) for adding items to the cart. Real-time stock availability is indicated.
  - **Inventory Conflict:** If an item is reserved online while a clerk attempts to sell it in-store, a clear "Item reserved online" warning appears, preventing double sales.
  - **Visual Design:** Premium, macOS-style Translucent Glass materials, easy-to-read typography, and clear status indicators.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Monitors the central ledger. If stock falls below a threshold, it generates an alert for the owner. It also handles edge cases where locks fail or temporary inconsistencies arise, suggesting reconciliation steps.
  - **Customer Success Agent (The Ambassador):** If an online customer's cart item becomes unavailable due to an in-store purchase, The Ambassador can draft a personalized apology or offer an alternative product.

  ### Key Design Decisions
  - **Central Source of Truth:** PostgreSQL with row-level locking ensures absolute consistency for final inventory counts.
  - **Distributed Locking:** Redis Redlock provides low-latency, temporary reservations during the checkout flow (e.g., holding an item for 5 minutes while a customer enters payment info).
  - **Offline Capability:** The POS must be able to function offline using cached data and sync changes when reconnected, gracefully handling any conflicts via AI-assisted resolution.

  ## Implementation Prompt
  **User-Facing Outcome:** As Priya, the boutique owner, I can sell the last specific dress in my store using my phone as a POS. The moment I initiate the transaction, that dress is instantly marked as unavailable on my online storefront, preventing any double sales. If the network drops in the store, the sale still goes through locally and syncs perfectly when connection returns.

  **CUJ & Acceptance Criteria:**
  1. A backend service must provide endpoints for inventory check, temporary reservation (using Redis locks), and final commit (updating PostgreSQL).
  2. Implement an offline-tolerant POS checkout flow on the mobile client that correctly uses these endpoints.
  3. Ensure optimistic concurrency or row-level locking is correctly applied in the database to prevent race conditions during simultaneous online and offline checkouts.
  4. Integrate 'The Manager' agent to log and alert on inventory threshold events or conflict resolutions.
  5. Provide Playwright E2E tests simulating simultaneous checkout attempts for the same limited-stock item from an online cart and the POS client, verifying only one succeeds and the inventory remains accurate.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
