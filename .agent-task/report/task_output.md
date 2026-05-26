issue_title: "Implement Autonomous Perishable Inventory and Flash Sale Engine"
issue_description: |
  # Title: Autonomous Perishable Inventory and Flash Sale Engine

  ## Problem Statement
  For small business owners like Fatima (who runs a halal food cart) and Maya (a custom cake baker), managing inventory is not just about counting boxes in a warehouse. Their products are highly perishable and have a very short shelf-life. If Fatima makes 50 plates of chicken over rice for the lunch rush, she needs them sold today. If they don't sell by 2 PM, she needs a way to instantly broadcast a "flash sale" or discount to her local followers without logging into a complex dashboard, creating a promo code, and designing an email campaign. Conversely, if she sells out at 12:30 PM, she needs her menu to instantly update to prevent angry customers from showing up for pre-orders that can't be fulfilled. They need an invisible assistant that monitors their daily capacity, automatically adjusts prices as closing time approaches, and instantly alerts customers, all from a simple mobile interface.

  ## Research Report
  Current e-commerce platforms handle static inventory well, but fail at real-time, perishable lifecycle management.
  - **Shopify**: Good for standard retail inventory. Apps exist for discounts and sales, but they require manual configuration of complex rules (e.g., "discount X by Y% on Z dates"). Not designed for the 5-minute "I need to clear this out now" food cart reality.
  - **Square / Toast**: Designed for food and beverage, and they have "sold out" toggles (86'ing items). However, they lack integrated, autonomous multi-channel marketing to automatically push flash sales when inventory is high and time is running out.
  - **Wix / Squarespace**: Focus on website building, not real-time, dynamic inventory-to-marketing pipelines.
  - **OneHumanCorp Gap**: We need an engine that seamlessly bridges the Universal Capacity Ledger with the Marketing/Operations AI Departments to automatically trigger localized flash sales (via SMS/WhatsApp) based on perishable inventory time-to-live (TTL).

  ## Design Doc

  ### Architecture Diagrams

  **Sequence Diagram: Autonomous Flash Sale Flow**
  ```mermaid
  sequenceDiagram
      participant Owner as Business Owner (Mobile)
      participant Core as OHC Core App
      participant Ledger as Universal Capacity Ledger
      participant DeptOps as Operations Agent
      participant DeptMktg as Marketing Agent
      participant Customer as Local Customer (SMS/Web)

      Owner->>Core: Taps "Flash Sale" on 15 unsold meals
      Core->>Ledger: Update Inventory (Item TTL expiring soon)
      Core->>DeptOps: Trigger Event: Perishable Inventory Surplus
      DeptOps->>DeptMktg: Request: Launch Localized Flash Sale
      DeptMktg->>Customer: Broadcast SMS: "50% off Chicken/Rice for next hour!"
      Customer->>Core: Claims Offer & Pays
      Core->>Ledger: Decrement Inventory
      Ledger->>Owner: Push Notification: "Sale complete, 15 items claimed."
      alt Inventory Hits Zero
          Ledger->>DeptOps: Event: Item Sold Out
          DeptOps->>Core: Update Storefront UI to "Sold Out"
      end
  ```

  **Entity-Relationship Diagram: Perishable Capacity Data Model**
  ```mermaid
  erDiagram
      Tenant {
          string organization_id PK
          string domain
          string timezone
      }
      Item {
          uuid id PK
          string tenant_id FK
          string name
          boolean is_perishable
          int default_ttl_minutes
      }
      CapacityLedger {
          uuid entry_id PK
          uuid item_id FK
          string tenant_id FK
          int available_quantity
          datetime expiration_time
          string status "AVAILABLE, LOW, SOLD_OUT"
      }
      FlashSaleEvent {
          uuid event_id PK
          uuid ledger_entry_id FK
          string target_audience
          float discount_amount
          datetime broadcast_time
      }

      Tenant ||--o{ Item : owns
      Tenant ||--o{ CapacityLedger : isolated_to
      Item ||--o{ CapacityLedger : tracks_daily
      CapacityLedger ||--o{ FlashSaleEvent : triggers
  ```

  ### Data Model & Invariants (Multi-Tenant & Security)
  - **Strict Multi-Tenancy**: Every `CapacityLedger` entry and `FlashSaleEvent` is hard-bound to the `organization_id`. Database policies must enforce tenant isolation at the schema level.
  - **Zero Trust Authentication**: Inter-department agent calls (e.g., Operations triggering Marketing) must be cryptographically signed and mTLS validated via the SPIFFE/SPIRE identity framework. No unauthenticated event dispatches are permitted.
  - **Offline-First & Latency**: Mobile inventory updates (like marking "Sold Out") must persist locally via SQLite-backed SIPDB and sync optimistically to the cloud. Storefront API edge caching must reflect "Sold Out" state with <500ms latency.

  ### UI Wireframes (375px Mobile-First)

  **Screen 1: Inventory Dashboard (The "Daily Prep" View)**
  - **Header**: Translucent Glass (macOS style), clean typography. "Today's Prep"
  - **List**: UniFi-style modular cards for each item.
    - Card: "Chicken Over Rice" | "15 Remaining" | "Closes in 2h"
    - **Quick Action Button (Primary)**: A prominent, single-tap "Flash Sale / Clear Out" button.
    - **Toggle**: "Mark as Sold Out" (instantly 86's the item across all channels).

  **Screen 2: Flash Sale Confirmation (The "Magic Action")**
  - **Bottom Sheet Modal**: Pops up when "Flash Sale" is tapped.
  - **Content**: AI-generated plain language summary.
    - "I'll send a text to your 120 local followers offering these for $5 instead of $10 to sell them out before 3 PM. Sound good?"
  - **Action Buttons**: "Yes, send it" (Primary) / "Edit Details" (Secondary).

  ### Mobile UX Flow
  1. **Trigger**: Fatima opens the app at 2 PM, sees she has too much food left.
  2. **Action**: She taps "Clear Out" on the Chicken over Rice card.
  3. **AI Intervention**: The system instantly formulates a localized marketing push based on her historical data and time of day, presenting a 1-tap confirmation.
  4. **Execution**: The Marketing Agent broadcasts the sale. The Operations agent monitors inventory, auto-updating the public menu to "Sold Out" the moment the last item is claimed.

  ### AI Agent Integration Points
  - **Operations Department**: Monitors real-time inventory decrements from the Universal Capacity Ledger. Triggers "Sold Out" state changes on the storefront.
  - **Marketing Department**: Listens for "Surplus / Flash Sale" events. Automatically generates promotional copy and targets local opted-in customers via SMS or WhatsApp integration.
  - **Context/Memory**: Retains data on which flash sales perform best (e.g., "$5 off" vs "50% off") to optimize future AI-generated suggestions.

  ### Key Design Decisions and Why
  - **1-Tap Automation over Complex Rules**: Food cart owners don't have time to set up 'If-This-Then-That' rules. The system uses AI to propose the best action (the 'Magic Action') based on the current context, requiring only a simple approval.
  - **Tight Coupling of Inventory and Marketing**: In traditional systems, these are separate modules. For perishables, they must be a single pipeline. A surplus in inventory should automatically trigger a marketing action.
  - **Mobile-First, Plain Language UI**: Avoids terms like "Inventory Count" or "Campaign Broadcast". Uses "Today's Prep" and "Clear Out" to speak the user's language.

  ## Implementation Prompt
  **User-Facing Outcome**: Provide the small business owner with a "Daily Prep" mobile dashboard where they can track highly perishable items. Include a 1-tap "Clear Out" button that automatically triggers an AI-generated flash sale broadcast (SMS/WhatsApp) to local customers, and instantly marks items as "Sold Out" on the public menu when inventory hits zero.

  **Critical User Journeys (CUJ)**:
  1. Owner checks daily prep inventory and taps "Clear Out" on an overstocked item.
  2. Owner approves the AI-suggested flash sale broadcast with one tap.
  3. Customer receives the broadcast, purchases the item, and inventory decrements.
  4. When inventory reaches 0, the public storefront automatically updates the item to "Sold Out".

  **Acceptance Criteria**:
  - The engine must support defining items with a short Time-To-Live (TTL) or daily reset cycle.
  - A single user action on the frontend must successfully dispatch an event to the AI Marketing Department to initiate a broadcast.
  - The Universal Capacity Ledger must enforce strict locking to prevent double-booking during a high-velocity flash sale.
  - The public storefront must reflect "Sold Out" status within <500ms of the inventory hitting zero.
  - The UI components must follow the established Translucent Glass and modular card design system, optimized for a 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
