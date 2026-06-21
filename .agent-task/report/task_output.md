issue_title: "Implement Tencent Workbuddy-style Assistant App Layout and KAIROS Integration"
issue_description: |
  # Research Report & Design Doc: OHC Assistant Parity with Tencent WorkBuddy

  ## Problem Statement
  OneHumanCorp (OHC) is built to be an AI work assistant for owners and operators. However, the current UX is service-centric and not sufficiently assistant-centric. The overarching promise is that "anyone can launch and run a real small business from their phone or browser in under 10 minutes" using invisible AI agents, acting like Tencent Workbuddy. We need to implement a dedicated, mobile-first Assistant Workspace (`/assistant`) that unifies conversations, task management, document and artifact generation, and background agentic operations seamlessly.

  ## Research Report
  - **Market Landscape**: Tools like Shopify Sidekick and Wix Studio provide basic AI assistance but are limited to specific features.
  - **Tencent Workbuddy Benchmark**: Tencent WorkBuddy operates as a powerful desktop and mobile workstation operator that:
    - Accepts natural language tasks,
    - Plans and executes them using specialized teams or tools,
    - Operates on authorized files and generates actionable artifacts,
    - Preserves context, handles remote control (Slack/DMs), and remembers preferences.
  - **Gap Identification**: OHC already has a robust backend with the KAIROS orchestration engine, an Expert Center (`/agents`), and robust capabilities (booking, POS, unified inbox, etc.). However, it lacks the centralized assistant feed that connects these capabilities to an outcome-driven UX.

  ## Design Doc
  ### Architectural Roadmap
  The new feature will establish `/assistant` as the primary application surface, hiding advanced setups behind conversational paths or clear UI tokens.

  1. **Assistant Workspace (`/assistant`)**:
     - **UI Elements**: Task list, timeline conversation, rich composer with file/image inputs, and split views for artifacts/changes.
     - **Layout Standard**: Clean, Apple/Ubiquiti-style hierarchy with translucent materials, responsive down to 375px (no horizontal scrolling).
  2. **Data Model Updates**:
     - Consolidate agent feed events and task interactions into the necessary task data tables.
     - Track generated documents referencing the parent task.
  3. **KAIROS Integration**:
     - Connect the natural language composer to the Planner engine to parse intents into operational commands (e.g., booking, posting).
  4. **AI Department Coordination**:
     - The assistant dynamically invokes the Operations, CS, Sales, or Legal agents in the background depending on the identified intent.
     - Lock and memory systems coordinate concurrent operations to prevent state corruption.

  ```mermaid
  graph TD
      User[User / Owner] -->|Natural Language Prompt| Assistant[Assistant Workspace]
      Assistant --> Planner[KAIROS Planner]
      Planner --> O[Operations Agent]
      Planner --> C[Customer Success Agent]
      Planner --> S[Sales & Revenue Agent]
      O --> Artifact[Artifacts/Documents]
      C --> Artifact
      S --> Artifact
      Artifact --> Assistant
  ```

  ### Mobile UX Flow (375px first)
  - **Home**: A unified feed showing urgent tasks at the top, a conversational prompt at the bottom.
  - **Interaction**: Tap to review a generated quote or reply draft. The UI expands an action card without navigating away from the feed.
  - **Touch Targets**: Guaranteed 44x44px for action buttons.

  ## Implementation Prompt (For Implementer Agents)
  **Objective**: Build the core `/assistant` route in the Flutter frontend and connect it to the KAIROS backend to match the Tencent WorkBuddy workflow.
  **CUJ (Critical User Journey)**:
  1. The user navigates to `/assistant`.
  2. The user inputs a command: "Draft a response to Maya about the vegan cake order and attach a quote for $50."
  3. The Assistant responds in the feed, showing progress states ("Reviewing Maya's history...", "Drafting quote...").
  4. The Assistant returns a drafted response and an interactive Quote Artifact card.
  5. The user taps "Approve & Send" on the card.

  **Acceptance Criteria**:
  - The UI matches the OHC Premium Token library (Translucent Glass).
  - The layout is flawless at 375px wide.
  - The backend planner successfully maps the prompt to the quoting capability and generates an actionable artifact card.
  - Test coverage covers the rendering of the card and successful submission to the KAIROS task queue.
  - Must include Playwright E2E tests validating the full loop.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
