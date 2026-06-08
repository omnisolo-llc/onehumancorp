issue_title: "Implement Offline-Tolerant Mobile POS with Real-Time Multi-Channel AI Inventory Sync"
issue_description: |
  ## Title
  Implement Offline-Tolerant Mobile POS with Real-Time Multi-Channel AI Inventory Sync

  ## Problem Statement
  Small business owners with both physical and digital presence (e.g., Priya the boutique owner, Fatima the food cart operator) face a critical operational gap: disconnected inventory. When an item is sold in-store via a physical card reader, the online storefront often still shows it in stock, leading to double-sales, unhappy customers, and manual reconciliation nightmares. Existing platforms (Shopify, Square) offer POS systems but require complex integrations or manual syncing, and lack proactive AI agents that can pause online sales instantly and suggest restocks when offline transactions occur in low-connectivity environments (like farmers markets).

  ## Research Report
  - **Market Context**: Square dominates offline POS but its e-commerce capabilities are often bolted-on. Shopify has excellent e-commerce but its POS can be complex and expensive for micro-SMEs. Both fail to provide true "offline-first" agentic syncing for spotty network conditions without heavy manual intervention.
  - **Competitor Gaps**: Traditional systems rely on synchronous database calls. If Priya is at a local pop-up shop with poor mobile data, her tap-to-pay might work locally, but the online store remains unaware until she regains connection.
  - **OHC Opportunity**: OHC can differentiate by offering a mobile-first, offline-tolerant POS client coupled with an AI "Operations Agent" that uses optimistic locking and eventual consistency. When an offline transaction is recorded, local inventory is adjusted. Upon network restore, the sync protocol updates the PostgreSQL central ledger, and the Operations Agent dynamically adjusts online storefronts and drafts restock orders.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client - Flutter 375px] -->|Local SQLite Cache| B(Local Inventory Store)
      A -->|Tap-to-Pay/Stripe Terminal| C[Offline Transaction Queue]
      C -->|Network Restore| D[API Sync Gateway REST/gRPC]
      D --> E{Multi-Tenant Sync Engine}
      E -->|Optimistic Lock & Update| F[(PostgreSQL Central Ledger with Row-Level Security)]
      E -->|Redis Redlock| G[(Redis Distributed Lock)]
      E --> H[Event Bus]
      H --> I[Operations Agent]
      I -->|Check Thresholds| J[Push Notification: Restock Suggestion]
      I -->|Auto-hide| K[Dynamic Edge Storefront]
  ```

  ### Mobile UX Flow (375px First)
  1. **POS Dashboard (Mobile)**: Clean grid of products with large touch targets (min 44x44px). Top bar shows a subtle "Offline Mode" indicator if connectivity is lost.
  2. **Cart & Checkout**: User taps products to add to cart. Taps "Charge". If offline, it stores the transaction securely using Stripe's offline capabilities or a local queue for cash/later-sync.
  3. **Sync Feedback**: Once online, a small snackbar indicates "Syncing 3 sales...".
  4. **Agent Intervention**: If a sync results in an item hitting zero stock, a translucent glass notification card appears: "Red Dress sold out. Hide from online store and draft restock order? [Approve] [Dismiss]".

  ### AI Agent Integration Points
  - **Operations Agent**: Subscribes to the synchronization event bus. When an offline batch is committed to the central ledger, the Agent checks inventory thresholds. If an item sells out, it automatically updates the CDN edge cache (Universal Edge Storefront) to prevent online double-sales and drafts a restock notification for the owner.
  - **Finance Agent**: Reconciles offline cash transactions with Stripe Terminal data to provide a unified daily revenue summary.

  ### Key Design Decisions
  - **Local-First Architecture**: The mobile app must treat its local SQLite database as the primary read source for the POS catalog, falling back to network sync in the background.
  - **Eventual Consistency with Conflict Resolution**: In the rare event of an online sale and an offline sale exceeding stock simultaneously, the Operations Agent handles the conflict by prioritizing the offline sale (already paid in person) and drafting an apology/refund email for the online customer via the Customer Success Agent.
  - **Zero Trust & Multi-Tenancy**: All sync endpoints must strictly enforce `tenant_id` at the API layer and utilize PostgreSQL Row-Level Security (RLS) to ensure no bleed between businesses.

  ## Implementation Prompt
  **User-Facing Outcome**: As an owner selling at a farmer's market with spotty 4G, I can quickly ring up customers using my phone. When I get back to wifi, all sales sync automatically. If I sold the last of a product, my online store updates instantly, and my AI assistant asks if I want to reorder more.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. Login to OHC Mobile App on a 375px viewport and navigate to the POS screen.
  2. Simulate network disconnect (Offline Mode).
  3. Add items to the cart (touch targets >44px) and complete a local transaction.
  4. Simulate network restore.
  5. The app must autonomously push the offline transaction queue to the backend.
  6. The backend PostgreSQL ledger must reflect the new inventory count securely isolated by `tenant_id`.
  7. The Operations Agent must detect the change and, if stock hits zero, trigger an event to hide the product from the public storefront and send an approval card for restocking.

  **Constraints**: Do not prescribe specific API route names or SQL DDL. Design the service layer to handle the synchronization queue gracefully. Ensure 100% unit test coverage and at least 5 Playwright E2E tests validating the offline-to-online transition flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
