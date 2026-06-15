issue_title: "[research] AI Agentic Inventory & Prep Forecasting Engine"
issue_description: |
  **Title**: AI Agentic Inventory & Prep Forecasting Engine

  **Problem Statement**:
  Micro-business owners dealing with physical goods, especially perishables (like Fatima the food cart operator or Maya the baker), suffer from the "prep guessing game". Over-prepping wastes expensive ingredients and cuts into tight margins; under-prepping results in sold-out items early in the day, missing out on crucial revenue. They do not have the time to analyze spreadsheets, weather forecasts, or local event calendars to determine exact daily prep amounts.

  **Research Report**:
  - **Market Context**: Square offers basic inventory tracking and low-stock alerts, but no predictive prep intelligence for the micro-operator. Shopify's inventory forecasting requires expensive third-party apps (e.g., Inventory Planner) that are built for large e-commerce warehouses, not a local food cart or bakery. Wix Restaurants has basic sold-out toggles but lacks predictive analytics.
  - **OHC Opportunity**: Integrate an invisible AI Operations Agent that synthesizes past sales data, local weather APIs, and day-of-week trends to generate a simple, actionable "Daily Prep List" pushed to the owner's 375px mobile feed each morning or evening before.

  **Design Doc**:
  - **Architecture Diagram**:
  ```mermaid
  graph TD
      A[Historical Sales DB] --> D(Operations AI Agent)
      B[Weather/Local Events API] --> D
      C[Current Inventory] --> D
      D -->|Synthesizes| E[Daily Prep Plan]
      E --> F[Owner Mobile Feed 375px]
      F -->|1-Tap Accept| G[Update Inventory/Order Supplies]
  ```
  - **Mobile UX Flow (375px)**:
    1. **Push Notification / Feed Card**: "Prep Plan for Tuesday: Expecting 15% more foot traffic due to sunny weather. Recommended prep: 50 Falafel wraps, 20 chicken plates."
    2. **Review Screen**: A clean, touch-friendly list (44x44px touch targets) showing items and quantities. The owner can adjust quantities with + / - buttons.
    3. **Approval**: Tapping "Approve Plan" creates internal tasks for staff or generates a supply order draft for any missing ingredients.
  - **AI Agent Integration Points**: The Operations Agent runs a nightly cron job utilizing PostgreSQL `SKIP LOCKED` job queues. It queries the tenant's Historical Ledger and external context to output a JSON-structured prediction, which the UI renders as a native card.

  **Implementation Prompt**:
  **Feature Name**: AI Agentic Inventory & Prep Forecasting Engine
  **Target Persona**: Fatima the food cart operator, Maya the baker.
  **Outcome**: Fatima wakes up, checks the OHC app, and sees an AI-generated prep list for the day based on historical sales and weather. She approves it, and her staff knows exactly what to prepare, minimizing waste and maximizing sales.

  **Next Actions**:
  1. Implement a new background worker job (Go/PostgreSQL) that generates daily forecast records per tenant using sales history.
  2. Create a 375px mobile UI card in Flutter for the "Daily Prep Plan" feed item with adjust/approve actions.
  3. Wire the Operations Agent to synthesize the data into the forecast model.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
