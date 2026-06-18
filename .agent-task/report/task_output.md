issue_title: Mobile-First Omnichannel Sync (Tap-to-Pay)
issue_description: "\n# Mobile-First Omnichannel Sync (Tap-to-Pay)\n\n## Problem Statement\n\
  Priya (The Boutique Owner, 35) relies on OneHumanCorp (OHC) to run her business\
  \ smoothly. She has both a physical storefront and an online catalog. Recently,\
  \ she started using her smartphone to take in-person payments (\"Tap-to-Pay\").\
  \ However, when she sells a limited-edition piece in her physical boutique, her\
  \ online store doesn't automatically reflect the drop in inventory. This forces\
  \ her to manually deduct the inventory online to avoid double-selling\u2014a stressful,\
  \ error-prone task that completely undercuts the promise of OHC doing everything\
  \ invisibly in the background.\n\nThe core issue is that our mobile tap-to-pay infrastructure\
  \ is completely isolated from our global multi-tenant inventory ledger system. They\
  \ are currently treated as disparate systems, causing friction for omnichannel merchants\
  \ who expect their stock to simply be accurate, regardless of where the sale occurred.\n\
  \n## Research Report\n- **The Status Quo:** Competitors like Shopify bundle this\
  \ functionality through their Point-of-Sale (POS) application. Square treats inventory\
  \ centrally but focuses aggressively on terminal hardware. Wix and GoDaddy treat\
  \ POS as a cumbersome \"add-on.\"\n- **The OHC Differentiator:** OHC's mandate is\
  \ \"zero-config, invisible management.\" Maya, Carlos, Priya, Leo, and Fatima do\
  \ not understand what \"omnichannel syncing\" means\u2014they just know they sold\
  \ an item, so the stock should be updated.\n- **Architectural Gap Discovered:**\
  \ There is currently no unified integration layer between terminal sessions/event\
  \ capture (the tap-to-pay SDK logic) and the real-time global multi-tenant database\
  \ cache that powers the online storefronts. The current latency of updating the\
  \ inventory via traditional polling is unacceptable for a fast-paced retail environment.\n\
  - **Goal Targets:**\n  - Inventory reflects point-of-sale deduction globally under\
  \ 500ms.\n  - Complete offline resilience: if Priya processes an offline transaction,\
  \ the inventory ledger resolves conflicts gracefully and securely once the network\
  \ is restored.\n\n## Design Doc\n\n### Architecture Diagram\n```mermaid\ngraph TD\n\
  \    subgraph \"Mobile Device (Priya's Phone)\"\n        App[OHC App - 375px UI]\n\
  \        Tap[Tap-to-Pay Terminal Session SDK]\n        LocalCache[Offline Action\
  \ Queue / Local DB]\n        App --> Tap\n        Tap --> LocalCache\n    end\n\n\
  \    subgraph \"Edge Network\"\n        Gateway[Zero-Trust Mobile API Gateway]\n\
  \        LB[Load Balancer]\n    end\n\n    subgraph \"Core OHC Multi-Tenant Platform\"\
  \n        Ledger[Inventory Ledger Service]\n        PaymentDB[(Transaction / POS\
  \ DB)]\n        InventoryDB[(Global Inventory DB)]\n        AI_Ops[AI Operations\
  \ Dept - Anomaly Det.]\n        AI_Marketing[AI Marketing Dept - Restock Alerts]\n\
  \    end\n\n    LocalCache -->|Background Sync (gRPC/Websockets)| Gateway\n    Gateway\
  \ --> LB\n    LB --> Ledger\n    Ledger --> PaymentDB\n    Ledger --> InventoryDB\n\
  \    InventoryDB -->|Webhook/Event| AI_Ops\n    InventoryDB -->|Low Stock Trigger|\
  \ AI_Marketing\n```\n\n### Mobile UX Flow (375px baseline)\n1. **Checkout Flow (Tap-to-Pay):**\n\
  \   - **Screen 1 (Cart):** Clean glassmorphic list of items (e.g., \"Vintage Silk\
  \ Scarf\", Qty: 1). Large, primary bottom button: \"Charge $45.00\".\n   - **Screen\
  \ 2 (Tap):** Translucent overlay triggers OS-native Tap-to-Pay UI.\n   - **Screen\
  \ 3 (Success):** Seamless transition back to OHC. A subtle, elegant toast notification\
  \ confirms: \"Paid. Online inventory updated.\"\n2. **Inventory Management (Behind\
  \ the scenes):**\n   - The user never has to leave the main workflow. If they visit\
  \ the \"Inventory\" tab later, the quantities are simply correct.\n   - If the connection\
  \ drops during tap, the success screen shows: \"Paid. Syncing when online...\" and\
  \ queues the update invisibly.\n\n### AI Agent Integration Points\n- **Operations\
  \ Dept (AI Ops):** Monitors the queue. If there is a massive spike in offline transactions\
  \ that fail to sync, the AI proactively investigates and temporarily flags high-risk\
  \ inventory items.\n- **Marketing Dept (AI Marketing):** Triggered automatically\
  \ when the sync drops inventory below a threshold (e.g., \"Only 1 Vintage Silk Scarf\
  \ left\"). It prepares a draft Instagram post: \"Almost sold out!\" for Priya to\
  \ approve with 1 tap.\n\n### Key Design Decisions\n- **Eventual Consistency with\
  \ Conflict Resolution:** Designed around a CRDT (Conflict-Free Replicated Data Type)\
  \ or robust timestamp-based ledger to ensure offline sales never result in negative\
  \ inventory online (or gracefully mark items \"sold out online\").\n- **Zero-Trust\
  \ Boundaries:** The API gateway mandates SPIFFE/SPIRE identity for the mobile client.\
  \ Multi-tenant isolation is enforced at the `Ledger` level so Maya's bakery data\
  \ never touches Priya's boutique data.\n\n## Implementation Prompt\n**To the Implementer\
  \ Swarm:**\nYour objective is to implement the \"Omnichannel Sync Engine\" bridging\
  \ the mobile Tap-to-Pay module and our global Inventory DB.\n\n**Acceptance Criteria:**\n\
  1. A transaction authorized via the mobile Tap-to-Pay SDK successfully deducts the\
  \ corresponding SKU's inventory in the global `InventoryDB`.\n2. The end-to-end\
  \ sync (from terminal authorization to database update) must occur in < 500ms under\
  \ standard network conditions.\n3. The system must support offline caching: if the\
  \ network drops during checkout, the transaction is cached locally and synchronized\
  \ exactly once when the network returns.\n4. The system must strictly enforce multi-tenant\
  \ boundaries (Tenant ID must be validated on every sync request).\n5. Ensure the\
  \ AI Operations queue is notified via event bridge whenever a sync occurs to trigger\
  \ low-stock automations.\n\nDo not prescribe specific database ORMs or queue technologies.\
  \ Implement the bridging logic, data structures, and multi-tenant security layers\
  \ required to satisfy the constraints above. Ensure all features pass the \"grandmother\
  \ test\"\u2014keep the UI purely transactional and hide the syncing complexity entirely.\n\
  \n## Priority\n`P0`\n\n## Estimated Scope\nLarge\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
