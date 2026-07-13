issue_title: "Implement Autonomous AI-Driven Dynamic Pricing Engine"
issue_description: |
  # Research Report: Autonomous AI-Driven Dynamic Pricing & Yield Management Engine

  ## 1. Problem Statement
  Small business owners like Priya (Boutique Owner) and Leo (Music Tutor) struggle with setting optimal prices for their products and services. They lack the time, tools, and expertise to monitor competitor pricing, analyze local demand surges, or adjust prices based on inventory levels or booking availability. This leads to either underpricing (leaving money on the table) or overpricing (losing potential customers). Pricing is often a "set and forget" guess. They need an intelligent system that automatically adjusts prices or offers targeted promotions to clear out stagnant inventory or fill empty calendar slots, maximizing revenue without requiring manual spreadsheets or complex discount code setups.

  ## 2. Market Mapping & Competitor Discovery
  - **Shopify**: Requires third-party apps (e.g., "Dynamic Pricing", "Prisync", "StreetPricer") which are complex to configure and cost $50-$200+/month. No native AI dynamic pricing exists for everyday merchants.
  - **Wix & Squarespace**: Offer basic manual discount codes and sale prices. No native AI-driven yield management.
  - **Airlines/Hotels**: Utilize sophisticated yield management (e.g., dynamic pricing based on availability), but this technology is entirely inaccessible to small businesses.

  ## 3. OHC Opportunity
  OHC can democratize yield management by integrating an AI Dynamic Pricing Engine directly into the Finance & Payments ("The Accountant") and Operations ("The Manager") departments. The AI will monitor sales velocity, upcoming calendar availability, and seasonal trends to proactively suggest dynamic pricing adjustments (e.g., "Increase custom cake prices by 10% for the busy wedding season") or targeted promotions (e.g., "Offer a 15% discount on remaining Tuesday guitar slots"). The owner simply taps "Approve" on a mobile notification.

  ## 4. Architecture Design & Deep Dive

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Business Advisory Agent] -->|Analyzes| B(Inventory & Booking Data)
      B -->|Identifies| C{Stagnant Items / Empty Slots}
      C -->|Triggers| D[Finance & Payments Agent]
      D -->|Calculates Optimal Discount / Price| E[Dynamic Pricing Engine]
      E -->|Proposes Action| F[Agent Feed]
      F -->|Owner Approval| G[(Multi-Tenant DB)]
      G -->|Notifies| H[Customer Success Agent]
      H -->|Emails/SMS| I[Targeted Customers]
  ```

  ### Data Model & Sync Protocol
  1. Extend the `Product` and `Service` data models to support dynamic pricing rules, including `base_price`, `minimum_margin`, `max_price`, and `active_promotional_modifiers`.
  2. Implement a background worker (using the OHC Job Queue) that periodically evaluates inventory velocity and calendar utilization against demand signals.

  ### Mobile UX Flow (375px First)
  - **Agent Feed Integration**: The system pushes actionable cards to the Unified Agent Feed. E.g., "✨ 3 items are moving slow. Run a flash sale to clear them out?"
  - **Preview Screen**: A clean, translucent card showing the items, suggested discount, and estimated revenue.
  - **Actions**: Two large (≥ 44x44px) touch targets: "Approve & Notify Customers" (Primary) and "Adjust Details" (Secondary).
  - **Opt-in Autonomy**: The system suggests the pricing change first, requiring a 1-tap approval. As trust builds, users can enable "Auto-Pilot" for specific categories.

  ## 5. Implementation Prompt
  **Objective**: Build the user-facing flow, data model extensions, and background worker structure for the Dynamic Pricing & Yield Management Engine.

  **Critical User Journey (CUJ)**:
  1. Priya logs into the OHC mobile app.
  2. She navigates to the Unified Agent Feed.
  3. She sees an AI Advisory Card suggesting a 20% discount on "Summer Hats" because they haven't sold in 30 days.
  4. She taps "Approve & Run Sale".
  5. The system automatically updates the storefront price, and the Marketing Agent drafts an email to her past customers.

  **Acceptance Criteria**:
  - Extend the data schema to support pricing bounds without prescribing specific SQL.
  - Ensure all screens are fully responsive down to 375px.
  - Use OHC's Glassmorphism tokens for the suggestion cards in the Unified Agent Feed.
  - Implement the "Approve" flow with optimistic UI updates.
  - Ensure the background pricing update adheres to strict multi-tenant isolation boundaries.
  - Implement full E2E Playwright tests covering the configuration flow and the presentation of a price change suggestion to the user.

  ## 6. Priority & Estimated Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
