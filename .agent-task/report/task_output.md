issue_title: "[Architectural Gap] Real-time Centralized Inventory & Distributed POS Lock Synchronization"
issue_description: |
  ## Core Vision Alignment
  OneHumanCorp (OHC) is an AI work assistant for business owners (like Priya the boutique owner) who need a single unified view of their operations. Currently, our architecture lacks robust multi-channel synchronization for inventory operations. Legacy platforms either require desktop setups to manage complex inventory or they suffer from double-booking risks when sales happen concurrently offline (tap-to-pay) and online.

  ## Problem Statement (The Gap)
  Priya manages inventory both in her physical boutique and via her online storefront. When an item is placed in an online cart, but sold simultaneously in-store via a POS terminal, the current architecture lacks a distributed locking mechanism to prevent double-booking. This forces the owner into frustrating manual reconciliations, violating our "AI Does Useful Work" core value.

  ## Research Report & Competitive Analysis
  - **Shopify/Wix:** Rely heavily on desktop applications to manage complex inventory state and sync rules. Their offline POS clients are strong but disconnected from instantaneous cart reservations on the web.
  - **Square:** Great local POS but lacks integrated agentic workflows that can automatically notify customers or adjust online campaigns when stock runs low.
  - **OHC Need:** A mobile-first, robust inventory locking system that reserves stock instantly via Redis across all sales channels (web + terminal), supported by Agent coordination to handle edge cases invisibly.

  ## High-Level Architectural Design (Design Doc)

  **1. Data Model & Distributed Sync Protocol**
  - **Central Ledger:** PostgreSQL remains the source of truth, but we introduce strict row-level isolation using `ENABLE ROW LEVEL SECURITY` with `tenant_id`.
  - **Distributed Locks:** Implement Redis Redlock (e.g., `ohc:lock:{tenant_id}:inventory:{product_id}`) to securely reserve items during checkout. The lock TTL will be dynamically adjusted based on channel (e.g., 5 min for online cart, 15 sec for POS tap-to-pay).
  - **Eventual Consistency POS Client:** The offline-first POS component must cache catalog and availability states, synchronizing finalized sales to the central ledger asynchronously upon network recovery.

  **2. Mobile-First UX Flow (375px Target)**
  - A unified Inventory & POS dashboard optimized for 375px screens.
  - Large, translucent "Glassmorphism" touch targets (≥ 44x44px) to easily adjust stock levels manually.
  - Optimistic UI updates when an item is tapped for purchase, reflecting instant stock decrement while the Redis lock is negotiated in the background.

  **3. AI Agent Coordination (The Invisible Hand)**
  - **Operations Agent:** Monitors Redis lock events and Postgres inventory counts. Automatically flags items running low and prepares restock drafts for owner approval.
  - **Customer Success Agent:** If a cart reservation fails due to an offline sale, the agent automatically drafts an apologetic, personalized follow-up offering a similar item or a backorder notification.

  ## Implementation Prompt (For Implementer Agent)
  Implement the distributed inventory locking architecture described above.

  1. **Backend:** Introduce Redis Redlock-based reservation logic in the Go/Bazel backend for the checkout flow. Ensure it strictly respects `tenant_id` boundaries.
  2. **Frontend:** Update the Flutter/PWA POS screen (targeting 375px width) to use optimistic updates and visually indicate when an item is locked or out of stock. Apply OHC Premium Token styling (translucent materials, clear spacing).
  3. **Agent Integration:** Plumb an event trigger from the inventory lock failure state to the Customer Success Agent to automatically draft the follow-up message.
  4. **Verification:** You MUST implement comprehensive E2E Playwright tests covering the concurrent online/offline purchase conflict scenario to prove the double-booking is prevented.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
