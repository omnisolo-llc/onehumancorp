issue_title: "[Architecture] Autonomous Dynamic Pricing & Yield Management Engine"
issue_description: |
  # Issue Brief: Autonomous Dynamic Pricing & Yield Management Engine

  ## Title
  [Architecture] Autonomous Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners frequently lose revenue due to perishable inventory or expiring time slots.
  - **Fatima (Food Cart)** preps 50 Halal chicken platters for the lunch rush. At 1:30 PM, she has 15 left that will spoil if unsold, but she is too busy cooking to manually log in, calculate a discount, and send a blast to her WhatsApp group.
  - **Leo (Music Tutor)** has a cancellation for a 4:00 PM lesson today. It remains unbooked because he isn't actively marketing that specific slot to his waitlist.

  Enterprise systems (airlines, hotels) use yield management to solve this, dynamically adjusting prices to guarantee sell-through. SMBs lack this entirely. Current platforms (Shopify, Wix) treat prices as static and require manual intervention to create discount codes or send emails.

  ## Research Report
  - **Competitor Landscape**: Shopify and Wix allow discounts, but they are static rules (e.g., "10% off on Tuesdays"). They do not autonomously monitor real-time capacity and react.
  - **The Gap**: SMBs need an invisible yield manager. They need the system to notice "We have excess capacity expiring in 2 hours" and automatically calculate a safe, margin-positive discount and broadcast it to past customers to recover revenue that would otherwise drop to zero.
  - **AI Differentiation**: Instead of forcing the owner to be a data analyst, the Autonomous Dynamic Pricing Engine acts as a Revenue Manager. It analyzes the `Universal Capacity and Inventory Ledger`, detects at-risk capacity, drafts a localized marketing blast, and requests a 1-tap approval from the owner.

  ## Design Doc

  ### High-Level Architecture & Business Journey
  1. **Detection Phase**: The `YieldAgent` constantly monitors the `Universal Capacity and Inventory Ledger` against historical sell-through rates.
  2. **Strategy Generation**: If an anomaly is detected (e.g., "15 platters unsold 1 hr before close"), the `YieldAgent` triggers a `PricingStrategyEvent`.
  3. **Calculation Phase**: The `FinanceAgent` ensures the proposed flash sale price (e.g., 30% off) remains above the COGS baseline for margin safety.
  4. **Action Draft**: The `MarketingAgent` drafts a compelling, localized message (e.g., "Lunch rush over? Grab a Halal platter for 30% off in the next hour!").
  5. **Approval Flow**: An Optimistic UI card appears in the owner's Activity Feed. Upon 1-Tap approval, the system updates the localized price on the edge-cached storefront and dispatches the message via the Omnichannel Inbox (WhatsApp/SMS).

  ### Entity Relationship Diagram (Mermaid.js)

  ```mermaid
  erDiagram
      TENANT ||--o{ INVENTORY_ITEM : owns
      TENANT ||--o{ BOOKING_SLOT : offers
      INVENTORY_ITEM ||--o{ YIELD_STRATEGY : triggers
      BOOKING_SLOT ||--o{ YIELD_STRATEGY : triggers
      YIELD_STRATEGY ||--|| DYNAMIC_PRICE_ADJUSTMENT : generates

      YIELD_STRATEGY {
          string id
          string target_entity_id
          string target_entity_type
          float predicted_spoilage_risk
          datetime expiration_window
      }

      DYNAMIC_PRICE_ADJUSTMENT {
          string id
          float original_price
          float adjusted_price
          string marketing_draft_copy
          string approval_status
      }
  ```

  ### AI Department Coordination (Mermaid.js Sequence)

  ```mermaid
  sequenceDiagram
      participant CapacityLedger
      participant YieldAgent
      participant FinanceAgent
      participant MarketingAgent
      participant MobileUI
      participant StorefrontEdge

      YieldAgent->>CapacityLedger: Polling/Event: 15 platters remaining at 1:30 PM
      YieldAgent->>FinanceAgent: Request safe margin for flash sale
      FinanceAgent-->>YieldAgent: 30% max discount approved
      YieldAgent->>MarketingAgent: Draft SMS blast for 30% off Halal platters
      MarketingAgent-->>MobileUI: Push Optimistic Card to Activity Feed
      MobileUI->>MobileUI: Owner taps "Approve Flash Sale"
      MobileUI->>YieldAgent: Approval Event
      YieldAgent->>StorefrontEdge: Update item price (TTL: 1 hour)
      YieldAgent->>MarketingAgent: Execute WhatsApp/SMS blast
  ```

  ### Technical Integrity & Mobile-First Review

  #### Mobile-First UX Flow (375px Viewport)
  - **Activity Feed Card**: A macOS-style Translucent Glass card appears at the top of Fatima's feed.
    - **Header**: "Revenue Recovery Alert 🚨"
    - **Body**: "15 platters remain. Suggestion: 30% off Flash Sale for the next hour to clear inventory. We'll text your 40 local regulars."
    - **Interaction**: Two large, Unifi-style buttons: `[1-Tap Approve]` (Primary, solid brand color) and `[Dismiss]` (Secondary, ghost outline).
    - **Grandmother Test**: No complex settings. Fatima doesn't need to know what a discount code or customer segment is. She just says "Yes" to making money.

  #### Performance & Offline Targets
  - **Edge Pricing Latency**: The price update to the storefront must be pushed to the edge cache in **< 50ms**.
  - **Optimistic UI**: The 1-Tap approval must provide instant haptic feedback and transition the card to a "Flash Sale Active" state locally, handling the background sync via the Hybrid Event Mesh without blocking the user.

  #### Zero Trust & Security
  - **Multi-Tenant Isolation**: The `YieldAgent` operates strictly within the context of a single `tenant_id`. SPIFFE/SPIRE certificates guarantee that the Yield worker processing Fatima's data cannot query Leo's calendar data.
  - **Financial Guardrails**: The `FinanceAgent` is a hard dependency; a dynamic price adjustment can *never* execute if the resulting price drops below the defined COGS threshold, ensuring the system never bankrupts the user autonomously.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the `Autonomous Dynamic Pricing & Yield Management Engine`. Create the core entities (`YieldStrategy`, `DynamicPriceAdjustment`) with strict multi-tenant isolation. Implement the background `YieldAgent` worker that monitors the `UniversalCapacityLedger` for expiring inventory/time slots based on configurable temporal thresholds. Connect the agent to the `Hybrid Event Mesh` to emit a `StrategyDrafted` event. Construct the mobile-first UI card component (in Rust/Slint or the target mobile framework) that consumes this event and presents a 1-tap approval interface, hiding all complexity behind a simple "Approve Flash Sale" button. Ensure the price update propagates to the edge-cached storefront. Do not prescribe specific database ORMs; focus on the business logic, state transitions, and event flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
