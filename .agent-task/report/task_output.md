issue_title: "[Feature] Autonomous Dynamic Yield & Smart Pricing Engine"
issue_description: |
  ## Title
  [Feature] Autonomous Dynamic Yield & Smart Pricing Engine

  ## Problem Statement
  Small business owners like Carlos (handyman) and Leo (music tutor) struggle with optimizing their prices based on demand, seasonality, and their current workload. They leave money on the table when demand is high and lose out on bookings when demand is low because they lack the time and data to manually adjust pricing. A non-technical small business owner needs an invisible engine that dynamically adjusts quoting and pricing to maximize yield without requiring manual intervention or complex dashboard management.

  ## Research Report
  - **Market Gap:** Competitors like Wix and Squarespace offer static pricing with basic discount codes. Shopify has apps for dynamic pricing, but they are expensive, complex to configure, and heavily biased toward physical goods rather than services or bookings.
  - **OHC Advantage:** OneHumanCorp's unique Agentic Teammate Model can monitor the Universal Capacity & Inventory Ledger in real-time. By deploying the Operations and Sales Agents, OHC can automatically suggest or enforce smart pricing (e.g., charging a 15% premium for Carlos's last available emergency repair slot on a Friday, or offering a 10% discount for Leo's off-peak Tuesday morning slots).
  - **Competitor Analysis:**
    - Shopify: Requires third-party apps ($30-$100/mo), complex rule configuration.
    - Wix: Manual peak-pricing only.
    - OHC: Zero-config. AI agent analyzes historical data, competitor pricing, and current capacity to adjust quotes dynamically.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Acquisition & Booking
          User[Customer] -->|Requests Quote / Views Slot| App[Mobile Storefront / Booking View]
      end

      subgraph OHC Backend
          App --> API[OHC Rust API]
          API --> Orchestrator[KAIROS Orchestrator]

          Orchestrator -->|Checks Capacity| Ledger[(Universal Capacity Ledger)]
          Orchestrator -->|Triggers| SalesAgent[Sales & Pricing AI Agent]

          SalesAgent -->|Reads rules| Memory[(Business Memory/Rules)]
          SalesAgent -->|Calculates Premium/Discount| PriceEngine[Dynamic Yield Engine]
      end

      PriceEngine -->|Returns Dynamic Price| API
      API -->|Displays Price| App
  ```

  ### UI & UX Flow (Mobile-First 375px)
  - **Customer View:** When viewing a high-demand slot or requesting an urgent service, the UI displays a clean, premium card indicating "High Demand Pricing" with a subtly highlighted price, adopting macOS-style Translucent Glass materials.
  - **Owner View (The Grandmother Test):** Carlos does not configure complex rules. He receives an actionable card in his Activity Feed: "Your Friday afternoon slots are booking up fast. Want to enable Smart Pricing to charge a 15% premium for remaining slots?" with a simple 1-tap "Enable" or "Ignore" button.
  - **Zero Trust & Security:** All pricing adjustments are strictly scoped to the tenant's multi-tenant isolation boundary, validated via SPIFFE/SPIRE workload identity.
  - **Performance:** Dynamic pricing calculations must occur within the same latency budget as a standard catalog lookup (<200ms) by utilizing edge-caching for base prices and fast Redis lookups for capacity multipliers.

  ## Implementation Prompt
  Implement the Dynamic Yield & Smart Pricing Engine for the KAIROS Orchestrator.
  1. Extend the `SalesAgent` to subscribe to capacity threshold events from the Universal Capacity Ledger.
  2. Create a dynamic pricing endpoint that accepts a service/product ID and a requested time/quantity, returning the dynamically calculated price.
  3. Build the mobile-first approval card component in the Tauri v2 frontend (React/HTML) allowing the business owner to enable Smart Pricing with a single tap.
  4. Ensure all responses meet strict <200ms latency targets and adhere to the Zero-Trust multi-tenant isolation rules. Do not expose any configuration dashboards; keep the logic invisible and agent-driven.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
