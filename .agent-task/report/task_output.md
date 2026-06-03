issue_title: "[Feature] Mobile Tap-to-Pay Omnichannel Inventory Synchronization"
issue_description: |
  # Mobile Tap-to-Pay Omnichannel Inventory Synchronization

  **Problem Statement:**
  Priya (The Boutique Owner, 35) relies on OneHumanCorp (OHC) to run her business smoothly. She has both a physical storefront and an online catalog. Recently, she started using her smartphone to take in-person payments ("Tap-to-Pay"). However, when she sells a limited-edition piece in her physical boutique, her online store doesn't automatically reflect the drop in inventory. This forces her to manually deduct the inventory online to avoid double-selling—a stressful, error-prone task that completely undercuts the promise of OHC doing everything invisibly in the background.

  **Research Report:**
  - **The Status Quo:** Competitors like Shopify bundle this functionality through their POS application. Square treats inventory centrally but focuses aggressively on terminal hardware. Wix and GoDaddy treat POS as an "add-on."
  - **The OHC Differentiator:** OHC's mandate is "zero-config, invisible management." Non-technical owners don't understand "omnichannel syncing" - they just know they sold an item, so stock should be updated.
  - **Architectural Gap Discovered:** There is currently no unified integration layer between terminal sessions/event capture (the tap-to-pay SDK logic) and the real-time global multi-tenant database cache that powers the online storefronts.

  **Goal Targets:**
  - Inventory reflects point-of-sale deduction globally under 500ms.
  - Complete offline resilience: resolves conflicts gracefully and securely once network is restored.

  **Design Doc:**
  ### Architecture
  Mobile App with local offline cache pushes background sync to Edge Gateway via gRPC/Websockets. Gateway load balances to Ledger Service, updating Payment DB and Global Inventory DB. Global Inventory DB triggers AI Operations (for anomalies) and Marketing (for low-stock restock alerts).

  ### UI Flow (375px)
  - Checkout Flow: Cart -> Native Tap-to-Pay -> Success toast ("Paid. Online inventory updated.")
  - Offline mode shows "Paid. Syncing when online..." and queues the update invisibly.

  **Implementation Prompt:**
  Implement the "Omnichannel Sync Engine" bridging the mobile Tap-to-Pay module and our global Inventory DB.
  1. A transaction authorized via the mobile Tap-to-Pay SDK successfully deducts the corresponding SKU's inventory in the global Inventory DB.
  2. End-to-end sync in < 500ms under standard network conditions.
  3. Support offline caching (sync exactly once when network returns).
  4. Strictly enforce multi-tenant boundaries.
  5. Trigger AI Operations queue via event bridge whenever a sync occurs to enable low-stock automations.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
