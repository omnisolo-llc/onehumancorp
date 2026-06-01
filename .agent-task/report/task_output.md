issue_title: "[architecture] Proactive Business Advisory & Analytics Engine (The Advisor)"
issue_description: |
  # Architecture Research for Proactive Business Advisory & Analytics Engine

  ## Problem Statement
  Small business owners like Priya (Boutique Owner) and Leo (Music Tutor) lack the time and expertise to dive into complex data analytics dashboards to figure out what's working and what isn't. Legacy platforms like Shopify and Wix provide static charts and graphs ("You had 500 visitors today"), but fail to provide *actionable, plain-language business advice* ("Your top seller was lemonade. Tuesday was your busiest day. Consider running a discount this Tuesday.").

  To fulfill the OHC Mission, we need a native **Business Advisory Engine ("The Advisor")** that acts as a personal consultant, proactively analyzing the multi-tenant PostgreSQL ledger and generating weekly insights, trend predictions, and plain-language action items delivered via the mobile UI.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify Analytics:** Comprehensive dashboards, but highly reactive. The user has to know what they are looking for (e.g., cohort analysis, bounce rates). It requires cognitive load.
  - **Wix Analytics:** Similar to Shopify, provides "Overview" reports, but lacks an AI layer that converts data into narrative "next steps".
  - **Square Dashboard:** Good at summarizing POS data, but disconnected from external marketing metrics or holistic business health.

  **Identified Gap:**
  OHC is uniquely positioned to solve this because we possess the unified ledger across all departments (Operations, Sales, Marketing, CS). However, there is currently no structural mechanism to autonomously query this data, process it through an LLM, and push proactive insights to the mobile app.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Cloud Backend
          Ledger[(Cloud Postgres CRM Ledger)] --> Cron[Business Advisory Cron Trigger]
          Cron --> VectorDB[(pgvector Embedding Store)]
          Cron --> LLM[Gemini Pro Advisory Pipeline]
          Ledger --> LLM
          LLM --> InsightModel[Insight Entity Store]
      end

      subgraph Mobile Device
          App[OHC Mobile App 375px] --> AdvisorUI[Glassmorphism Advisor Dashboard]
          InsightModel -- Sync via API Gateway --> AdvisorUI
          AdvisorUI --> NativePush[Push Notification]
      end

      subgraph Actions
          AdvisorUI --> MarketingAgent[Marketing: "Draft Promo Email"]
          AdvisorUI --> InventoryAgent[Ops: "Reorder Red Dress"]
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Push Notification:** Priya receives a push notification: "Your weekly boutique health report is ready! 📈"
  2. **The Advisor Dashboard:** She opens the app to the "Advisor" tab. She is presented with a clean, Glassmorphism-styled feed of insight cards. No complex line graphs.
  3. **Insight Card:** A card reads, "Your 'Summer Floral Dress' is selling 40% faster than last week. You have 3 left in stock."
  4. **One-Tap Action:** Below the text is an actionable, gradient button: "[Restock from Supplier]" or "[Draft Instagram Post to sell the last 3]". Tapping it seamlessly hands the context off to the Operations or Marketing agent.

  ### AI Agent Integration Points
  - **The Advisor (Business Advisory Agent):** Operates on a scheduled queue, analyzing temporal slices of tenant data. Emits `Insight` events.
  - **Marketing & Advertising Agent:** Consumes `Insight` events (e.g., "slow Tuesday sales") to autonomously propose marketing campaigns to the user.
  - **Operations Agent:** Consumes `Insight` events to warn about inventory depletion or suggest pricing adjustments.

  ### Key Design Decisions
  - **Asynchronous Processing:** Advisory generation is heavy. It runs on a decoupled AI Job Queue (PostgreSQL `SKIP LOCKED`) to avoid impacting core transactional APIs.
  - **Zero Jargon:** All LLM prompts for The Advisor are strictly configured to output simple, conversational English (or Arabic/Spanish, localized). No mention of "conversion rates" or "funnels".
  - **Multi-Tenant Safety:** The background worker authenticates via SPIFFE and asserts row-level security (`tenant_id`) before summarizing ledger data.

  ### Top 5 Things That Do Not Make Sense in Repository (For Future Implementation Optimization):
  1. **Go/Rust Architecture Confusion:** Several research docs (`[architecture]_agent-harness-analysis.md`) instruct implementation in Go under `src/server/harness/`, but the backend architecture is explicitly Rust (e.g., `src/server/harness/BUILD.bazel` is a `rust_library`).
  2. **Legacy Next.js Prototype:** The `src/ui/next` directory is deprecated and marked for removal but still exists alongside the canonical `src/ui/tauri` app.
  3. **Slint Remnants:** Slint UI was reportedly removed, but older architecture docs (`[backend]_agent_harness_architecture.md`) still reference Slint implementations (`ohc_harness/bash_security.rs (or equivalent Slint implementation)`).
  4. **Multiple Package Managers:** `package.json` and `pnpm-workspace.yaml` are present at the root, which can cause confusion alongside the primary Bazel build system if not strictly isolated.
  5. **Missing Multi-tenant Sync Tests:** The hybrid sync logic lacks comprehensive E2E playwright coverage that tests offline-to-online transitions under multi-tenant isolation.

  ## Implementation Prompt
  Implement the Proactive Business Advisory & Analytics Engine backend infrastructure.
  - **User-Facing Outcome:** The mobile application displays a weekly feed of actionable, plain-language business insights generated by an AI analyzing the user's recent transactions.
  - **CUJ:** A background job runs at 8 AM on Monday for a specific tenant. It queries the `tenant_id`'s sales ledger for the past 7 days, formats the data, and prompts the LLM to generate 3 actionable insights. These insights are saved to the database. The user opens the app, sees the insights, and clicks one to trigger a marketing action.
  - **Acceptance Criteria:**
    - Create the Postgres schema (`insights` table with `tenant_id` enforcement).
    - Implement the asynchronous AI Job worker in Rust that securely aggregates tenant data and interfaces with the Gemini LLM.
    - Expose a secure gRPC/REST endpoint for the mobile app to fetch insights.
    - Include unit tests (100% coverage) for the worker and API endpoint.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
