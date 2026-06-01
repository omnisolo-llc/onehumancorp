issue_title: "[Research] Autonomous Plain-Language Business Advisory & Forecasting Engine"
issue_description: |
  # [Research] Autonomous Plain-Language Business Advisory & Forecasting Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Priya (boutique owner) do not have the time, skills, or desire to stare at complex charts, interpret raw analytics, or build Excel spreadsheets to understand their cash flow. Existing tools (Shopify Analytics, Quickbooks) provide raw numbers but require the user to derive meaning. Our users need an invisible "Advisor" that translates complex financial, operational, and marketing data into simple, actionable plain-language sentences (e.g., "You sold 30% more vegan cakes this week. You might run out of vegan flour by Thursday.")

  ## Research Report
  **Market Gap:**
  - **Shopify:** Provides dashboards and graphs, but no plain-language narrative.
  - **Wix/Squarespace:** Basic analytics, requires manual review.
  - **Quickbooks:** Complex financial reporting, requires accounting knowledge.
  **Opportunity:**
  Transform analytics from "dashboards" into an autonomous "Business Advisory" AI Agent that proactively reads data across all departments (Operations, Finance, Marketing) and generates a weekly or daily digest. This agent must understand the business context (e.g., local events, seasonal trends) and provide concrete next steps (e.g., "Tap here to reorder flour" or "Tap here to run a weekend discount on slow-moving inventory").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Sales & Finance Ledger] --> D[Data Aggregation Pipeline]
      B[Inventory & Ops Events] --> D
      C[Marketing & Web Traffic] --> D
      D --> E[Business Advisory AI Agent]
      E --> F[Contextual Memory - pgvector]
      E --> G[Insights Generation - Gemini Pro]
      G --> H[Plain Language Report]
      H --> I[Mobile Dashboard UI]
      H --> J[Push Notifications / Email]
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  - **Dashboard Widget (Glassmorphism):** A top-level "Advisor" card on the home screen. Translucent background with a friendly greeting.
  - **Example Text:** "Good morning Maya! Your vegan chocolate cakes are trending. You have enough ingredients for 5 more. [Order Supplies]"
  - **Interaction:** Swiping the card reveals the weekly plain-language breakdown (Sales, Upcoming Bookings, Inventory Warnings).
  - **No Charts:** Numbers are presented purely in text or simple progress bars.

  ### AI Agent Integration Points
  - **The Advisor (Business Advisory):** Orchestrates data from "The Accountant" (Finance) and "The Manager" (Operations).
  - **Trigger:** Runs via a CRON job (e.g., Sunday evening for weekly summaries, daily at 8am for daily briefs).
  - **Actions:** Can trigger downstream workflows (e.g., drafting an email campaign via "The Promoter" based on slow inventory).

  ### Key Design Decisions
  - **Narrative Over Numbers:** Data is useless without narrative. We prioritize LLM-generated summaries over raw D3.js charts.
  - **Proactive Push:** The owner shouldn't have to seek out the data; the app pushes actionable insights when they matter.
  - **Actionable Buttons:** Every insight must come with a 1-tap action button (e.g., "Create Promo", "Restock").

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the Business Advisory Agent backend service and its mobile-first UI representation.
  1. Create a `BusinessAdvisoryService` that aggregates daily metrics from the Finance and Operations tables.
  2. Use the AI interface to pass these metrics to the LLM with a system prompt instructing it to generate a short, friendly, plain-language business health summary.
  3. Build a Flutter UI widget (`AdvisorCard`) for the mobile dashboard (375px width optimized, using our glassmorphism design tokens) that displays this summary and an actionable button.
  4. Acceptance Criteria: A user logging in sees a freshly generated plain-language summary on their dashboard without needing to navigate to an analytics tab.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
