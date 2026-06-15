issue_title: "[Research] OHC Autonomous Work Feed - The Agentic Command Center"
issue_description: |
  # Research Report: OHC Autonomous Work Feed - The Agentic Command Center

  ## Problem Statement
  Small business owners and operators (Maya, Carlos, Priya, Leo, Fatima) suffer from "dashboard fatigue." Traditional SaaS platforms force them to hunt for information across fragmented tools (inbox, calendar, order list, analytics). They don't need another dashboard with charts; they need a trusted assistant that tells them exactly what needs attention *right now* and what to do next. Currently, OHC lacks a centralized, mobile-first feed that aggregates cross-domain events and presents them as actionable AI-drafted recommendations.

  ## Research Report
  - **Market Context**: Legacy platforms (Shopify, Wix) rely on static dashboards. Even new AI tools often limit AI to a chat interface (e.g., Shopify Sidekick) where the owner must proactively ask questions.
  - **The OHC Opportunity**: True differentiation lies in **Invisible AI Automation** through an *Autonomous Work Feed*. This feed should invert the relationship: the system proactively brings prioritized, context-aware work to the owner.
  - **Competitor Gaps**:
    - *Shopify*: Has an activity feed, but it's mostly a passive audit log.
    - *Wix*: Static dashboard widgets.
    - *Lindy.ai / 11x.ai*: Strong AI execution, but often lack the deep, native integration with the business's core commerce/operations data.

  ## Design Doc

  ### Architecture Diagram (Concept)
  ```mermaid
  graph TD
    A[Event Sources: Webhooks, DB Triggers, CRON] --> B(Event Bus / Job Queue)
    B --> C{Work Triage Agent}
    C -->|Classifies & Prioritizes| D[Domain Agents: Ops, Sales, CS]
    D -->|Query RAG/State| E[(Tenant DB / Memory)]
    D -->|Drafts Action/Response| F[Action Card Generator]
    F --> G(Owner's Work Feed UI)
    G -->|Approve/Edit/Discard| H[Execution Engine]
  ```

  ### Data Model (High Level)
  - `WorkItem`: Represents a single actionable item in the feed.
    - `tenant_id`
    - `type` (e.g., `message_draft`, `booking_request`, `low_inventory`, `daily_summary`)
    - `status` (pending, approved, discarded, executed)
    - `priority` (P0 urgent, P1 normal, P2 low)
    - `context_data` (JSON payload with relevant info)
    - `proposed_action` (What the AI suggests doing)

  ### Mobile UX Flow (375px First)
  1. **The "Morning Briefing"**: Upon opening the app, the first screen is the Work Feed. It's clean, using OHC Premium Glass tokens.
  2. **Action Cards**: Each item is a card. E.g., "Maya, you have 3 new Instagram inquiries about vegan cakes. I've drafted replies."
  3. **One-Tap Actions**: The card displays the drafted reply with prominent buttons: [Approve & Send] [Edit] [Dismiss].
  4. **Swiping**: Users can swipe cards away to discard or delegate back to the agent.
  5. **Empty State**: When all items are cleared, a satisfying "Inbox Zero" style message: "All caught up! The bakery is running smoothly."

  ### AI Agent Integration
  - **Work Triage Agent**: The orchestrator. It ingests raw events (new order, new message, missed call) and decides which specialized agent to route it to.
  - **Domain Agents (Operations, Customer Success, Finance)**: These agents receive the triage event, query the database/memory for context, and generate the specific `WorkItem` with a `proposed_action`.

  ## Implementation Prompt
  **Feature Name**: Autonomous Work Feed Core & First Action Card
  **Target Persona**: Maya (Home Baker) & Carlos (Field Service)
  **Outcome**: The owner opens the OHC app and sees a unified feed of prioritized items. As a V1, implement the Feed UI and the pipeline for handling incoming messages as actionable "Drafted Reply" cards.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. **Backend Pipeline**: Create the `WorkItem` data model and the initial event pipeline (using the existing job queue).
  2. **Triage to Draft**: When a simulated customer message arrives, the system routes it to the Customer Success Agent, which generates a draft reply and creates a `WorkItem`.
  3. **Mobile UI**: Build the primary `WorkFeedView` in Flutter/Tauri. It must look exceptional on a 375px screen (no horizontal scrolling, clear touch targets).
  4. **Action**: The owner sees the "Drafted Reply" card, taps "Approve", and the system marks the `WorkItem` as executed (and simulates sending the message).
  5. **Zero Mock Data**: The UI must pull real `WorkItem` records from the backend. E2E tests must verify this flow end-to-end.

  **Priority**: P0 (This is the defining UI/UX paradigm of OHC)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
