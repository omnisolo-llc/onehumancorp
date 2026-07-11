issue_title: "[Research] Architecture: Autonomous Dynamic Pricing & Yield Management Engine"
issue_description: |
  # [architecture] Autonomous Dynamic Pricing & Yield Management Engine

  ## Title
  Implement Autonomous Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners like Priya (Boutique Owner) and Leo (Music Tutor) struggle with optimal pricing. Priya has slow-moving inventory taking up shelf space, but manually auditing sales and applying discounts is tedious. Leo has unbooked time slots that expire, resulting in lost revenue. For non-technical owners, pricing is often a "set and forget" guess. They need an intelligent system that automatically adjusts prices or offers targeted promotions to clear out stagnant inventory or fill empty calendar slots, maximizing revenue without requiring manual spreadsheets or complex discount code setups.

  ## Research Report
  ### Market Context
  - **Shopify**: Requires third-party apps (like "Dynamic Pricing") which cost monthly fees and require complex rule configurations. Not suitable for non-technical users.
  - **Wix**: Basic manual discount codes and sale prices. No native AI-driven yield management.
  - **Squarespace**: Manual sales and discounts only.
  - **Airlines/Hotels**: Use sophisticated yield management, but this technology is entirely inaccessible to small businesses.

  ### OHC Opportunity
  OHC can democratize yield management. By leveraging the "Finance & Payments" and "Business Advisory" AI agents, OHC can monitor inventory velocity and booking density. When stagnation is detected, the AI proposes (or automatically applies) optimized discounts to specific customer segments.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Business Advisory Agent] -->|Analyzes| B(Inventory & Booking Data)
      B -->|Identifies| C{Stagnant Items / Empty Slots}
      C -->|Triggers| D[Finance & Payments Agent]
      D -->|Calculates Optimal Discount| E[Dynamic Pricing Engine]
      E -->|Updates Price/Promo| F[(Multi-Tenant DB)]
      F -->|Notifies| G[Customer Success Agent]
      G -->|Emails/SMS| H[Targeted Customers]
  ```

  ### UI Wireframes / Screen Flow (375px Mobile First)
  1. **Home Dashboard**: A Glassmorphism card appears: "✨ 3 items are moving slow. Run a flash sale to clear them out?" with a single "Preview Sale" button.
  2. **Preview Screen**:
     - Clean, translucent card showing the 3 items (e.g., Red Summer Dress).
     - "Suggested Discount: 20% off for 48 hours."
     - "Estimated Revenue: $450."
     - Two buttons: "Approve & Notify Customers" (Primary) and "Adjust Details" (Secondary).
  3. **Active Sale View**: A compact progress bar showing items sold during the dynamic promotion.

  ### Mobile UX Flow
  - Native touch targets (44x44px).
  - Haptic feedback when approving the sale.
  - Bottom sheet modal for adjusting sale parameters, using large numeric keypads for percentage inputs.

  ### AI Agent Integration Points
  - **Business Advisory**: Constantly runs in the background analyzing `velocity` (sales per week) vs `stock_level`.
  - **Finance & Payments**: Calculates margin impacts to ensure the business doesn't lose money on the discount.
  - **Marketing & Advertising**: Generates the social media post or push notification text ("Flash Sale on Summer Dresses!").

  ### Key Design Decisions
  - **Opt-in Autonomy**: The system suggests the pricing change first, requiring a 1-tap approval. As trust builds, users can enable "Auto-Pilot" for specific categories.
  - **No Complex Rules**: Hide all algorithmic complexity. The user only sees "Stagnant Item" -> "Suggested Fix" -> "Expected Revenue".

  ## Implementation Prompt
  **Objective**: Build the user-facing flow and background worker structure for the Dynamic Pricing & Yield Management Engine.

  **Critical User Journey (CUJ)**:
  1. Priya logs into the OHC mobile app.
  2. She sees an AI Advisory Card suggesting a 20% discount on "Summer Hats" because they haven't sold in 30 days.
  3. She taps "Approve & Run Sale".
  4. The system updates the storefront price, and the Marketing Agent drafts an email to her past customers.

  **Acceptance Criteria**:
  - Ensure all screens are fully responsive down to 375px.
  - Use OHC's Glassmorphism tokens for the suggestion cards.
  - Implement the "Approve" flow with optimistic UI updates.
  - Ensure the background pricing update adheres to tenant isolation boundaries.
  - No DB schema or specific APIs prescribed—design the necessary services to satisfy this CUJ.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
