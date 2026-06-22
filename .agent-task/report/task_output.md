issue_title: "Agentic Dynamic Yield Management & Pricing Optimization"
issue_description: |
  # Research Report: Agentic Dynamic Yield Management & Pricing Optimization

  ## 1. Problem Statement
  Small business owners such as Fatima (Food Cart Operator) and Leo (Music Tutor) struggle with capacity and yield management. Fatima often either sells out of items too quickly during peak hours or has to throw away unsold food at the end of the day. Leo has unfilled slots during weekday mornings while his evening slots are constantly overbooked. Traditional platforms (Shopify, Wix, Calendly) only support static pricing unless the user manually creates complex discount codes or uses expensive 3rd-party yield management apps built for hotels/airlines, not micro-SMBs.

  ## 2. Research Report
  - **Market Context**: Existing platforms lack native, intelligent dynamic pricing. Shopify users rely on apps like "Bold Custom Pricing" or "Prisync," which cost $30-$100/mo and require technical setup (rules engines). Wix and Squarespace have no native yield management.
  - **The OHC Opportunity**: By leveraging the Sales/Operations Agents to continuously analyze inventory levels, booking density, and time-to-expiration, OHC can automatically suggest or apply subtle pricing adjustments (e.g., a 10% discount on slow Tuesday mornings, or a slight premium during a rush) to maximize revenue and minimize waste, without requiring the owner to be a revenue manager.
  - **Competitor Gaps**:
    - *Shopify*: Static pricing by default. Dynamic pricing requires heavy apps and manual rules.
    - *Wix/Squarespace*: Static pricing only.
    - *Calendly*: Time slots cost the same regardless of demand.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Inventory/Calendar State] -->|Event Stream| B(Yield Analysis Engine)
      C[Sales/Transactions] -->|Event Stream| B
      B --> D{The Operations Agent}
      D -->|Identifies Optimization| E[Draft Pricing Update]
      E --> F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|1-Tap Approve| H[Central Ledger / Stripe Billing]
      H --> I[Storefront/Booking UI]
  ```

  ### Mobile UX Flow (375px First)
  1. **Owner Feed View**: The app feed surfaces a card from the Operations Agent: "Traffic is high but 30% of your Halal Chicken plates are left 1 hour before close. Apply a 15% flash discount to avoid waste?"
  2. **Interaction**: The card displays the current price, suggested price, and projected revenue impact.
  3. **Action**: The owner taps a prominent "Approve" button.
  4. **Customer View**: The online storefront immediately updates with a visually highlighted "Flash Sale" or "Off-Peak" price tag.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager)**: Triggers when inventory decay (time to close) or booking vacancy exceeds a specific threshold. It analyzes historical tenant data to find the optimal discount rate that clears inventory without cannibalizing regular sales.
  - **Customer Success Agent (The Ambassador)**: If the owner approves, this agent can optionally push a quick SMS/Push notification to nearby or past customers: "Flash sale on Halal Chicken plates for the next hour!"

  ### Key Design Decisions
  - **Approval-Driven, Not fully Autonomous**: Prices are sensitive. The system will NEVER change a price without a 1-tap approval from the owner unless they explicitly enable "Auto-Pilot" mode.
  - **Unified Event Stream**: Relies on a unified stream of inventory, time, and transaction data to make accurate predictions.

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Dynamic Yield Management
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima receives a proactive mobile notification 1 hour before closing if she has excess inventory, suggesting a 1-tap flash sale to clear stock and boost daily revenue.

  **Next Actions**:
  1. Design the `YieldAnalysis` worker that periodically checks inventory levels/booking slots against time constraints (e.g., approaching store closing time).
  2. Integrate the worker with the LLM Operations Agent to generate the contextual flash-sale proposal.
  3. Develop the Mobile-First (375px) Action Card in the Agent Feed allowing the user to 1-tap approve the temporary price adjustment.
  4. Build the temporary price override logic in the Core Product/Service Data Model, ensuring it automatically reverts when the condition (e.g., store closes, or stock runs out) is met.

  **Acceptance Criteria**:
  - The worker correctly identifies excess inventory approaching expiration/closing.
  - The Action Card renders flawlessly on a 375px viewport with a minimum 44x44px "Approve" button.
  - E2E Playwright tests verify that an approved price drop reflects on the test storefront immediately.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []