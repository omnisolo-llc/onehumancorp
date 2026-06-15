issue_title: "Agent Feed Deep Dive & Actionable UI Implementation"
issue_description: |
  # Research Report: Agent Feed Deep Dive & Actionable UI Implementation

  ## Problem Statement
  Small business owners often feel overwhelmed by reactive dashboards that require them to hunt for information, configure settings, and manually trigger actions. The current OHC dashboard provides data but lacks the "Invisible AI Automation" vision. Owners like Maya or Carlos need a proactive, mobile-first feed that tells them exactly what needs their attention today and allows them to approve AI-drafted actions with a single tap.

  ## Research Report
  - **Market Context**: Traditional platforms (Shopify, Wix) rely on static dashboards and notification centers. AI assistants (like Shopify Sidekick) are often chat-based, requiring the user to initiate the conversation and ask the right questions.
  - **The OHC Opportunity**: OHC's Agent Feed flips the paradigm. Instead of the owner asking the AI, the AI proactively pushes actionable items to the owner. This is the core of the "Assistant-First Shell" vision.
  - **Competitor Gaps**:
    - *Shopify*: Sidekick is powerful but reactive (chat interface). Pulse offers insights but lacks 1-tap execution of complex workflows.
    - *Lindy/11x*: Excellent autonomous execution but often disconnected from the core commerce/booking engine.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Pipeline (Webhooks, Orders, Timers)] --> B[Agent Task Queue]
      B --> C[AI Agent (Operations/Sales/CS)]
      C -->|Draft Action| D[Feed Item Database]
      D --> E[Mobile App (Agent Feed UI)]
      E -->|User Approves| F[Action Execution Engine]
      F --> G[External System/Database]
  ```

  ### Mobile UX Flow (375px)
  1. **The Command Center**: The main screen is a unified vertical feed of actionable cards.
  2. **Action Cards**: Each card clearly states the context and the proposed action.
     - *Example 1 (CS)*: "Maya asked about vegan cakes on Instagram. Draft: 'Yes, we have 3 left!' [Approve] [Edit]"
     - *Example 2 (Sales)*: "Inventory for 'Red Dress' is low. Draft a restock order for 50 units? [Approve] [Ignore]"
  3. **Interaction**: 1-tap "Approve" triggers the execution. "Edit" opens a native bottom sheet to adjust the AI's draft.
  4. **Visuals**: Translucent glass materials, strong hierarchy, Unifi-style modular cards.

  ### Key Design Decisions
  - **Proactive over Reactive**: The feed is the primary UI, not a hidden notification bell.
  - **Approval Gate**: Critical actions (sending messages, ordering stock, processing refunds) require explicit owner approval via the feed, maintaining control and building trust.

  ## Implementation Prompt
  **Feature Name**: Agent Feed UI & Backend Integration
  **Target Persona**: All OHC Personas (Maya, Carlos, Priya)
  **Outcome**: The owner opens OHC and sees a prioritized feed of AI-drafted actions (Action Cards). They can approve or dismiss these actions directly from the feed.

  **Next Actions**:
  1. **Data Model**: Design the `FeedItem` schema (title, context, proposed_action_payload, status) with strict multi-tenant isolation.
  2. **Backend API**: Implement endpoints to fetch the feed and a generic endpoint to "approve" a feed item, which routes the payload to the appropriate execution engine.
  3. **Frontend (Flutter/PWA)**: Build the mobile-first (375px) Agent Feed UI. Create the reusable "Action Card" component using the design system tokens.
  4. **Agent Integration (Mock/Initial)**: Create a background job that simulates an agent adding a drafted reply or suggested action to the feed to demonstrate the end-to-end flow.

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
