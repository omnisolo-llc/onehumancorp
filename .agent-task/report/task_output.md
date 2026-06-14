issue_title: "Implement Tencent Workbuddy-style Assistant Workstation and Automations"
issue_description: |
  # Research Report: Tencent Workbuddy-style Assistant Workstation and Automations

  ## Problem Statement
  Small business owners and operators (Maya, Carlos, Priya, Leo, Fatima) struggle with disjointed, complex platforms that force them to learn tools rather than execute work. Existing platforms provide "Dashboards" where owners must hunt for information and manually perform tasks across different modules. We need to implement the vision defined in `2026-06-07-jarvis-workbuddy-parity-design.md`, creating an AI-driven, proactive "Assistant Workstation" (`/assistant`) that coordinates tools, generates artifacts, requests permissions, and executes scheduled automations, replacing the fragmented dashboard paradigm.

  ## Research Report
  - **Market Context**: Traditional platforms (Shopify, Wix) rely on user-driven configuration. AI-native point solutions (Durable) focus on initial generation but lack deep operational integration. OHC's unique value is "Invisible AI Automation" driven by an assistant.
  - **Tencent Workbuddy Parity**: The design doc specifies a dense workstation layout, durable task/artifact records, guarded permission profiles, remote control (Slack/Discord integration), and scheduled automations.
  - **The OHC Opportunity**: By building the `/assistant` shell as the primary app surface, we transition the product from a static set of tools to a dynamic, agentic workflow engine. The assistant coordinates the "Expert Center" agents, executes tasks, and presents results in real-time.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      UI[Frontend /assistant Route] --> |Task Requests| API[Backend Task API];
      API --> DB[(PostgreSQL Tasks, Artifacts, Messages)];
      API --> Planner[AI Planner Agent];
      Planner --> |Decomposes into| Steps[Execution Steps];
      Steps --> Permissions{Guarded Permission Policy};
      Permissions -- Approved --> Executor[AI Tool Executor];
      Permissions -- Blocked --> UI;
      Executor --> |Updates| DB;
      Executor --> |Generates| Artifacts[Artifact Pipeline];
      Artifacts --> DB;
      UI <-- |Streams Updates| API;

      Cron[Automation Scheduler] --> |Triggers| Planner;
      MessagingApp[Remote Control Slack/Discord] --> |Webhook| API;
  ```

  ### Architecture
  - **Route**: Introduce `/assistant` as the primary workspace (with `/agents` remaining as the Expert Center).
  - **Core Data Model Additions**:
      - `Workspace`, `AssistantTask`, `TaskMessage`, `TaskArtifact`, `TaskFileChange`, `ConnectorConfig`, `Automation`, `MemoryItem`.
  - **Execution Flow**: User creates a task -> Planner decomposes into steps -> Permission policy evaluates risk (Guarded Mode) -> Agent streams progress -> Artifacts/Changes registered -> Results panel updates.
  - **Artifact Pipeline**: First-class support for Markdown, HTML previews, CSV/XLSX, charts, PDFs, PPTX, and ZIP bundles.

  ### Data Model (ER Diagram)
  ```mermaid
  erDiagram
      WORKSPACE ||--o{ ASSISTANT_TASK : "owns"
      ASSISTANT_TASK ||--o{ TASK_MESSAGE : "contains"
      ASSISTANT_TASK ||--o{ TASK_ARTIFACT : "generates"
      ASSISTANT_TASK ||--o{ TASK_FILE_CHANGE : "proposes"
      WORKSPACE ||--o{ AUTOMATION : "configures"
      WORKSPACE ||--o{ MEMORY_ITEM : "stores"
  ```

  ### Mobile UX Flow (375px)
  - The dense workstation layout must gracefully collapse for mobile.
  - Left rail (task list) becomes a slide-out drawer or bottom sheet.
  - Center panel focuses on the conversation feed and immediate approval actions.
  - Bottom composer remains accessible above the keyboard.
  - Right panel (results/artifacts) is accessible via tabs or secondary screens.
  - "Action Cards" (1-tap approvals) are central to the mobile feed.

  ### AI Agent Integration Points
  - Planner agent decomposes tasks and selects tools/connectors/skills/experts.
  - Agent runtime executes steps and streams progress.
  - Permission evaluation dictates when to pause and request user approval (especially for file writes or external actions).
  - Automation scheduler triggers background agent tasks based on time or events.

  ## Implementation Prompt
  **Target Persona**: Maya (Home Baker) / Nora (Agency Principal)
  **Outcome**: Provide a unified `/assistant` interface where users can chat with their business, approve drafted actions (e.g., replying to DMs, generating proposals), view generated artifacts, and set up recurring automations, all within a guarded permission model.

  **Acceptance Criteria**:
  1. Build the primary `/assistant` route and shell (Workstation Layout), ensuring responsiveness down to 375px.
  2. Implement the durable backend data models for Tasks, Messages, and Artifacts.
  3. Implement the guarded permission model: read-only actions proceed, file writes/external actions pause for user approval.
  4. Develop the Artifact Pipeline to register and display generated files in a dedicated results panel.
  5. Implement Automations: create, schedule, and run recurring assistant tasks.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
