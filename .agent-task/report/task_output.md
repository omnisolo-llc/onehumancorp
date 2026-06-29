issue_title: "OHC Strategic Objective Engine: Proactive Cross-Agent Goal Execution"
issue_description: |
  ### Problem Statement
  Small business owners like Maya (the baker) or Carlos (the handyman) often have clear business objectives—such as "I need to clear my stagnant winter inventory by Friday" or "I want to fill my remaining booking slots for next Tuesday"—but they lack the operational bandwidth to coordinate individual agents (Marketing, Finance, Operations) to achieve these goals. Currently, OHC agents operate primarily in silos or react to discrete events (like a new DM). There is no "Chief of Staff" capability that translates high-level owner intent into a coordinated, multi-department execution plan.

  ### Research Report
  **Market Context & Competitive Analysis:**
  - **Shopify Sidekick / Wix AI:** These are primarily reactive chatbots. They help users find data or perform UI-builder tasks when asked. They do not proactively monitor business health and propose/execute cross-functional strategies.
  - **OHC Competitive Edge:** Our "Teammate Mesh" and "Unified Orchestrator" architecture already support multiple specialized agents. By introducing a **Strategist Agent**, OHC moves from a "tool suite" to an "autonomous management team."

  **Key Findings:**
  - Owners experience "Decision Fatigue." They don't want to manage three different AI agents; they want to set one objective and have the agents collaborate behind the scenes.
  - The existing `DailyBriefingWorker` provides data but not direction. The proposed engine will close the loop from **Insight → Strategy → Execution**.

  ### Design Doc
  **High-Level Architecture:**
  The `StrategistAgent` acts as a supervisor layer above the `DepartmentOrchestrator`. It consumes "Vitality Signals" from the database and "Episodic Memory" from the `AutoDream` pipeline to architect multi-agent plans.

  ```mermaid
  graph TD
      Owner[Owner] -->|Set Goal: 'Clear Stagnant Stock'| Strategist[Strategist Agent]
      Strategist -->|Analyze Vitality| DB[(PostgreSQL)]
      Strategist -->|Query Memory| RAG[AutoDream RAG]
      Strategist -->|Decompose Goal| Orchestrator[Unified Orchestrator]

      subgraph Multi-Agent Plan Execution
          Orchestrator -->|Task: Set 20% Discount| Finance[Accountant Agent]
          Orchestrator -->|Task: Draft Flash Sale Post| Marketing[Promoter Agent]
          Orchestrator -->|Task: Update Inventory Levels| Ops[Manager Agent]
      end

      Finance/Marketing/Ops -->|Generated Action Cards| Feed[Mobile Action Feed]
      Feed -->|1-Tap Approval| Owner
  ```

  **Data Model & Multi-Tenancy:**
  - `StrategicObjective`: Stores the high-level goal, target date, and current state. Strict RLS on `tenant_id`.
  - `MultiAgentPlan`: A JSONB DAG (Directed Acyclic Graph) of tasks assigned to specific departments.

  **Mobile-First UX Flow (375px):**
  - **Vitality Dashboard:** A premium glassmorphism screen showing plain-language health signals (e.g., "Cupcake sales are 30% lower than usual").
  - **Objective Input:** A radically simple "What is your focus this week?" text input.
  - **Plan Review:** A "Strategy Card" showing the coordinated steps (e.g., "Accountant suggests $5 price drop; Promoter drafted 2 Instagram posts").
  - **Progress Tracker:** A simple, non-technical progress bar showing the objective's lifecycle.

  ### Implementation Prompt
  **Feature Name:** OHC Strategic Objective Engine & Strategist Agent
  **Target Persona:** Maya the Baker
  **User-Facing Outcome:** Maya opens OHC, sees that her winter cookies aren't selling, and types "Sell out of winter cookies by Friday." The Strategist Agent coordinates with the Promoter to draft social posts and the Accountant to set a temporary discount. Maya taps "Approve All," and the agents execute the plan.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Implement the `StrategicObjective` and `PlanTask` entities in the PostgreSQL schema with strict tenant isolation.
  2. Develop the `StrategistAgent` as a new department implementation in `src/server/orchestration/departments/`.
  3. Wire the `StrategistAgent` to the `UnifiedOrchestrator` to enable multi-step task decomposition.
  4. Create the **"Business Vitality & Strategy"** views in the Tauri frontend, adhering to the macOS glassmorphism design system.
  5. Provide Playwright E2E tests: A user logs in, sets a business goal, reviews the multi-agent plan cards, and verifies that the corresponding records (Discounts/Posts) are created in the backend.

  ### Priority & Scope
  **Priority:** P1 (High)
  **Estimated Scope:** Large (Architectural Core)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, mobile-first]
assignees: []
