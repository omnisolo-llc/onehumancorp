issue_title: "Implement Multi-Tenant Agentic Work Triage & Task Orchestration Engine"
issue_description: |
  # Research Report: Agentic Work Triage & Task Orchestration Engine

  ## 1. Problem Statement
  Non-technical owner/operators (e.g., Carlos the Field Service Owner, Nora the Agency Principal) are overwhelmed by fragmented intake channels. DMs, emails, web forms, and missed calls create a disjointed list of tasks across multiple apps. Currently, OHC lacks a unified, multi-tenant work triage engine that automatically captures this demand and turns it into prioritized, actionable task cards. Without this, owners miss leads, double-book themselves, and lose momentum.

  ## 2. Research Report
  **Market Mapping & Competitor Discovery:**
  - **Traditional CRMs (HubSpot, Salesforce):** Powerful but heavily reliant on manual data entry and complex pipeline configurations. Unusable for Carlos on a 375px mobile screen.
  - **Service Titans (Jobber, Housecall Pro):** Good for field services but vertical-specific; they don't serve Priya or Leo well, and lack autonomous AI drafting for responses.
  - **Unified Inboxes (Front, Intercom):** Great for messages, but they don't natively convert a message into a scheduled booking, quote, or dispatch task without Zapier/third-party integration.

  **The Gap:** Owners don't want another unified inbox; they want an assistant that reads the inbox, groups related items, and proposes the next action (e.g., "Draft quote for this lead" or "Schedule delivery for tomorrow").

  ## 3. Design Doc
  ### Architecture & System Design
  - **Central Event Bus (Redis / Kafka):** Ingests webhooks from all intake channels (Instagram, WhatsApp, Web Forms, Emails).
  - **Task Orchestrator (PostgreSQL / Go Worker):** Multi-tenant task table (`tenant_id`, `resource_id`, `status`, `priority_score`). Uses PostgreSQL `SKIP LOCKED` for high-throughput, conflict-free job processing.
  - **Triage AI Agent (Gemini Pro / Fallback OpenAI):** Listens to new raw intake events, extracts intent, and creates/updates a Task record. It generates a brief "Why this matters" summary and a "Proposed Next Action" payload.
  - **Distributed Locks (Redis Redlock):** Ensures that when multiple intake events for the same customer arrive concurrently, they are triaged sequentially, preventing duplicate task creation. Lock pattern: `ohc:lock:{tenant_id}:triage:{customer_id}`.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant C as Customer (IG/Web)
      participant IB as Ingestion Bus (Redis)
      participant TA as Triage Agent (AI)
      participant DB as PostgreSQL (Tasks)
      participant UI as Mobile App (Owner)

      C->>IB: New Inquiry (DM/Form)
      IB->>TA: Trigger Intent Extraction
      TA->>DB: Check past context & insert Task
      TA->>DB: Generate "Proposed Action"
      DB-->>UI: Real-time update (SSE/WebSocket)
      UI->>UI: Render Actionable Task Card
  ```

  ### Mobile UX Flow (375px Viewport)
  1. **The Feed:** The user opens the app to the Home screen (the "Command Center").
  2. **Action Cards:** New intakes appear as stacked cards with translucent glass styling (`rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px)`).
  3. **Card Content:**
     - **Context:** "3 new cake inquiries overnight."
     - **AI Suggestion:** "Drafted 3 replies and calculated deposit links."
  4. **Interactions:** A large `44x44px` primary button at the bottom of the card says "Review & Approve". Swiping right approves instantly; swiping left dismisses/delegates.

  ### AI Agent Integration Points
  - **System Prompt:** Instructs the Triage Agent to synthesize urgency based on temporal phrases ("need this tomorrow" = High Priority) and dollar value.
  - **Handoff:** If the intent is "Quote Request", the Triage Agent creates the task and triggers the Sales Agent in the background to prepare the quote draft before the owner even opens the app.

  ## 4. Implementation Prompt
  **Feature Name:** OHC Work Triage Engine & Agent Feed

  **User-Facing Outcome:** When Maya wakes up, she opens OHC on her iPhone. Instead of seeing 15 unread messages, she sees a single card: "You have 3 new custom cake requests. I've drafted replies and checked your calendar. [Review Actions]".

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Generate synthetic intake events (via an API endpoint or script) simulating a new DM and a web form submission.
  2. The Go/Rust backend must successfully enqueue these events, acquire a Redis Redlock, and process them using the AI Job Queue (PostgreSQL `SKIP LOCKED`).
  3. The Triage Agent must generate a unified `Task` record in the database.
  4. The mobile/web UI (375px layout) must display the new task as an actionable card with a "Review" button.
  5. Provide 100% unit test coverage for the task ingestion and triage routing logic. E2E tests (Playwright) must simulate the owner clicking "Review" on the generated card.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []