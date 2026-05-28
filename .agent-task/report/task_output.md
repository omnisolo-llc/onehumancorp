issue_title: "[Architecture] Autonomous Dynamic Pricing & Yield Management Engine"
issue_description: |
  # [Architecture] Autonomous Dynamic Pricing & Yield Management Engine

  **Title**: Architecting the Autonomous Dynamic Pricing & Yield Management Engine

  **Problem Statement**:
  Maya (baker) sells out her custom cakes every weekend but has zero orders on Tuesdays. Carlos (handyman) is booked solid during summer but quiet in winter. Fatima (food cart) has unsold food left over at the end of the day. They leave money on the table because they don't know how or when to adjust their prices, nor do they have the time. They shouldn't have to monitor competitor prices, track their own capacity, and manually update prices across all their listings to maximize revenue. They need OHC to automatically offer smart discounts during slow periods (yield management) and gently increase prices when demand is high (dynamic pricing) without them lifting a finger.

  **Research Report**:
  - **Competitor Analysis**:
    - *Shopify*: Requires expensive third-party apps for dynamic pricing. These apps are mostly rule-based (e.g., "discount if inventory > X") and far too complex for a non-technical SMB owner.
    - *Wix & Squarespace*: Offer manual coupons and scheduling, but completely lack autonomous yield management.
    - *Uber & Airbnb*: Utilize highly sophisticated dynamic pricing and yield management, maximizing revenue for hosts/drivers. This power is currently completely inaccessible to our OHC personas.
  - **Findings**: Small businesses lack the data analysis skills and time to perform dynamic pricing. By introducing an AI-driven background engine that analyzes local demand, historical sales velocity, and real-time capacity/inventory, OHC can dynamically adjust prices within user-defined bounds.
  - **Impact**: Significant increase in overall revenue and capacity utilization for time-based services (Carlos, Leo) and perishable goods (Fatima, Maya) by maximizing utilization and minimizing waste.

  **Design Doc**:

  *Architecture Diagram*:
  ```mermaid
  erDiagram
    PricingConfig ||--o{ ProductService : applies_to
    PricingConfig {
      float min_price
      float max_price
      boolean auto_pilot_enabled
    }
    DemandSignal_Ledger }|--|| AI_Pricing_Engine : inputs
    Capacity_Ledger }|--|| AI_Pricing_Engine : inputs
    AI_Pricing_Engine ||--|| AI_Finance_Agent : orchestrated_by
    AI_Pricing_Engine ||--o{ PriceUpdateEvent : generates
    PriceUpdateEvent }|--|| Universal_Catalog : updates
  ```

  *UI Wireframes & Mobile UX Flow (375px first)*:
  - **Screen 1 (Smart Pricing Setup)**: Adheres to the Visual Excellence Mandate. macOS-style Translucent Glass background with a clean Ubiquiti UniFi modular dashboard card. A simple toggle switch: "Auto-adjust prices to maximize sales".
  - **Screen 2 (Guardrails)**: If toggled on, two simple, touch-friendly sliders appear: "Lowest price I'll accept" and "Highest price to charge". No complex percentage rules, formulas, or developer jargon.
  - **Screen 3 (Insight Feed)**: The AI agent posts to the activity feed: "Increased cake prices by $5 this weekend due to high demand. +$150 expected revenue."

  *AI Agent Integration Points*:
  - **Operations Agent**: Continuously monitors the Universal Capacity and Inventory Ledgers.
  - **Finance Agent**: Analyzes historical sales velocity and local demand signals.
  - **Pricing Agent**: Coordinates with Finance and Ops to calculate optimal real-time pricing and pushes `PriceUpdateEvent`s to the Universal Catalog.

  *Key Design Decisions*:
  - **Zero-Config Rule**: Hide all complex pricing algorithms. The user only sets price bounds ("Grandmother test").
  - **Multi-Tenant Isolation**: Demand signals and pricing models are strictly isolated per tenant via SPIFFE/SPIRE identity. No cross-tenant data leakage.
  - **Offline-First & Edge Parity**: Base prices must be cached locally on the POS. Dynamic adjustments are applied asynchronously when online to ensure no latency during checkout.

  **Implementation Prompt**:
  "Implement the Autonomous Dynamic Pricing & Yield Management Engine for OHC. Create the background worker processes and AI agent orchestrator that can analyze a tenant's real-time capacity (via the Universal Capacity Ledger) and sales velocity to adjust catalog prices dynamically. The system must expose simple price bound configurations for the user (min/max price) and abstract away all algorithm complexity. Ensure all price updates are asynchronously propagated to the mobile POS edge caches and logged transparently in the tenant's activity feed. Do not design specific database tables or APIs; establish the domain logic, agent communication flow, and security boundaries."

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
