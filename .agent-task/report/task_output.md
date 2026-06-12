issue_title: "Unified Agent Feed Architecture & Implementation Plan"
issue_description: |
  # Architecture Design: Unified Agent Feed (Mobile-First)

  ## Title
  Implement the Unified Agent Feed: The Central Nervous System for OHC

  ## Problem Statement
  Legacy business platforms (e.g., Shopify, Wix) rely on complex, static admin dashboards. These are overwhelmingly desktop-first and force users to hunt for actionable information across dozens of disconnected menus and plugins. For non-technical owners—like Maya the baker or Carlos the handyman—this "dashboard fatigue" creates a massive barrier. They need to know immediately what demands their attention, and they need a way to take action instantly from their 375px mobile screen without navigating away.

  ## Research Report & Gap Analysis
  - **The Mobile Management Gap**: While tools like Linktree succeed by optimizing for mobile simplicity, they lack full business capabilities. Shopify's companion app is good for tracking but poor for complex operations (e.g., setting up a promotion).
  - **The "Approval" Paradigm**: Based on our market research (`ohc_smb_mobile_first_design_research.md` and `agent_feed_deep_dive.md`), the solution is to replace complex forms with simple, AI-generated "Approval Cards".
  - **Current OHC State**: We have fragmented agent workflows (e.g., Cart Recovery, the Promoter). The `src/ui/next/src/app/dashboard/page.tsx` shows an early `UnifiedAgentFeed` component, but it needs to become the absolute core UX. The backend needs a robust event ingestion pipeline and an LLM context resolution layer to generate these action cards proactively.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Sources: Webhooks, DB triggers, Cron] --> B(Event Ingestion Pipeline: Redis Pub/Sub)
      B --> C{LLM Intent & Context Resolution}
      C -->|RAG: Query Tenant Data| D[Draft Generation]
      D --> E[Action Card Creation]
      E --> F((Unified Agent Feed UX))
      F --> G[User Approval / Edit / Dismiss]
      G --> H[Agent Execution via PostgreSQL SKIP LOCKED Queue]
  ```

  ### Mobile UX Flow (375px First)
  1.  **Launch**: User opens the OHC app.
  2.  **The Feed**: The primary view is a vertical stack of "Action Cards" using the OHC Premium Token library (translucent glass styling, clear spacing).
  3.  **Prioritization**: Urgent items (e.g., "3 New Orders to Fulfill") are at the top, followed by advisory/marketing proposals (e.g., "Drafted Instagram Post for new product").
  4.  **Interaction**: User taps a card. It expands inline (no page navigation). They review the AI's proposal and tap a large, minimum 44x44px "Approve" button.
  5.  **Execution**: The UI shows a brief, satisfying success animation, the card is dismissed from the feed, and the backend asynchronous worker handles the execution.

  ### AI Agent Integration Points
  -   **Customer Assistant**: Drafts replies to DMs and emails, presenting them in the feed for approval.
  -   **Operations Assistant**: Flags low inventory and proposes purchase orders.
  -   **Marketing/Promoter**: Suggests social posts and promotional campaigns based on milestones or slow sales periods.

  ## Implementation Prompt
  **Target Implementer**: Frontend/Backend Full Stack Engineer

  **Goal**: Solidify the `UnifiedAgentFeed` as the primary dashboard experience on mobile and ensure the backend pipeline can reliably deliver Action Cards to it.

  **Critical User Journey (CUJ)**:
  1.  User (Maya) opens the OHC dashboard on a mobile viewport (375px).
  2.  Maya sees a card from the "Customer Assistant" proposing a reply to a vegan cake inquiry.
  3.  Maya taps "Approve".
  4.  The feed updates instantly, the card disappears, and the message is queued for sending.

  **Acceptance Criteria**:
  -   The backend must expose a robust API endpoint for fetching aggregated Agent Action Cards for a specific `tenant_id`.
  -   The frontend `UnifiedAgentFeed` component must be completely responsive, passing the 375px horizontal scroll test.
  -   All interactive elements (Approve, Edit, Dismiss) must have a touch target of at least 44x44px.
  -   The feed must integrate cards from at least two different agents (e.g., Customer Assistant and Marketing/Promoter) to prove the unified architecture.
  -   The backend must use a robust queue (e.g., PostgreSQL `SKIP LOCKED`) to process approved actions asynchronously without blocking the UI.
  -   **ZERO** mock data in the UI; empty states must reflect the actual database state.

  ## Priority & Scope
  - **Priority**: P0 (Core to the OHC Vision)
  - **Estimated Scope**: Large (Involves backend event pipeline, LLM integration, and comprehensive frontend UX overhaul).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
