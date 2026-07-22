issue_title: "Implement Multi-Channel POS Tap-to-Pay Architecture & Offline-First Sync"
issue_description: |
  # Research Report: Centralized Multi-Channel POS Tap-to-Pay Architecture & Offline-First Sync

  ## Problem Statement
  Small business owners with both physical and digital presences (like Priya, the boutique owner, or Fatima, the food cart operator) struggle to maintain consistent inventory and synchronized sales data across channels. Legacy tools like Shopify require complex add-ons for unified POS and lack intelligent fallback for intermittent network connectivity (such as at a farmer's market). When the network drops, they cannot securely accept tap-to-pay transactions or guarantee inventory locks, resulting in double-selling and manual reconciliation chaos. They need a robust, offline-tolerant POS solution that unifies in-person and digital channels under the OHC Assistant's purview.

  ## Research Report & Gap Analysis
  - **Competitor Systems Audit:** Shopify POS is robust but requires an expensive tier and doesn't seamlessly integrate AI-driven workflows out-of-the-box. Stripe Terminal provides excellent Tap-to-Pay APIs but leaves the burden of offline queueing, inventory synchronization, and conflict resolution entirely to the merchant developer.
  - **The OHC Gap:** The current OHC platform lacks a unified `TerminalSession` mechanism with offline-first caching for Tap-to-Pay. There's no systemic protocol for queueing offline transactions securely, nor a distributed lock mechanism (Redis Redlock) for rapid, short-lived inventory reservations that bridge the physical and digital storefronts instantly.
  - **Persona Need:** Priya needs to tap a customer's card in her boutique. If the store's Wi-Fi drops, the transaction must queue safely. Once restored, the system must reconcile the inventory globally and alert her Operations Agent if an online customer simultaneously bought the last item.

  ## Design Doc
  ### System Architecture
  ```mermaid
  graph TD
      A[Mobile POS App - 375px] -->|Local Sync Queue| B(Local SQLite Cache)
      A -->|Tap-to-Pay| C[Stripe Terminal SDK]
      A -.->|Online/Restored Network| D[OHC Core API Gateway]
      D --> E{Stripe API}
      D --> F[Redis Redlock - Inventory Reservation]
      F --> G[PostgreSQL Central Ledger]
      G --> H[Operations AI Agent - Conflict Resolution]
  ```

  ### Mobile-First UX Flow (375px)
  1. **POS Dashboard:** A clean, high-contrast, translucent UI card layout optimized for 375px viewports with touch targets >= 44x44px. Displays the active product catalog.
  2. **Checkout Flow:** One-tap to add to cart. Large "Charge $XX.XX" button initiates Tap-to-Pay.
  3. **Network Status Indicator:** Unobtrusive status pill (Green/Online, Amber/Offline).
  4. **Offline Mode:** If Amber, the app records the intent, queues the transaction securely (if offline processing rules permit), and updates local inventory optimistically.
  5. **Reconciliation State:** When network restores, background sync occurs. If a conflict arises (e.g., inventory oversold online), the Operations Agent triggers a unified inbox notification prompting the owner for resolution (e.g., "Draft refund or backorder?").

  ### AI Agent Integration
  - **Operations Agent:** Monitors the background sync queue. If an inventory conflict is detected during reconciliation, it drafts an actionable summary for the owner.
  - **Finance Agent:** Automatically categorizes POS transactions vs. digital transactions in the daily financial digest.

  ### Key Design Decisions
  - **Offline-First Resilience:** Prioritize local state using SQLite for the catalog and transaction queue on mobile, ensuring zero disruption to physical sales.
  - **Redis Redlock:** Employ Redis for micro-reservations (e.g., 15 seconds) during the critical checkout phase to drastically reduce double-booking chances when online.
  - **Zero Trust:** Strictly enforce tenant isolation on all POS endpoints and sync routes via SPIFFE/SPIRE-backed authentication context.

  ## Implementation Prompt
  Implement the core architectural foundations for the OHC Multi-Channel POS Tap-to-Pay feature.
  1. Define the backend data models and gRPC/REST API endpoints for POS Terminal Sessions and Offline Sync Queues. Ensure strict row-level multi-tenant isolation.
  2. Implement a distributed locking mechanism using Redis (Redlock pattern) to handle short-lived inventory reservations during checkout.
  3. Design the mobile-first frontend flow (simulated or real UI components) adhering to the macOS Translucent Glass and UniFi modular dashboard design tokens (375px optimized, 44x44px touch targets). Include the visual network status indicator and optimistic UI updates for the cart.
  4. Wire the conflict resolution trigger: If sync detects an inventory negative balance, emit an event for the Operations AI Agent to handle.

  **Acceptance Criteria:**
  - Robust backend sync endpoints that handle offline-queued data gracefully.
  - Demonstrated distributed locking preventing concurrent checkout on the same inventory item.
  - Mobile UI flow verified via Playwright E2E tests simulating online and offline checkout scenarios.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
