issue_title: "Implement the Real-Time Operational Agent Feed"
issue_description: |
  # Research Report: Real-Time Operational Agent Feed & Triage

  ## Executive Summary
  Based on thorough research of the OHC codebase and documentation (`docs/business/market_research/agent_feed_deep_dive.md`), the core "Unified Agent Feed" architecture is conceptualized but lacks a robust, real-time backend pipeline and frontend streaming integration to make it fully operational.

  The Agent Feed is the "central nervous system" for business owners (e.g., Maya, Carlos, Priya) using OHC. It transitions them from passively reading static dashboards to proactively approving AI-drafted actions (e.g., "Approve Reply", "Confirm Booking"). Without a real-time, low-latency event stream and solid data model for these "Action Cards," the core value proposition of "Invisible AI Automation" fails to materialize.

  ## 1. Problem Statement
  Currently, the Agent Feed concepts exist primarily as UX mocks or E2E test targets (`src/ui/next/src/app/dashboard/UnifiedAgentFeed.tsx`, `src/e2e/dashboard_unified_feed.spec.ts`). There is no end-to-end, real-time data pipeline connecting backend agent draft generation to the mobile-first React frontend.

  Small business owners need an inbox/feed that actively pushes high-priority work (Work Triage) and AI suggestions (Customer Relationship responses, Operations tasks) to their device immediately, just like a human assistant would text them.

  ## 2. Research Report
  - **Competitor Analysis**: Tools like Shopify Inbox or Zendesk aggregate messages but don't autonomously draft multi-step actions (e.g., checking inventory + drafting reply). They also use polling or basic websockets that often lose state on mobile disconnects.
  - **Codebase Findings**: We have `ScoutAgent` for tool processing and various sub-agents in `src/agents/builtin/`. We have an E2E spec for `unified_agent_feed.spec.ts`.
  - **The Gap**: We need the `ActionCard` data model in the database, an API to fetch/stream them, and the frontend components to render and interact with them (Approve/Reject/Edit) seamlessly on a 375px viewport.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Agent Departments (Sales, Ops, etc.)] -->|Generate Draft/Task| B(Agent Feed API / PubSub)
      B --> C[(PostgreSQL: Action Cards Table)]
      B --> D[Redis: Real-time PubSub]
      C --> E[Frontend Feed Component]
      D --> E
      E -->|User Taps 'Approve'| F(Agent Feed Approval API)
      F --> A
  ```

  ### Data Model (Action Card)
  - `id`: UUID
  - `tenant_id`: UUID (Multi-tenant isolation)
  - `agent_department`: Enum (Operations, CustomerSuccess, etc.)
  - `priority`: Enum (High, Medium, Low)
  - `title`: String (e.g., "Draft Reply to Maya")
  - `description`: Text (The context or drafted message)
  - `status`: Enum (Pending, Approved, Rejected, Dismissed)
  - `metadata`: JSONB (For specific action payloads, e.g., `booking_id`)

  ### Mobile UX Flow (375px First)
  1. **The Feed View**: A unified, vertically scrolling list of cards. High priority at the top.
  2. **The Card**: Clean, translucent glass styling. Shows the AI's reasoning briefly.
  3. **The Actions**: Large (min 44x44px) buttons for "Approve" (Primary, e.g., green/blue) and "Dismiss" (Secondary/Ghost).
  4. **Optimistic UI**: Tapping "Approve" immediately fades the card out while the backend processes the approval via API.

  ### AI Agent Integration
  When an agent (e.g., The Ambassador) finishes drafting a response, instead of executing it directly, it writes an `ActionCard` to the DB and publishes a Redis event. The owner's frontend receives the event, updates the feed, and awaits user confirmation.

  ## 4. Implementation Prompt
  **To the Implementer:**
  Your task is to build the end-to-end Operational Agent Feed.
  1. Define the PostgreSQL schema for the Action/Feed cards, ensuring strict `tenant_id` isolation.
  2. Create the backend REST/gRPC APIs to list, create, and update (approve/reject) these cards.
  3. Implement the frontend `UnifiedAgentFeed` React component in `src/ui/next/src/app/dashboard/` to fetch these cards and render them according to the Mobile UX Flow described above.
  4. Implement optimistic UI updates on the frontend for the Approve/Reject actions.
  5. Ensure 100% test coverage for the API and update E2E tests (e.g., `src/e2e/dashboard_unified_feed.spec.ts`) to use the real flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
