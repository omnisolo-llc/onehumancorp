issue_title: "Implement Offline-First Native Mobile Tap-to-Pay Integration for Flutter"
issue_description: |
  # [Feature] Mobile Tap-to-Pay Omnichannel Inventory Synchronization

  ## Problem Statement
  Priya (The Boutique Owner, 35) relies on OneHumanCorp (OHC) to run her business smoothly. She has both a physical storefront and an online catalog. Recently, she started using her smartphone to take in-person payments ("Tap-to-Pay"). However, when she sells a limited-edition piece in her physical boutique, her online store doesn't automatically reflect the drop in inventory. This forces her to manually deduct the inventory online to avoid double-selling—a stressful, error-prone task that completely undercuts the promise of OHC doing everything invisibly in the background.

  The core issue is that our mobile tap-to-pay infrastructure is completely isolated from our global multi-tenant inventory ledger system. They are currently treated as disparate systems, causing friction for omnichannel merchants who expect their stock to simply be accurate, regardless of where the sale occurred.

  ## Research Report
  - **The Status Quo:** Competitors like Shopify bundle this functionality through their Point-of-Sale (POS) application. Square treats inventory centrally but focuses aggressively on terminal hardware. Wix and GoDaddy treat POS as a cumbersome "add-on."
  - **The OHC Differentiator:** OHC's mandate is "zero-config, invisible management." Maya, Carlos, Priya, Leo, and Fatima do not understand what "omnichannel syncing" means—they just know they sold an item, so the stock should be updated.
  - **Architectural Gap Discovered:** There is currently no unified integration layer between terminal sessions/event capture (the tap-to-pay SDK logic) and the real-time global multi-tenant database cache that powers the online storefronts. The current latency of updating the inventory via traditional polling is unacceptable for a fast-paced retail environment.
  - **Goal Targets:**
    - Inventory reflects point-of-sale deduction globally under 500ms.
    - Complete offline resilience: if Priya processes an offline transaction, the inventory ledger resolves conflicts gracefully and securely once the network is restored.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph "Mobile Device (Priya's Phone)"
          App[OHC App - 375px UI]
          Tap[Tap-to-Pay Terminal Session SDK]
          LocalCache[Offline Action Queue / Local DB]
          App --> Tap
          Tap --> LocalCache
      end

      subgraph "Edge Network"
          Gateway[Zero-Trust Mobile API Gateway]
          LB[Load Balancer]
      end

      subgraph "Core OHC Multi-Tenant Platform"
          Ledger[Inventory Ledger Service]
          PaymentDB[(Transaction / POS DB)]
          InventoryDB[(Global Inventory DB)]
          AI_Ops[AI Operations Dept - Anomaly Det.]
          AI_Marketing[AI Marketing Dept - Restock Alerts]
      end

      LocalCache -->|Background Sync (gRPC/Websockets)| Gateway
      Gateway --> LB
      LB --> Ledger
      Ledger --> PaymentDB
      Ledger --> InventoryDB
      InventoryDB -->|Webhook/Event| AI_Ops
      InventoryDB -->|Low Stock Trigger| AI_Marketing

  ```

  ### Mobile UX Flow (375px baseline)
  1.  **Checkout Flow (Tap-to-Pay):**
      - **Screen 1 (Cart):** Clean glassmorphic list of items (e.g., "Vintage Silk Scarf", Qty: 1). Large, primary bottom button: "Charge $45.00".
      - **Screen 2 (Tap):** Translucent overlay triggers OS-native Tap-to-Pay UI.
      - **Screen 3 (Success):** Seamless transition back to OHC. A subtle, elegant toast notification confirms: "Paid. Online inventory updated."
  2.  **Inventory Management (Behind the scenes):**
      - The user never has to leave the main workflow. If they visit the "Inventory" tab later, the quantities are simply correct.
      - If the connection drops during tap, the success screen shows: "Paid. Syncing when online..." and queues the update invisibly.

  ### AI Agent Integration Points
  - **Operations Dept (AI Ops):** Monitors the queue. If there is a massive spike in offline transactions that fail to sync, the AI proactively investigates and temporarily flags high-risk inventory items.
  - **Marketing Dept (AI Marketing):** Triggered automatically when the sync drops inventory below a threshold (e.g., "Only 1 Vintage Silk Scarf left"). It prepares a draft Instagram post: "Almost sold out!" for Priya to approve with 1 tap.

  ### Key Design Decisions
  - **Eventual Consistency with Conflict Resolution:** Designed around a CRDT (Conflict-Free Replicated Data Type) or robust timestamp-based ledger to ensure offline sales never result in negative inventory online (or gracefully mark items "sold out online").
  - **Zero-Trust Boundaries:** The API gateway mandates SPIFFE/SPIRE identity for the mobile client. Multi-tenant isolation is enforced at the `Ledger` level so Maya's bakery data never touches Priya's boutique data.

  ## Implementation Prompt
  As an implementer, your task is to implement the "Omnichannel Sync Engine" bridging the mobile Tap-to-Pay module and our global Inventory DB.

  Acceptance Criteria:
  1. A transaction authorized via the mobile Tap-to-Pay SDK successfully deducts the corresponding SKU's inventory in the global Inventory DB.
  2. The end-to-end sync must occur in < 500ms under standard network conditions.
  3. The system must support offline caching: if the network drops during checkout, the transaction is cached locally and synchronized exactly once when the network returns.
  4. The system must strictly enforce multi-tenant boundaries (Tenant ID must be validated on every sync request).
  5. Ensure the AI Operations queue is notified via event bridge whenever a sync occurs to trigger low-stock automations.

  Do not prescribe specific database schemas or API endpoints. Let the implementer design those.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
