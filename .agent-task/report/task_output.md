issue_title: "[architecture] Autonomous Dynamic Yield & Pricing Engine"
issue_description: |
  # Autonomous Dynamic Yield & Pricing Engine

  ## Problem Statement
  Small businesses, particularly service providers (Carlos the handyman, Leo the music tutor) and bespoke creators (Maya the baker), routinely leave money on the table or face burnout because they manually manage "rush" fees, last-minute cancellations, and peak demand pricing. The non-technical business owner cannot sit and tweak pricing models all day; they need an engine that automatically dynamically yields their capacity, just like airlines or ride-shares, but invisibly and politely.

  ## Research Report
  *   **The Gap:** Legacy platforms like Shopify or Wix allow for static variant pricing but lack dynamic, capacity-aware yield management. Pricing is rarely tied to real-time calendar density or inventory velocity without installing complex third-party apps.
  *   **Competitor Analysis:** Shopify/Wix requires manual setup. Acuity/Calendly allows setting different appointment types, but does not autonomously surge pricing based on real-time calendar saturation.
  *   **The OHC Advantage:** Because OHC has a Unified Capacity and Inventory Ledger, an AI agent can analyze real-time demand and autonomously adjust pricing or require larger deposits without the owner lifting a finger.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Client [Mobile UI]
          C[Consumer Booking/Cart]
          O[Owner Dashboard]
      end
      subgraph OHC API Gateway
          API[GraphQL/REST API]
      end
      subgraph Operations Department AI
          DE[Dynamic Yield Agent]
          CA[Capacity Analyzer]
          PM[Pricing Model Synthesizer]
      end
      subgraph Core Data Layers
          TL[Unified Capacity & Inventory Ledger]
          OL[Order/Booking Ledger]
      end
      C -->|Request Quote/Slot| API
      API --> DE
      DE --> CA
      CA <--> TL
      CA <--> OL
      CA -->|Capacity Saturation Metric| PM
      PM -->|Adjusted Price| DE
      DE -->|Quote/Price| API
      O -->|Set Boundaries i.e. max 20% surge| API
      API --> PM
  ```

  ### Key Design Decisions
  1.  **Invisible by Default, Bounded by Owner:** The system must not accidentally charge a customer 10x the normal price. The owner sets simple, plain-language boundaries during onboarding (e.g., "Allow up to a 20% premium for rush jobs or when I'm almost fully booked").
  2.  **Polite Presentation:** The UI must present the dynamic pricing to the end consumer as a feature, not a penalty. (e.g., "Popular Slot - Remaining capacity is low").
  3.  **Cross-Domain Application:** This engine must apply equally to a physical product with low inventory (Maya's last cake slot) and a service booking (Carlos's last Friday slot). It taps into the underlying Unified Capacity and Inventory Ledger.

  ### UI Flow & Mobile UX (375px First)
  1.  **Consumer Booking Flow:** The user taps a date/time on the booking widget. If the date is 90% saturated, a small, elegant badge appears: `⚡ High Demand: +$15 Premium`. The total updates instantly.
  2.  **Owner Insight Notification:** Instead of a complex settings page, Maya receives a daily plain-language push notification: "Your weekend slots filled up fast! The Yield Engine automatically added rush fees to the last 3 orders, earning you an extra $45."
  3.  **Owner Advanced Settings (Hidden by default):** Accessible via "Advanced Settings" switch. Simple sliders for "Max Surge %" and "Discount for slow days %".

  ## Implementation Prompt
  **Objective:** Implement the backend logic and data models for the Autonomous Dynamic Yield Pricing Engine, integrating with the Unified Capacity and Inventory Ledger.

  **User Journey (CUJ):**
  1. An owner opts into "Smart Pricing".
  2. A consumer attempts to book a service or buy a product.
  3. The system queries the capacity ledger before displaying the price.
  4. If capacity is below a threshold, the system autonomously applies a configured premium percentage.
  5. The final quoted price includes the transparent premium line item.

  **Acceptance Criteria:**
  *   Create necessary database models or extensions to the existing Capacity/Inventory ledger to store pricing boundaries per tenant/item.
  *   Implement the core algorithmic service that calculates the dynamic price based on real-time saturation.
  *   Ensure the engine fails gracefully (falls back to base price) if the calculation times out.
  *   Expose an API endpoint for the frontend to query the "current quoted price" given an item/timeslot, which executes the dynamic yield logic.
  *   Do not prescribe the exact AI prompt or the UI implementation in this step; focus on the high-performance calculation engine and data structure.
  *   Maintain strict multi-tenant isolation.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
