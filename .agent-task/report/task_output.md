issue_title: "Architecture: Universal AI-Driven Dynamic Pricing & Yield Management Engine"
issue_description: |
  # [architecture] Universal AI-Driven Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners—whether they are service providers like Leo (music tutor) trying to fill last-minute cancellations, or Fatima (food cart) trying to move perishable food before closing, or Maya (baker) dealing with peak holiday demand—lack the sophisticated yield management tools used by airlines and large e-commerce platforms. They manually adjust prices or, more often, eat the loss of empty slots and unsold perishable inventory. They need an invisible system that automatically optimizes pricing based on real-time capacity, perishability, and demand without them having to think about it.

  ## Research Report
  **Findings & Market Gap:**
  - **Competitors (Shopify, Wix, Squarespace):** Currently offer manual discount codes, sale prices, or basic third-party apps for dynamic pricing. These require setup, rules configuration, and constant monitoring.
  - **The OHC Opportunity:** By leveraging the Universal Capacity and Inventory Ledger alongside the AI Operations Department, OHC can dynamically adjust prices (e.g., last-minute discount on an empty tutoring slot, surge pricing on peak custom cake orders) entirely autonomously.
  - **Data:** Small businesses lose an estimated 15-20% of potential revenue due to sub-optimal pricing on expiring inventory or unbooked time slots.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      CapacityLedger ||--o{ YieldRule : triggers
      YieldRule {
          string rule_type
          float elasticity_factor
          datetime trigger_window
      }
      YieldRule }o--|| AIOpsDepartment : evaluated_by
      AIOpsDepartment ||--o{ PricingEngine : dispatches
      PricingEngine {
          float dynamic_price
          string reasoning
      }
      PricingEngine ||--|| StorefrontCache : updates
  ```

  ### UI Wireframes & Screen Flow (375px first)
  1. **The "Auto-Pilot" Toggle (Settings > Revenue):** A single iOS-style toggle card titled "Smart Pricing". Subtext: "Let AI adjust prices slightly to fill empty slots and clear unsold food."
  2. **The Impact Notification:** A push notification on the lock screen: "Leo, we filled your 4 PM cancellation by offering a 10% last-minute discount. +$45 revenue."
  3. **The Analytics Card:** A glassmorphic dashboard card showing "Revenue Lift from Smart Pricing: +$120 this week."

  ### Mobile UX Flow
  - The user flips a single switch during onboarding or in settings. No complex rules to configure.
  - The system operates invisibly. The only interaction is a celebratory notification when the AI successfully saves a lost sale or maximizes peak demand.
  - Developer terminology like "yield management" or "elasticity" is hidden entirely behind an "Advanced Settings" switch.

  ### AI Agent Integration Points
  - **Operations Department:** Monitors the CapacityLedger for nearing expirations (e.g., food shelf life, imminent calendar slots).
  - **Finance Department:** Calculates the optimal price floor to ensure profitability is maintained even when discounting.
  - **Marketing Department:** Generates plain-language copy for the storefront (e.g., "Last minute deal!" or "Only 2 left!") based on the dynamic price state.

  ### Key Design Decisions
  - **Opt-in Simplicity:** The feature must be a single toggle, abstracting away the complex rule engine required for yield management.
  - **Zero-Trust Multi-Tenancy:** Yield rules and pricing computations are strictly isolated per tenant (SPIFFE/SPIRE). Cross-tenant data cannot influence pricing without anonymized, aggregated opt-in (out of scope for MVP).
  - **Edge-Cached Storefronts:** Dynamic price updates must propagate to the edge cache within <50ms to prevent checkout price mismatches.

  ## Implementation Prompt
  **Objective:** Build the autonomous dynamic pricing engine that adjusts prices based on inventory perishability and calendar capacity.
  **User Journey (CUJ):** Leo has an empty tutoring slot tomorrow at 2 PM. At 24 hours out, the AI automatically reduces the price by 15% and updates the storefront. A student books it. Leo gets a notification that the AI saved a booking.
  **Acceptance Criteria:**
  - Create the core logic for evaluating capacity thresholds against time.
  - Integrate with the AI Operations agent to trigger price recalculations.
  - Ensure price updates are reflected in the mobile API payload under 50ms.
  - Implement the "Smart Pricing" toggle in the mobile-first revenue settings UI.
  - Do NOT prescribe database schemas; design the data model to fit seamlessly into our existing multi-tenant architecture.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
