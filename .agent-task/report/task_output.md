issue_title: "Autonomous Dynamic Yield & Surge Pricing Engine"
issue_description: |
  # Autonomous Dynamic Yield & Surge Pricing Engine

  ## Problem Statement
  Small business owners frequently leave money on the table or face unmanageable spikes in demand because their pricing is static.
  - **Leo (music tutor)** has empty slots on Tuesday mornings that go unfilled, while his weekend slots are booked solid. He has no time or expertise to manually discount slow periods.
  - **Maya (baker)** goes viral on Instagram and suddenly receives 100 requests for custom vegan cakes in an hour. Her static pricing means she misses out on premium rush-order revenue and gets overwhelmed.
  - **Carlos (handyman)** wants to incentivize bookings during his slow winter months but doesn't know how to run a structured promotion.
  Existing platforms like Shopify or Wix require owners to manually configure complex discount codes, install third-party "yield management" apps, or manually adjust prices daily. This is impossible for a non-technical owner running their business from a phone.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Requires third-party apps (e.g., Bold Custom Pricing or Prisync) which often cost $30-$100/mo. These apps are mostly rule-based (e.g., if inventory < X, price = Y) and lack AI-driven demand prediction.
  - **Wix:** Basic discount features available, but dynamic yield management (like airline pricing for service slots) is non-existent out of the box. Users must manually manage coupons.
  - **Squarespace:** No native dynamic pricing. Everything is static.
  - **Uber/Airlines:** The gold standard for dynamic pricing, utilizing massive data to optimize yield.

  **The Gap for OHC:**
  SMBs need enterprise-grade yield management (like airlines or Uber surge pricing) but with zero configuration. The system must autonomously detect supply (inventory/time slots) and demand (traffic, booking velocity), and dynamically adjust pricing or offer incentives to maximize revenue and smooth out operational spikes, communicating this transparently to the business owner via the daily briefing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Frontend - Mobile First
          A[Mobile Storefront] --> B(Dynamic Pricing Layer);
          B --> C[Unified Inventory & Capacity Mesh];
      end
      subgraph OHC Backend
          C --> D[(Ledger & Inventory DB)];
          E[Traffic & Velocity Monitor] --> F(AI Finance Agent);
          D --> F;
          F --> G(Dynamic Yield Configuration Config);
          G --> B;
      end
      subgraph AI Operations
          F --> H(Notification & Briefing Engine);
      end
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  **User Flow (Business Owner - Maya):**
  1. **Notification:** Maya receives an OHC push notification: "Demand spike detected! I've activated Surge Pricing (+15%) for weekend orders to maximize revenue and manage your load."
  2. **Review Screen (375px):** A clean, translucent glass card shows the current pricing multiplier and a simple toggle: "Enable Smart Surge".
  3. **Advanced Settings (Hidden):** If expanded, shows threshold settings (e.g., "Max surge +20%", "Trigger when >5 orders/hr").

  **User Flow (Customer - Leo's Student):**
  1. **Storefront (375px):** When viewing Leo's booking calendar, peak times (Weekends) show standard price ($50). Tuesday morning slots show a subtle "Off-Peak Savings" tag ($40).
  2. **Checkout:** The discount is automatically applied and clearly labeled as a "Smart Booking Discount" without needing a coupon code.

  ### Key Design Decisions
  - **Zero-Config Default:** The AI Finance Agent will default to suggesting yield optimizations based on historical data. The owner only needs to approve via a 1-tap notification.
  - **Transparent Customer Experience:** Price adjustments (discounts or surges) must be clearly explained to the customer to maintain trust (e.g., "High demand" or "Off-peak discount").
  - **Multi-Tenant Isolation:** The pricing multiplier configurations must be strictly isolated by tenant ID within the `Dynamic Yield Configuration Config`.
  - **Performance Targets:** The `Dynamic Pricing Layer` must calculate prices at the edge (sub 50ms latency) without blocking the storefront rendering, using cached baseline prices and multipliers.

  ## Implementation Prompt
  **Context for Implementer:**
  You are building the "Autonomous Dynamic Yield & Surge Pricing Engine". The goal is to allow the OHC platform to automatically adjust prices for both physical goods and service bookings based on supply and demand velocity.

  **Core User Journeys (CUJs):**
  1. The AI Finance Agent detects high booking velocity and proposes a temporary price surge to the business owner.
  2. A customer views a service calendar and sees off-peak discounts applied automatically to low-demand time slots.

  **Acceptance Criteria:**
  - The data model must support dynamic price multipliers attached to specific inventory items or time slots.
  - The pricing calculation logic must apply these multipliers dynamically at checkout and storefront display.
  - There must be an API for the AI Agent to adjust these multipliers based on business rules.
  - Mobile-first UI components for both the owner's notification approval and the customer's storefront view.
  - Comprehensive unit and integration tests verifying pricing accuracy and multi-tenant data isolation.
  - Ensure 375px viewport parity and premium macOS-style translucent glass UI for owner notifications.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []