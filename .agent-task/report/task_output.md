issue_title: "[Research] AI Unified Inbox & Smart Triage Feed"
issue_description: |
  # Research Report: AI Unified Inbox & Smart Triage Feed

  ## Problem Statement
  Owners across all segments (Maya the baker, Carlos the handyman, Nora the agency principal) suffer from "inbox fragmentation." They receive work inquiries, customer questions, and operational alerts across Instagram DMs, WhatsApp, SMS, email, and web forms. Manually checking 5 apps and deciding what matters is exhausting and leads to missed revenue. They need a unified feed that not only aggregates messages but *triages* them—drafting replies, suggesting actions (like sending a quote), and highlighting what needs immediate attention.

  ## Research Findings
  Competitive analysis of platforms like HubSpot, Intercom, and specialized SMB tools (e.g., Podium) shows that while unified inboxes exist, they lack true AI *triage* (understanding intent and drafting actions proactively). OHC must differentiate by building a feed that acts like an Executive Assistant, not just a message list. The existing triage implementation in OHC is rudimentary and needs to be fully realized according to the new architecture design.

  ## Proposed Solution & Design
  We will build a fully unified "Work Triage" feed, accessible from the command center dashboard, serving as the central clearinghouse for owner attention.

  ### Mobile UX Flow (375px)
  1. **Triage Feed:** A vertical stack of premium translucent "Triage Cards."
  2. **Card Content:** Each card clearly shows: Source (e.g., WhatsApp icon), Customer Context, the Message/Alert summary, and an AI-drafted response or suggested action.
  3. **Interaction:**
     - Primary button: e.g., "Approve & Send Draft", "Create Quote", "Confirm Booking".
     - Secondary button/swipe: "Dismiss".
  4. **Empty State:** A satisfying, beautiful "Inbox Zero" state indicating no pending actions.

  ### AI Agent Integration
  - **Triage AI Service:** Analyzes incoming webhooks/messages, determines intent (inquiry, complaint, booking request), extracts context, and drafts the initial reply or suggests the next workflow step.
  - **Storage:** Persisted in PostgreSQL (`triage_items` table or similar), with real-time updates via WebSocket/polling to the UI.

  ### Implementation Prompt for Engineer Agent
  **Target Persona:** Maya (Baker) & Carlos (Handyman).
  **Objective:** Implement the backend API and frontend UI for the AI Unified Inbox & Smart Triage Feed based on the architecture docs.
  **Requirements:**
  1.  **UI Implementation:** Build the responsive Triage Feed UI (starting at 375px) in the Dashboard/Command Center. Use premium styling (translucent glass, refined spacing).
  2.  **API/Backend Integration:** Ensure the UI connects to the actual backend endpoints (`/api/triage/pending`, `/api/ui/triage/create` etc. - verify exact names in existing code/tests) to fetch real, unmocked data.
  3.  **Interactions:** Implement working 'Approve' and 'Dismiss' actions that communicate with the backend.
  4.  **Testing:** Ensure all existing triage-related E2E Playwright tests (e.g., `unified_triage.spec.ts`) pass against the local stack. Write new tests if necessary to cover the complete UI flow. Do not use UI mocks for data.

  ## Scope & Priority
  **Priority:** P0 (Core to the "Command Center" value proposition).
  **Scope:** Large (Frontend UI, API integration, E2E validation).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
