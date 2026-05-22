issue_title: "Autonomous Dynamic Pricing & Yield Management Engine"
issue_description: |
  # Title: Autonomous Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners frequently leave money on the table or face sudden overwhelming demand because their pricing is static. A bakery (Maya) might have excess inventory at the end of the day that goes to waste, while a handyman (Carlos) might be fully booked weeks in advance but charging his baseline rate for emergency weekend calls. Managing dynamic pricing or discounting manually is computationally complex, requires constant market monitoring, and adds significant cognitive load. They need an invisible yield management engine that automatically adjusts prices based on inventory levels, calendar utilization, local market demand signals, and time-of-day, without them ever opening a spreadsheet.

  ## Research Report
  *   **Current Limitations in SMB Tools**:
      *   *Shopify*: Requires complex third-party apps (e.g., Bold Custom Pricing) that non-technical users struggle to configure with rule-based logic.
      *   *Wix/Squarespace*: Offer basic manual coupons but lack autonomous, real-time demand-based pricing adjustments.
      *   *Enterprise Tools (Airlines/Hotels)*: Use highly sophisticated yield management (Sabre, Amadeus), which is entirely inaccessible to micro-businesses.
  *   **The OHC Advantage**: OHC possesses the core pillars required for autonomous pricing: a Unified Inventory Ledger, a Booking Engine, and an Operations AI Agent. By combining these, OHC can democratize enterprise yield management for solopreneurs.
  *   **Market Opportunity**: Providing dynamic pricing can directly increase gross margins by 5-15% for service and perishable-goods businesses, demonstrating immediate ROI to the user.
  *   **Risks**: Opaque pricing changes can erode consumer trust. The system must use psychological safety rails (e.g., "Surge pricing" is bad, but "Last-minute availability discount" or "Premium weekend slot" is acceptable).

  ## Design Doc

  ### High-Level Architecture

  ```mermaid
  erDiagram
      OPERATIONS-AGENT ||--o{ YIELD-ENGINE : "Monitors Capacity & Market"
      YIELD-ENGINE ||--o{ INVENTORY-LEDGER : "Checks Perishable Stock"
      YIELD-ENGINE ||--o{ BOOKING-ENGINE : "Checks Calendar Density"
      YIELD-ENGINE ||--o{ PRICING-RULES : "Applies Bounded Adjustments"

      PRICING-RULES ||--o{ PRODUCT-CATALOG : "Updates Live Price"
      PRICING-RULES ||--o{ SERVICE-QUOTES : "Adjusts Draft Quote"

      YIELD-ENGINE ||--o{ MARKETING-AGENT : "Triggers Flash Sale SMS"
  ```

  ### Mobile UX Flow (375px First)
  1. **Onboarding / Setup**: In the "Advanced Settings" of a product or service, the user sees a single toggle: "Auto-optimize pricing to maximize sales".
  2. **Constraint Configuration**: A simple slider sets bounds: "Never drop price below $X. Never raise above $Y."
  3. **The Yield Event**:
      *   *Scenario A (Surplus)*: It's 4:00 PM and Maya has 12 croissants left. The Operations Agent drops the price by 30% and triggers the Marketing Agent to send a push notification/SMS to her loyalty list: "Late afternoon special: 30% off croissants until 6 PM!"
      *   *Scenario B (Scarcity)*: Carlos's calendar is 95% booked for the next 7 days. The engine automatically applies a "High Demand" 20% premium to his remaining open slots and incoming quote requests.
  4. **Insights Widget**: The weekly plain-language brief includes: "Your smart pricing moved 15 extra items that usually go to waste, adding $45 to your week."

  ### AI Agent Integration Points
  *   **Operations Agent**: Monitors the velocity of sales (Inventory Ledger) and calendar fill rates (Booking Engine) against historical baselines.
  *   **Marketing Agent**: Executes localized, targeted outreach when a yield-optimization event (like a flash discount) occurs to stimulate demand.
  *   **Sales Agent**: When drafting custom quotes (e.g., for Carlos), it invisibly references the Yield Engine to adjust the margin based on current backlog.

  ### Key Design Decisions
  *   **Bounded Autonomy**: The AI can never set a price below the user-defined floor (cost of goods + baseline margin) to prevent catastrophic losses.
  *   **Transparent to User, Invisible to Customer**: The business owner sees the AI's logic ("Discounted due to impending expiration"). The customer just sees a great deal ("Happy Hour Special").
  *   **Multi-Tenant Ledger Isolation**: Pricing algorithms and localized demand signals must be strictly isolated per tenant to prevent cross-contamination of pricing strategies.

  ## Implementation Prompt

  **To Implementer**: Design and implement the autonomous Dynamic Pricing & Yield Management Engine module within the Operations AI domain.
  *   Create the underlying data model (`PricingRules`, `YieldEvents`) extending the product catalog and booking services.
  *   Implement the background job logic where the Operations Agent evaluates inventory decay (e.g., perishable goods nearing end of day) and calendar density (e.g., >90% booked within 48 hours).
  *   Expose internal APIs for the Marketing Agent to consume so it can broadcast flash sales when prices drop due to excess inventory.
  *   Ensure that all pricing updates are bounded by user-defined minimums and maximums.
  *   **Crucial**: Do not expose complex rule builders to the UI. The UI should consist of simple toggles and min/max sliders.

  **Acceptance Criteria**:
  1.  A product can be configured with an "auto-pricing" toggle and min/max price bounds.
  2.  A background agent job successfully identifies an "excess inventory" state and temporarily lowers the catalog price within bounds.
  3.  A background agent job successfully identifies a "high calendar density" state and raises the service quote price within bounds.
  4.  The system records the yield adjustment event in the tenant's ledger for inclusion in weekly briefings.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
