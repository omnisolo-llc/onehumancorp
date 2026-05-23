issue_title: "Autonomous Dynamic Pricing & Yield Management Engine"
issue_description: |
  # [Architecture] Autonomous Dynamic Pricing & Yield Management Engine

  ## Title
  Autonomous Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners (like Leo the music tutor, Maya the baker, and Fatima the food cart owner) struggle to optimize their pricing based on real-time demand, available capacity, and market conditions. They often rely on static pricing, leading to unbooked time slots, unsold perishable inventory, or missed revenue during peak demand (like Carlos the handyman during a storm). They lack the time, data, and expertise to manually adjust prices. They need an invisible, AI-driven yield management system that automatically optimizes prices and offers targeted discounts to maximize revenue and fill capacity, without requiring them to become data scientists or manually update their catalogs.

  ## Research Report

  ### Competitive Landscape
  *   **Shopify / Wix / Squarespace**: Offer basic manual discount codes and sale prices. They lack native, autonomous dynamic pricing based on inventory velocity or calendar utilization. Third-party apps exist (e.g., for Shopify), but they require complex rules setup, target advanced e-commerce merchants, and fail the "grandmother test".
  *   **Airlines / Ride-Sharing (Uber) / Hotels**: Highly sophisticated yield management, but these are closed systems built for enterprises, not platforms accessible to solopreneurs.
  *   **Mindbody / Booking platforms**: Some offer "last-minute offers" or basic off-peak pricing, but it's often manual or requires rigid rule configuration that overwhelms the average user.

  ### The OHC Gap
  OneHumanCorp currently lacks an intelligent, autonomous pricing engine linked to the Universal Capacity and Inventory Ledger. To deliver true "business in a box" operations, we need an architecture where the AI Operations and Finance Agents can observe available capacity (e.g., empty calendar slots for Leo) or expiring inventory (e.g., Fatima's food at 4 PM) and proactively adjust prices or send targeted offers to known customers, all transparently and with zero complex configuration.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INVENTORY_ITEM : has
      TENANT ||--o{ CAPACITY_SLOT : has
      INVENTORY_ITEM ||--o{ PRICE_MODIFIER : affected_by
      CAPACITY_SLOT ||--o{ PRICE_MODIFIER : affected_by
      YIELD_ENGINE ||--o{ PRICE_MODIFIER : generates
      AI_OPERATIONS_AGENT }|--|| YIELD_ENGINE : configures_strategy
      AI_MARKETING_AGENT }|--|| YIELD_ENGINE : triggers_promotions
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Storefront
      participant YieldEngine
      participant InventoryLedger
      participant AIOpsAgent

      AIOpsAgent->>YieldEngine: Set Strategy (e.g., "Maximize Fill Rate for Tuesday")
      YieldEngine->>InventoryLedger: Observe low utilization for Tuesday slots
      YieldEngine->>YieldEngine: Calculate dynamic discount (-15%)
      YieldEngine->>Storefront: Publish updated price modifier
      Customer->>Storefront: View Tuesday availability
      Storefront-->>Customer: Display dynamic price (strikethrough original)
  ```

  ### Mobile UX Flow & UI Wireframes (375px)

  **Flow: The AI Price Nudge**
  1.  **Observation**: The AI Operations Agent notices low bookings for next week.
  2.  **The Nudge (Home Screen Card)**: A clean, translucent glass card appears on the OHC dashboard: "Next week is looking slow. Turn on 'Smart Booking Boost' to offer a 10% discount on empty slots?"
  3.  **Action**: User taps "Enable" (1-tap). No complex rules to configure.
  4.  **Result**: The Yield Engine applies the modifier. The AI Marketing Agent optionally drafts a promotional SMS to past customers.

  **Wireframe Description (Mobile Dashboard):**
  *   **Header**: Clean greeting, unified inbox summary.
  *   **Agent Suggestion Card (Glassmorphism)**:
      *   *Icon*: Sparkles / Chart.
      *   *Text*: "You have 8 open slots next week. Want to offer a 15% 'slow day' discount to fill them?"
      *   *Buttons*: [Yes, boost bookings] [No, keep prices]
  *   **Advanced Settings (Hidden)**: A toggle deep in settings for "Yield Strategy" (Maximize Profit vs. Maximize Volume), strictly for power users.

  ### AI Agent Integration Points
  *   **AI Operations Agent**: Monitors the Universal Capacity Ledger and Inventory Ledger. Triggers the Yield Engine when utilization drops below historical baselines or when peak demand is detected.
  *   **AI Finance Agent**: Ensures dynamic pricing does not violate minimum margin thresholds set by the tenant.
  *   **AI Marketing Agent**: When prices are dropped to fill capacity, this agent drafts and sends targeted campaigns (SMS/Email) to the CRM segments most likely to book.

  ### Key Design Decisions
  *   **Opt-In Nudges over Silent Changes**: For trust, the system will initially use "Nudges" for approval before changing prices, eventually allowing users to toggle "Fully Autonomous Mode" once they trust the AI.
  *   **Strict Margin Floors**: The system must never price an item/service below its cost or a user-defined minimum threshold, ensuring profitability.
  *   **Modifier-Based Architecture**: Base prices remain intact in the ledger. Dynamic pricing is implemented via transient `PriceModifiers` applied at checkout/display, ensuring clean accounting and easy rollbacks.

  ## Implementation Prompt
  **Objective**: Implement the core logic for the Yield Management Engine.
  **User Journey**: When a tenant has low utilization (e.g., < 30% booked for a given day next week), the system should automatically generate a `PriceModifier` recommendation. The UI should display this as an actionable nudge card. When accepted, the storefront should reflect the dynamically adjusted price.
  **Acceptance Criteria**:
  1.  The engine can ingest capacity metrics and generate a `PriceModifier` entity.
  2.  The modifier respects a hard-coded minimum margin floor (e.g., never discount > 30% without explicit manual override).
  3.  The API supports querying the "current effective price" which resolves the base price against active modifiers.
  4.  All changes must be multi-tenant safe and strictly isolated.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
