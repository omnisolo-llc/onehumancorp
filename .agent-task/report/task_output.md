issue_title: "Universal Autonomous AI Business Advisory Engine"
issue_description: |
  # [Architecture] Universal Autonomous AI Business Advisory Engine

  ## Problem Statement

  Small business owners—whether it's Maya baking custom cakes, Carlos managing handyman jobs, or Priya running a local boutique—are domain experts, but they are rarely data analysts or financial strategists. They operate on intuition and lack the time to sift through complex analytics dashboards to figure out how to grow their revenue. They frequently miss subtle signals: a trending product, an underpriced service, a seasonal slump, or a drop in customer retention.

  While competitors like Shopify, Wix, and Square provide comprehensive reporting and charts, they place the cognitive burden entirely on the user to interpret the data. OHC must differentiate by offering an invisible, proactive AI "Business Advisor" that analyzes daily metrics and delivers simple, plain-language insights and one-tap actionable recommendations directly to the owner's mobile device.

  ## Research Report

  We analyzed how leading SMB platforms handle business intelligence and analytics:

  ### Competitive Analysis

  | Platform | Analytics Approach | Strengths | Weaknesses (The OHC Opportunity) |
  |---|---|---|---|
  | **Shopify** | Shopify Analytics & Reports | Deep, customizable reports, live view. | Requires technical/analytical skills. The owner must find the insights themselves. |
  | **Wix** | Wix Analytics | Integrated with web traffic and basic sales. | Reactive dashboards. Lacks cross-department (finance + ops + marketing) synthesis. |
  | **Square** | Square Dashboard | Strong offline/POS data visualization. | Focused only on transactions. No proactive strategic advice. |
  | **OHC (Target)** | **Autonomous Advisory Agent** | **Proactive push notifications, 1-tap actions, plain-language summaries (e.g., "Increase cake price by $5").** | **Must ensure insights are accurate and not overwhelming.** |

  ### Persona Pain Points
  *   **Priya (Boutique):** "I have 200 items in my store. I don't know which ones are sitting dead on the shelves and costing me money until I do inventory twice a year."
  *   **Leo (Tutor):** "Some students haven't booked a lesson in a month. I forget to follow up with them because I'm busy teaching."
  *   **Maya (Baker):** "People keep asking for vegan cakes. I didn't realize it was my most requested item in DMs until a friend pointed it out."

  ### Key Architectural Findings
  To provide personalized, accurate advice, the system must securely aggregate data across multiple domains (Orders, CRM, Inventory, Website traffic). This requires a highly scalable, asynchronous background processing architecture using a distributed job queue. Computing LLM-driven insights dynamically on every page load would be too slow and expensive. Therefore, we must use an asynchronous "Nightly Briefing" model, caching the results as static feed items.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ DAILY_METRIC_SNAPSHOT : generates
      TENANT ||--o{ ADVISORY_INSIGHT : receives
      DAILY_METRIC_SNAPSHOT }|--|| ADVISORY_JOB_QUEUE : triggers
      ADVISORY_JOB_QUEUE ||--o{ AI_ADVISOR_AGENT : consumed_by
      AI_ADVISOR_AGENT ||--o{ ADVISORY_INSIGHT : creates
      ADVISORY_INSIGHT ||--o{ ACTIONABLE_SUGGESTION : contains
  ```

  ```mermaid
  sequenceDiagram
      participant Cron as K8s CronJob
      participant Queue as PG Job Queue (SKIP LOCKED)
      participant Worker as AI Advisor Worker
      participant LLM as Gemini / GPT-4o
      participant App as Mobile App (Tauri)

      Cron->>Queue: Enqueue nightly advisory tasks for active Tenants
      Worker->>Queue: Dequeue task (Lock record)
      Worker->>Worker: Aggregate daily metrics, CRM events, Inventory levels
      Worker->>LLM: Prompt: "Analyze this SMB data. Provide 2 plain-language insights and 1 action."
      LLM-->>Worker: JSON Response (Insight + Action payload)
      Worker->>Queue: Persist ADVISORY_INSIGHT and mark job complete
      App->>Worker: Fetch unread insights on morning app launch
      Worker-->>App: Display "The Briefing" Glassmorphism card
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  **Screen 1: The Home Dashboard (Morning Briefing)**
  - **Top:** A translucent glass card (backdrop-filter blur) titled "Your Daily Briefing".
  - **Content:** "Good morning Maya! ☀️ Yesterday was your busiest Tuesday this month. You had 8 orders. We noticed a 20% increase in requests for Vegan cakes."
  - **Call to Action (CTA):** A glowing button: "Review 1 Recommendation".

  **Screen 2: The Insight Card (Actionable Advice)**
  - **Header:** "Pricing Opportunity"
  - **Body:** "Your Vegan Chocolate Cake is selling out within hours of restocking. Similar bakeries in your area charge $55. Your current price is $45."
  - **Action Buttons:**
    - [Update Price to $55] (Primary, solid color)
    - [Dismiss] (Secondary, ghost button)
  - **UX Flow:** Tapping the primary button executes the change instantly via a backend mutation, showing a satisfying micro-animation (confetti or checkmark).

  ### AI Agent Integration Points
  - **Department: Business Advisory ("The Advisor"):** Orchestrates the overall logic. It reads aggregated data from "The Accountant" (revenue), "The Manager" (inventory), and "The Ambassador" (customer DMs).
  - **Prompt Architecture:** The system prompt for The Advisor enforces strict constraints: "You are speaking to a non-technical small business owner. Use a friendly, encouraging tone. Never use terms like 'conversion rate', 'bounce rate', or 'EBITDA'. Provide exactly one highly actionable suggestion."

  ### Key Design Decisions
  1. **Asynchronous Generation:** Insights are generated asynchronously via the PostgreSQL `SKIP LOCKED` job queue to ensure the mobile app remains highly responsive.
  2. **Deterministic Actions:** The LLM does not execute actions directly. It generates a structured JSON payload defining the *suggested* action (e.g., `{"action": "UPDATE_PRICE", "target_id": "item_123", "value": 55}`). The human owner must click the button to approve the mutation.
  3. **Multi-Tenant Isolation:** The background worker authenticates via SPIFFE/SPIRE and executes queries strictly scoped to the `tenant_id` being processed.

  ## Implementation Prompt

  **Implementer Agent Task:**
  Implement the asynchronous AI Business Advisory Engine pipeline and the mobile-first "Daily Briefing" UI card.

  **Customer-User Journey (CUJ):**
  1. The system runs a nightly background job that aggregates sales, inventory, and customer message metrics for a given tenant.
  2. The AI Advisor agent processes these metrics and generates a plain-language summary and a structured, actionable recommendation (e.g., "Increase price of Product X").
  3. When the business owner opens the OHC app the next morning, they see a beautiful "Daily Briefing" translucent glass card on their dashboard.
  4. The owner taps the recommendation, views the reasoning, and taps "Accept" to apply the change (e.g., updating the product price) with a single click.

  **Acceptance Criteria:**
  - Create the `AdvisoryInsight` and `ActionableSuggestion` data models.
  - Implement the background worker logic using the PostgreSQL `SKIP LOCKED` pattern to safely dequeue and process tenant metrics.
  - Integrate the LLM provider interface to generate insights based on a structured system prompt.
  - Build the 375px mobile UI for the Home Dashboard "Briefing" card and the "Insight Details" screen using the OHC premium design tokens (Glassmorphism, Outfit font).
  - Ensure that accepting a recommendation securely executes the corresponding state mutation (e.g., updating a product's price in the database).
  - Unit test coverage MUST be 100% for the new backend services and background workers. Provide Playwright E2E tests for the frontend briefing flow.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
