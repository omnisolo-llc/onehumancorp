issue_title: "Implement Unified Agent Feed (Mobile MVP)"
issue_description: |
  ## Problem Statement
  Legacy commerce platforms require small business owners to be part-time web developers, marketers, and IT administrators, forcing them to use complex desktop dashboards to run their businesses. Owners (like Maya, our baker selling via IG DMs, or Fatima, our food cart operator) need an immediate, mobile-first view of what needs attention right now without sifting through graphs and complex navigation. They need an assistant-first paradigm—a Unified Agent Feed—that brings actionable agent proposals and urgent operational items directly to a 375px mobile screen.

  ## Research Report
  Based on the OHC Global SMB Market Research Report and Mobile-First Design Research:
  - **The Gap**: Traditional dashboards (Shopify, Wix) fail on mobile because they cram desktop paradigms (charts, complex settings) into small screens. Link-in-bio tools succeed through simplicity but lack robust business operations.
  - **The Solution**: An approval-based interface. Instead of manual setup, invisible AI agents (Operations, Customer Service, Marketing) generate actionable drafts and insights.
  - **Competitive Differentiation**: Unlike Shopify Sidekick (a generic chatbot), OHC's Agent Feed pushes proactive "Action Cards" directly to the user. For instance, the system detects missing inventory or drafts a response to a customer DM, and simply asks the user to "Approve" or "Edit."

  ## Design Doc
  ### High-Level Architecture
  ```mermaid
  erDiagram
      EVENT_BUS ||--o{ AGENT_WORKER : "consumes"
      AGENT_WORKER ||--o{ INTENT_CLASSIFIER : "uses"
      INTENT_CLASSIFIER ||--o{ ACTION_CARD : "generates"
      ACTION_CARD ||--|| USER_FEED : "displayed in"
      USER_FEED {
          string tenant_id
          string user_id
          json active_cards
      }
      ACTION_CARD {
          string card_id
          string agent_type
          string status
          string draft_content
          string required_action
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Home/Feed Screen**: Upon app open, the user sees a vertically scrolling list of Action Cards. No complex hamburger menus or dense charts.
  2. **Action Card Anatomy**:
     - **Context Header**: e.g., "Operations Agent" or "Customer Service".
     - **Summary Statement**: e.g., "3 new custom cake inquiries waiting."
     - **Primary Action (Touch Target > 44px)**: e.g., "Review Drafts" or "Approve & Send".
     - **Secondary Action**: "Dismiss" or "Edit".
  3. **Interaction**: Tapping the primary action opens a modal/bottom-sheet with the AI-generated draft (e.g., an email reply or an Instagram DM). Tapping "Approve" executes the action via the corresponding Agent Worker and removes the card from the feed.

  ### AI Agent Integration Points
  - **Event Ingestion**: Stripe webhooks, Instagram DMs, or scheduled Cron jobs trigger events on the central message bus.
  - **Contextual Generation (RAG)**: The worker queries tenant-scoped memory (e.g., previous customer interactions, inventory levels) to generate the draft.
  - **Agent Handoff**: Once approved, the card dispatches a command back to the relevant agent (e.g., the Marketing Agent schedules an email, or the Operations Agent updates stock levels).

  ### Key Design Decisions
  - **Approval-First**: The UI focuses on reviewing AI drafts rather than creating from scratch.
  - **Mobile Constraints**: Strict adherence to 375px widths. Large, clear typography (OHC Premium Tokens) with translucent glass styling to feel premium yet simple.
  - **Tenant Isolation**: All feeds and generated drafts are strictly scoped to the `tenant_id`.

  ## Implementation Prompt
  **Objective**: Build the Unified Agent Feed (Mobile MVP) for the Flutter PWA and backend API. The feed should replace the traditional dashboard with a list of actionable cards.

  **User-Facing Outcome**: The user opens the app and sees a feed of prioritized agent suggestions (e.g., "Drafted reply to Carlos", "Inventory running low on Vegan Cakes"). They can tap a single big button to approve and execute the suggested action.

  **CUJ**:
  1. Log into the OHC app (simulated 375px viewport).
  2. The home screen renders the Unified Agent Feed.
  3. The feed displays a sample Action Card from the Customer Service Agent.
  4. The user taps "Approve" on the card.
  5. The UI shows a success state, the card is dismissed, and the underlying action is marked as executed.

  **Acceptance Criteria**:
  - Flutter frontend displays a vertical scrollable feed.
  - Backend provides an API to fetch pending Action Cards by `tenant_id`.
  - Cards support "Approve" and "Dismiss" mutations.
  - UI uses glassmorphism and clear Apple/Ubiquiti-style hierarchy.
  - Touch targets are strictly >= 44x44px.
  - Zero mock data in the final backend integration; cards must be persisted in PostgreSQL.

  ## Priority
  P0 (critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
