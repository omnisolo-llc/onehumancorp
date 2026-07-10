issue_title: "Implement Unified AI Work Triage Command Center for Owners"
issue_description: |
  ## Title
  Implement Unified AI Work Triage Command Center for Owners

  ## Problem Statement
  Small business owners (our core personas like Maya, Carlos, Priya, Leo, and Fatima) experience severe communication and operational fragmentation. They receive customer inquiries across a multitude of isolated platforms (Instagram DMs, Facebook Messenger, WhatsApp, Email, Web Forms) while juggling bookings, payments, and system alerts. Managing these disjointed channels from a mobile device while simultaneously running a physical business results in dropped leads, delayed responses, and lost revenue. We need an assistant-first shell that unifies messages, tasks, bookings, and alerts into a single, prioritized owner feed, guiding the owner from "unclear work" to "clear next action in minutes" without needing a technical manual.

  ## Research Report
  - **Findings**: The current OHC platform lacks a unified entry point that acts as a true work assistant. Competitor platforms (Shopify Sidekick, WeCom, DingTalk) handle isolated verticals but fail to weave cross-domain operations (e.g., matching an IG DM about a cake to a calendar availability and drafting a quote) into a single triage flow.
  - **Market Context**: Owners do not want dashboards; they want an assistant that tells them what matters right now. A unified triage feed dramatically reduces cognitive load and turns reactive management into proactive action.
  - **Current System**: Existing systems handle tasks in isolated silos (e.g., `shared_tasks`). We need a unified feed that aggregates entities via row-level tenant isolation, using the AI Job Queue to pre-process and draft responses.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Inbound Channels: IG, WhatsApp, Web] --> B(Ingestion & Normalization)
      C[Internal Events: Bookings, Alerts] --> B
      B --> D{KAIROS Orchestrator}
      D --> E[Customer Success Agent - Intent & Triage]
      D --> F[Operations Agent - Context Check]
      E --> G[Unified Feed Store - PostgreSQL]
      F --> G
      G --> H[Mobile-First PWA/Flutter Shell]
      H --> I[Owner 1-Tap Approval]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Screen (The Feed)**: An Apple/Ubiquiti-style clean layout with translucent glass materials. A single vertical list of prioritized cards.
  2. **Triage Cards**: Each card displays a summary (e.g., "Maya, you have 3 unread cake inquiries"), a generated explanation of why it matters, and a primary action button (e.g., "Review Drafts").
  3. **Interaction**: Tapping a card expands it in place (or pushes a detailed view) without horizontal scrolling. Native mobile keyboards are triggered appropriately.
  4. **Approval Loop**: The owner reviews the AI-drafted reply or action, modifies it if needed, and hits "Approve".

  ### AI Agent Integration
  - **Trigger**: New messages or events trigger a background job using PostgreSQL `SKIP LOCKED` pattern.
  - **Processing**: The Customer Success Agent analyzes intent, the Operations Agent retrieves tenant-scoped context (e.g., inventory or calendar), and the drafted response/action is saved to the feed.
  - **Memory**: The interactions update the tenant-scoped memory via the AutoDream pipeline to inform future drafts.

  ## Implementation Prompt
  **Goal**: Build the unified "Work Triage" feed UI and the backend API to serve it.
  **CUJ**: Maya logs into the OHC mobile app (375px width). She sees a prioritized feed card stating she has an Instagram DM asking about a vegan cake. The card contains an AI-drafted reply and a suggested next action (send quote). Maya reviews the draft, taps "Approve", and the system executes the action.
  **Acceptance Criteria**:
  1. Create the `Unified Triage Feed` UI components using the OHC Premium Token library, strictly adhering to the 375px mobile-first constraint.
  2. Implement backend REST/gRPC endpoints to aggregate pending triage items from the database.
  3. Ensure zero mock data in the UI; data must flow from the real backend.
  4. Include a Playwright E2E test verifying the complete flow from login to feed interaction to action approval.
  5. UI must pass the "grandmother test" — intuitive, readable typography, and no technical jargon.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
