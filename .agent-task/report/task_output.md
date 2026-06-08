issue_title: "Implement Jarvis-Style Mobile-First Assistant WorkBuddy Parity"
issue_description: |
  # Research Report: Jarvis-Style Mobile-First Assistant WorkBuddy Parity

  ## 1. Problem Statement
  Small business owners (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun) need a central work assistant that acts as their unified command center. Currently, they lack a unified assistant interface (`/assistant`) that provides a WorkBuddy-like parity where they can issue natural language commands, view daily tasks, track AI-agent drafted responses, and review critical metrics—all from a mobile-first (375px) device. Existing solutions are either too complex (Shopify) or lack active AI task orchestration (Wix).

  ## 2. Research Report
  - **Market Context**: Traditional platforms treat the dashboard as a static list of metrics or complex settings. OHC's unique value proposition is the "Invisible Autonomous Agent". The user needs an interface to interact with these agents (The Promoter, The Ambassador, The Manager).
  - **The OHC Opportunity**: We can differentiate by making the `/assistant` route the primary entry point. It will feel like a personalized "Jarvis" for their business, offering a coordinated feed of work intake, customer relationships, operations, and revenue insights.
  - **Competitor Gaps**:
    - *Shopify Sidekick*: A chatbot bolted onto a complex admin interface.
    - *Wix*: Static dashboard, no active agent coordination.
    - *WorkBuddy/Copilot*: Excellent for enterprise, but OHC brings this level of orchestration to micro-SMBs via mobile.

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Mobile UI :375px /assistant] -->|Natural Language Command| B(Omnichannel Gateway)
      A -->|View Feed| C[Agent Task Feed]
      B --> D{Task Orchestrator / KAIROS}
      D --> E[The Ambassador Agent - Customer CS]
      D --> F[The Manager Agent - Ops]
      D --> G[The Promoter Agent - Sales]
      E --> C
      F --> C
      G --> C
      C --> A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Assistant Home Feed (Mobile):** The default view after login. A clean, translucent-glass dashboard.
  - **Top Section - Daily Briefing:** "Good morning, Maya. 3 custom cake requests need replies. 1 order is ready for pickup."
  - **Middle Section - Active Agent Tasks:** Cards showing drafts or actions requiring approval (e.g., "The Ambassador drafted a reply to Sarah's IG DM. [Review & Send]"). Touch targets must be >= 44x44px.
  - **Bottom Section - Command Input:** A persistent chat-like input bar with a microphone icon to issue new voice/text commands to the Jarvis assistant ("Schedule a marketing post for the new vegan chocolate cake").
  - **Interaction:** Swiping cards or tapping primary action buttons executes the agent's proposed action.

  ### AI Agent Integration Points
  - **Task Orchestrator:** The `/assistant` acts as the frontend to the KAIROS task queue and built-in agent harness.
  - **Memory:** The assistant uses tenant-scoped memory to personalize the briefing (e.g., knowing Maya sells cakes).

  ### Key Design Decisions
  - **Proactive over Reactive:** The assistant feed prioritizes AI-drafted actions needing approval over raw data dashboards.
  - **Mobile-First Constraints:** 100% functionality on 375px without horizontal scroll. Native keyboard integrations for the command input.
  - **Visual Design:** OHC Premium Token library (macOS-style translucent glass, Ubiquiti UniFi modular cards).

  ## 4. Implementation Prompt
  **Feature Name**: Jarvis-Style Mobile Assistant Hub
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya opens the OHC app to the `/assistant` route and immediately sees a synthesized feed of her day: drafted IG DM replies, pending cake deposits, and a suggestion to post about her new flavor. She can approve tasks with a single tap.

  **Next Actions for Engineering**:
  1. Create the base `/assistant` route and UI shell using Flutter/Next.js adhering to the 375px mobile-first and translucent glass design constraints.
  2. Implement the Unified Agent Feed component to consume tasks from the AI Job Queue (PostgreSQL `SKIP LOCKED`).
  3. Integrate the natural language command input to dispatch new tasks to the backend orchestrator.
  4. Ensure end-to-end Playwright tests cover the primary CUJ: Login -> View Assistant Briefing -> Approve Agent Draft -> Verify State.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
