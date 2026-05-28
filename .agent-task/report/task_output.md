issue_title: "Design Autonomous Hyper-Local Dynamic Pricing Engine"
issue_description: |
  # [Architecture] Autonomous Hyper-Local Dynamic Pricing Engine

  ## Problem Statement

  Small business owners lose significant revenue because they cannot continuously adapt their pricing to match local demand, weather, inventory levels, or competitor actions in real-time.

  - **Fatima (food cart)**: On a hot day at the festival, she sells out of cold drinks instantly while hot food sits. If she had raised drink prices slightly when demand spiked and temperatures hit 95°F, she could have managed inventory better and increased margin.
  - **Leo (music tutor)**: His 4:00 PM slot is always booked, but his 11:00 AM slots sit empty. He needs an effortless way to offer "happy hour" pricing or last-minute discounts to fill his schedule without manually tweaking his calendar every day.
  - **Maya (baker)**: When she has excess perishable inventory (like croissants) at 3 PM, she needs to immediately push a discount to clear them out before closing, rather than throwing them away.

  Current tools require business owners to manually adjust prices, create coupon codes, or change calendar rates, which they forget to do or are too busy to manage. They need a system that autonomously detects hyper-local signals (weather, foot traffic, inventory velocity, time of day) and seamlessly adjusts pricing (within predefined limits) to optimize for revenue and zero waste.

  ## Research Report

  We investigated how large enterprises (airlines, Uber, Amazon) utilize dynamic pricing and how SMB platforms attempt to democratize it. The findings show that while large players use dynamic pricing aggressively, SMB platforms only offer manual discounting.

  ### Competitive Analysis

  | Platform | Dynamic Pricing Capability | Key Constraint |
  |---|---|---|
  | Shopify | Third-party apps only | Requires complex rules setup; not natively integrated with offline physical stores. |
  | Wix | Manual coupons & sales | No automated demand-based pricing; entirely manual intervention. |
  | Square | Time-based happy hours | Static rules only; does not adapt to real-time inventory velocity or external factors like weather. |
  | **OHC (Target)** | **Native, Autonomous, Omni-channel** | **Requires zero manual rule configuration; driven by AI evaluating local context (weather, time, inventory).** |

  ### Industry Findings

  - Yield management can increase revenue by 10-20% for perishable goods or time-based services.
  - SMB owners are overwhelmed by "if/then" rule builders. The system must simply ask, "Do you want AI to automatically discount items that are about to expire?" or "Do you want to charge a premium during peak rush hours?"
  - Transparency is critical. The buyer must understand *why* the price changed (e.g., "Last minute booking discount!" or "High demand pricing").

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      PricingEngine ||--o{ ContextNode : "Evaluates"
      ContextNode {
          string Type "Weather, Time, InventoryVelocity, Competitor"
          string Value
      }
      PricingEngine ||--o{ ProductVariant : "Adjusts Price For"
      PricingEngine ||--o{ ServiceTimeSlot : "Adjusts Price For"
      ProductVariant {
          float BasePrice
          float MinPrice
          float MaxPrice
      }
      ServiceTimeSlot {
          float BasePrice
          float MinPrice
          float MaxPrice
      }
      PricingEngine ||--|| AIDepartment : "Receives Strategy From"
      AIDepartment {
          string Goal "ClearInventory, MaximizeRevenue, FillSchedule"
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant MobileUI as OHC Mobile/Web UI
      participant PricingEngine
      participant ContextMesh as Local Context Mesh (Weather, Time)
      participant Inventory as Inventory/Capacity Ledger

      Customer->>MobileUI: Views Menu / Booking Calendar
      MobileUI->>PricingEngine: Request Prices (Location, Time)
      PricingEngine->>ContextMesh: Fetch local signals (Temp: 95F, Time: 2PM)
      PricingEngine->>Inventory: Check stock velocity (Cold drinks selling fast)
      PricingEngine-->>MobileUI: Return dynamically adjusted prices
      MobileUI-->>Customer: Display Price (e.g., "$4.00 (High demand)")
      Customer->>MobileUI: Purchases Item
      MobileUI->>Inventory: Deduct Stock
  ```

  ### Mobile UX Flow (375px First)

  **Owner Configuration (Maya / Leo)**
  1. **Dashboard Home**: A single card appears: "Enable Smart Pricing to sell out remaining inventory today. [Turn On]".
  2. **Settings Modal**: Clean, translucent glass UI.
     - **Toggle**: "Auto-discount perishables 2 hours before closing."
     - **Toggle**: "Surge pricing during high demand."
     - **Slider**: "Maximum price adjustment (+/- 20%)" - keeps it safe and predictable.
     - **Preview**: A small chart showing how a $10 item might fluctuate between $8 and $12.

  **Customer Experience**
  1. **Storefront / Calendar**: The user browses on their phone.
  2. **Dynamic Price Display**: The price tag shows a gentle visual cue (e.g., a small green down arrow with "Happy Hour!" or a small flame icon with "High Demand").
  3. **Checkout**: The adjustment is clearly itemized in the cart as a "Dynamic Adjustment" or "Last Minute Discount" to build trust.

  ### AI Agent Integration Points

  - **Finance / Operations Department**: The AI monitors inventory velocity. If it sees croissants aren't selling as fast as usual by 2 PM, it triggers the Pricing Engine to lower the price.
  - **Marketing Department**: The AI can automatically push an Instagram/WhatsApp update: "Last minute slots opened up for today! 20% off if you book now."
  - **Context Awareness**: The AI pulls local weather data. If it starts raining near Fatima's food cart, it might lower prices to entice people to stick around, or push hot food over cold.

  ### Key Design Decisions & Rationale

  - **Bounds over Rules**: Instead of asking users to build complex "If X then Y" rules, we simply ask them to set a minimum and maximum price boundary (e.g., +/- 20%). The AI handles the logic. This passes the "grandmother test."
  - **Context Mesh Separation**: The pricing engine does not hardcode weather or time APIs; it subscribes to a generic "Context Mesh" that provides normalized local signals.
  - **Transparency for Trust**: Dynamic pricing can feel predatory if hidden. We mandate that the UI always explains the adjustment (e.g., "Clearance," "Peak Hours") so customers feel informed.

  ## Implementation Prompt

  **Objective**: Implement the underlying Autonomous Hyper-Local Dynamic Pricing Engine and its associated mobile-first configuration UI.

  **User Journey (CUJ)**:
  1. As an OHC merchant, I want to enable "Smart Pricing" with a single tap, defining a safe price range (e.g., my $10 item can go down to $8 or up to $12).
  2. As the system, I will continuously evaluate local context (time of day, inventory velocity) and automatically adjust the active price within those bounds to maximize revenue or clear inventory.
  3. As a customer, I will see transparent, dynamically adjusted prices on the storefront or booking calendar with clear reasons (e.g., "Last Minute Deal!").

  **Acceptance Criteria**:
  - A merchant can toggle dynamic pricing on a per-product or per-service basis via the mobile app.
  - The merchant can set a minimum and maximum price floor/ceiling.
  - The system can accept external context signals (mocked for now, like "high demand" or "closing soon") and adjust the price returned to the storefront in real-time.
  - The storefront UI displays the dynamically adjusted price alongside a clear, localized explanation tag.
  - All pricing logic must respect multi-tenant boundaries (one tenant's high demand does not affect another's prices).
  - The solution must include telemetry to track how much extra revenue or saved inventory the dynamic pricing generated.

  ## Priority
  `P1`

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
