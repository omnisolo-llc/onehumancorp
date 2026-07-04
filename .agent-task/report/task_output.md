issue_title: "Implement Zero-Touch Dynamic Pricing & Yield Management Architecture"
issue_description: |
  # Research Report: Zero-Touch Dynamic Pricing & Yield Management Architecture

  ## 1. Problem Statement
  Small business owners (such as Leo the music tutor or Priya the boutique operator) often struggle with maximizing their revenue because they cannot continuously monitor market demand or manually adjust their prices in real-time. They leave money on the table during peak hours and lose out on potential sales during off-peak times. While enterprise solutions like Uber or airlines use dynamic pricing (yield management) to optimize revenue, these systems are far too complex, expensive, and technical for small operators.

  ## 2. Research Report
  - **Market Context:** Traditional SME platforms like Shopify, Wix, and Squarespace only allow static pricing. Modifying prices dynamically requires expensive enterprise integrations or highly manual daily adjustments. Pricing is treated as a static integer, ignoring the realities of supply and demand for services and limited inventory.
  - **The OHC Opportunity:** OHC can democratize yield management by integrating a "Zero-Touch Dynamic Pricing Engine". By leveraging the AI Finance Agent, the platform can automatically adjust pricing for bookings or inventory based on real-time factors like remaining availability, historical demand patterns, and current velocity.
  - **Competitor Gaps:**
    - *Shopify:* Static pricing. Scheduled sales are possible, but true demand-based dynamic pricing requires heavy 3rd-party apps (e.g., Prisync) which are B2B focused.
    - *Wix/Squarespace:* No native dynamic pricing.
    - *Mindbody (for services):* Basic off-peak pricing, but not autonomously managed by AI.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `smart_pricing_policies`: Rules defining the boundaries of dynamic pricing (e.g., minimum price floor, maximum price ceiling, target utilization rate).
  - `active_discounts`: Dynamically applied price modifiers based on current demand.

  ### AI Integration
  - **The Accountant (Finance Agent):** Continuously monitors the utilization of services (e.g., booked time slots) and inventory.
  - If utilization > 80% for a specific day, it proactively increases the price of remaining slots to the maximum ceiling to maximize yield.
  - If utilization < 30% and the date is approaching, it automatically creates a temporary targeted promotion or reduces the price to stimulate demand.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Requests Booking/Checkout] --> B(API Server)
      B --> C{Yield Management Engine}
      C -->|Read| D[(Postgres: Inventory/Bookings)]
      C -->|Read| E[(Postgres: smart_pricing_policies)]
      C -->|Calculate| F[Dynamic Price]
      F --> G[Return Price to Checkout]
      H[Finance Agent] -->|Analyze Patterns| E
      H -->|Alert Owner| I[Mobile Feed]
  ```

  ### Mobile UX Flow (375px)
  1. **Owner View (Dashboard):** The owner sees an "Action Card" in their feed: "Demand for Saturday is high. The Accountant has adjusted your cake pricing by +15% to maximize yield. [View Details] [Revert]".
  2. **Configuration View:** A simple toggle on the Product/Service screen: "Enable Smart Pricing". The user sets a "Base Price" and a "Minimum Acceptable Price". No complex math required.
  3. **Customer View:** The customer sees the final optimized price. (Optionally, a "High Demand" badge can create urgency).

  ## 4. Implementation Prompt
  **Feature Name:** OHC Zero-Touch Dynamic Pricing Engine
  **Target Persona:** Leo the Music Tutor / Priya the Boutique Operator
  **Outcome:** Users can toggle "Smart Pricing" on their services or limited inventory. The system will autonomously adjust the checkout price based on real-time demand and availability to maximize the owner's revenue, without requiring manual intervention.

  **Next Actions:**
  1. Implement the `smart_pricing_policies` and `active_discounts` tables with strict multi-tenant isolation.
  2. Update the `Checkout` and `Booking` backend services to query the Yield Management Engine for the dynamic price before creating a Payment Intent.
  3. Extend the Finance Agent to run periodic yield analysis jobs and adjust active discounts/premiums based on utilization.
  4. Build the mobile-first (375px) toggle for "Smart Pricing" on the Product/Service detail screen, hiding complex settings behind an "Advanced" accordion.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
