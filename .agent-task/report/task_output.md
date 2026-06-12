issue_title: "Feature: Autonomous Pre-Order & Real-Time Pickup Coordination Engine"
issue_description: |
  # Research Report: Autonomous Pre-Order & Real-Time Pickup Coordination Engine

  ## 1. Problem Statement
  Food operators with limited space and resources, such as Fatima the Food Cart Operator, struggle with overwhelming ad-hoc communication (DMs, texts) during peak hours. Legacy systems (Shopify, Wix) treat food orders like physical shipments, and dedicated restaurant POS systems (Square, Toast) are complex, expensive, and rely on passive dashboard management. Fatima needs a proactive system that handles pre-orders, dynamically manages pickup times based on current load, and auto-notifies customers—all orchestrated via a simple 375px mobile UI and AI agents working in the background.

  ## 2. Research Report
  - **Market Context**: Traditional commerce platforms are poorly suited for dynamic pickup constraints. Link-in-bio tools lack the depth for complex fulfillment. Dedicated food platforms often have a steep learning curve and take a significant cut of sales.
  - **The OHC Opportunity**: Integrating real-time capacity management natively with OHC’s Operations and Customer Success Agents creates a unique "Invisible Dispatcher" that smooths out spikes in demand.
  - **Competitor Gaps**:
    - *Square / Toast*: Built for in-person flow first. Often requires the owner to manually update wait times. High fees and rigid workflows.
    - *Shopify Local Pickup*: Cumbersome to adjust capacity dynamically. Geared towards retail, not hot food.

  ## 3. Design Doc
  ### Architecture Diagram (Concept)
  ```mermaid
  erDiagram
      Tenant ||--o{ CatalogItem : offers
      Tenant ||--o{ CapacitySlot : defines
      Customer ||--o{ Order : places
      Order }|--|| CapacitySlot : scheduled_for
      Order ||--|{ OrderItem : contains
      OrderItem }|--|| CatalogItem : references
  ```
  ### Data Model (PostgreSQL)
  - `CapacitySlot`: Represents a time window (e.g., 12:00-12:15 PM) with a `max_orders` and `current_orders` limit.
  - `Order`: Links to a specific `CapacitySlot` and manages fulfillment states (received, preparing, ready, fulfilled).

  ### AI Integration
  - **Operations Agent ("The Kitchen Manager")**: Continuously monitors the `CapacitySlot` load. If orders pile up faster than expected, it dynamically shrinks availability for upcoming slots or toggles a "High Volume" mode that extends standard wait times.
  - **Customer Success Agent ("The Maitre D'")**: Engages customers contextually. It sends automated, natural language SMS/WhatsApp updates ("Your order is being prepared and will be ready right on time at 12:15 PM!") and handles simple delays gracefully ("We're extra busy today, your pickup is slightly delayed by 5 mins").

  ### Mobile UX Flow (375px)
  1. **Customer View**: A highly visual, low-text menu optimized for fast tapping. Customers select items, see real-time available pickup windows, and checkout (Stripe) directly from their phone.
  2. **Owner View (The Kitchen Display System - KDS)**: A high-contrast, offline-tolerant order list. Big touch targets for marking items "Ready". Auto-refreshing queue. Toggles for "Pause New Orders" or "Mark Menu Item Sold Out" right at the top.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Pre-Order & Pickup Engine
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima receives an ordered queue of paid pre-orders. Her customers get exact pickup times that automatically adjust based on her cart's capacity, and receive proactive AI SMS notifications when their food is ready. She manages everything seamlessly from an older Android phone on a slow mobile network.

  **Next Actions for Engineering**:
  1. **Data Layer**: Implement the `CapacitySlot` logic in the Postgres ledger with concurrency control to prevent overbooking a specific 15-minute window.
  2. **Agentic Coordination**: Build the Operations Agent hook that monitors real-time order velocity and adjusts future `CapacitySlot` availability dynamically.
  3. **Mobile KDS**: Design the Mobile-First Owner Dashboard with 44x44px minimum touch targets to view the active queue, mark items complete, and automatically trigger the Customer Success Agent's "Order Ready" notifications.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
