issue_title: "Offline-Tolerant Multilingual Pre-Order & Print System for Food Operators"
issue_description: |
  # Research Report: Offline-Tolerant Multilingual Pre-Order & Print System for Food Operators

  ## 1. Problem Statement
  Food cart operators like Fatima face unique operational constraints: they operate in low-bandwidth outdoor environments, serve a multi-lingual customer base, and manage high-velocity, time-sensitive pre-orders. Traditional POS and e-commerce platforms like Shopify, Square Online, and Wix assume stable internet connections and primarily target desktop or high-end mobile devices. When Fatima's connection drops, she loses visibility into incoming orders and cannot update her menu availability. Furthermore, translating the menu for diverse customers or printing a consolidated end-of-day kitchen list is often a manual, disjointed process requiring multiple apps.

  ## 2. Research Report
  - **Market Context:** Existing platforms lack true offline-first operational capabilities tailored for micro food vendors.
    - *Square Online:* Good POS integration, but heavily reliant on continuous internet access for its web store updates.
    - *Shopify:* "Offline" POS is functional but web-based pre-orders require strong connectivity. Multilingual support usually requires paid apps.
  - **The OHC Opportunity:** By leveraging local-first architecture (offline-tolerant data synchronization) and AI-driven translation on a low-data mobile app, OHC can capture the micro food and beverage market. The system should allow operators to toggle item availability without a network connection, syncing automatically once reconnected.
  - **Competitor Gaps:**
    - Competitors do not offer built-in, plain-language end-of-day print summaries that auto-translate.
    - Mobile-first, low-data architectures are an afterthought for most e-commerce giants.

  ## 3. Design Doc
  ### Data Model & Sync Protocol (Local-First Architecture)
  - **Local Database (IndexedDB/SQLite):** The mobile POS app caches the active menu, inventory counts, and today's pre-orders locally.
  - **Conflict-Free Replicated Data Types (CRDTs) or Sync Queue:** When the operator toggles an item as "Sold Out" while offline, the action is stored in a local mutation queue. Upon reconnection, the queue syncs with the central PostgreSQL ledger.
  - **Central Ledger (PostgreSQL):** Maintains the ultimate source of truth, enforcing multi-tenant isolation via RLS.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ MenuItem : owns
      Tenant ||--o{ PreOrder : receives
      MenuItem ||--o{ SyncMutation : generates
      PreOrder ||--o{ SyncMutation : generates

      SyncMutation {
          uuid id PK
          uuid tenant_id FK
          string entity_type
          string entity_id
          jsonb payload
          string status "pending, synced, failed"
          timestamp created_at
      }

      MenuItem {
          uuid id PK
          string name
          boolean is_sold_out
          int inventory_count
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Fatima
      participant App as Mobile POS (Local SQLite)
      participant Network as Network Connection
      participant Server as OHC Backend (PostgreSQL)

      Fatima->>App: Marks "Chicken Over Rice" Sold Out
      alt Offline
          App->>App: Update Local UI (Optimistic)
          App->>App: Store Mutation in Sync Queue
      else Online
          App->>Server: Send Mutation Immediately
          Server-->>App: Ack
      end

      Network->>App: Connection Restored
      App->>Server: Flush Sync Queue
      Server-->>App: Ack
  ```

  ### AI Integration
  - **Operations Agent ("The Kitchen Manager"):** Monitors incoming pre-orders and automatically generates a consolidated daily prep list. When the connection drops and restores, the agent resolves any potential double-booking conflicts and notifies the owner.
  - **Translation Agent:** Autonomously translates menu items and customer notifications (e.g., "Your order is ready for pickup") between English and the operator's preferred language (e.g., Arabic), ensuring smooth communication regardless of language barriers.

  ### Mobile UX Flow (375px First)
  1. **Offline-Tolerant Menu Management:** A simple grid layout with large (44x44px minimum) touch targets for "Sold Out" toggles. Toggles provide immediate visual feedback (optimistic UI), even without an internet connection.
  2. **Consolidated Pre-Order List:** A clean, easy-to-read list of upcoming pickups grouped by time. A "Print Daily Summary" button generates a Bluetooth-printer-friendly text format of the day's requirements.
  3. **Low-Data Mode:** UI uses lazy loading for images and avoids heavy animations to function smoothly on older Android devices with 3G/intermittent connections.

  ## 4. Implementation Prompt
  **Feature Name:** OHC Offline-Tolerant Pre-Order & Kitchen Management System
  **Target Persona:** Fatima the Food Cart Operator
  **Outcome:** Fatima can confidently manage her daily pre-orders, toggle menu items as sold out without internet, and generate a translated end-of-day prep list, all from an older Android phone on a spotty cellular connection.

  **Next Actions:**
  1. Implement a robust offline mutation queue for the POS client that safely synchronizes menu availability toggles (e.g., "Sold Out") with the PostgreSQL backend upon reconnection.
  2. Develop a 375px mobile-first UI for managing daily pickup orders, featuring a dedicated "Low-Data Mode" and immediate optimistic UI updates.
  3. Integrate the Translation Agent to auto-translate incoming customer requests and menu items between English and the operator's primary language.
  4. Create a Bluetooth-printer-friendly summary generation capability triggered by the Operations Agent.

  **Acceptance Criteria:**
  - Toggling an item to "Sold Out" while network is disabled must update the UI immediately and sync to the backend once the network is restored.
  - The prep list must be generated in the operator's native language.
  - Playwright E2E tests must verify the offline sync queue mechanism using `context.setOffline(true)`.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []