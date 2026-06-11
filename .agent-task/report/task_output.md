issue_title: "Implement Agentic Feed Architecture for Invisible Automation"
issue_description: |
  # Research Report: Agentic Feed Architecture

  ## Problem Statement
  Business owners (like Maya the Baker or Carlos the Handyman) currently have to hunt through different dashboards, menus, and inbox tabs to understand what action is required next. Existing platforms like Shopify or Wix are passive—they wait for the user to seek out information. OHC needs a proactive "Agentic Feed" that serves as the central nervous system of the platform, pushing critical updates, suggested actions, and AI-drafted responses directly to a mobile-friendly stream.

  ## Research Report
  - **The Gap**: Currently, OHC lacks a unified way to present actionable items across different domains (Inbox, Orders, Inventory, System Alerts). Users have to navigate to specific sections to see if anything needs their attention.
  - **Market Context**: Traditional SaaS uses static dashboards. Modern creator tools use simplified lists. An "Agentic Feed" combines the simplicity of a social feed with the power of autonomous AI agents. The AI doesn't just notify; it drafts the next action (e.g., "Drafted reply to Instagram DM: 'Yes, vegan cakes are available.' [Approve] [Edit]").
  - **Competitor Landscape**:
    - *Shopify*: Excellent operational dashboards, but relies on passive notifications or the Sidekick chat interface (which requires user initiation).
    - *Wix*: Standard notification center.
    - *OHC Advantage*: The Feed is the primary interface. It turns operational work into a series of simple approvals.

  ## Design Doc
  ### Architecture (Mermaid)
  ```mermaid
  graph TD
      A[Webhooks/Internal Events] --> B(Event Bus / Queue)
      B --> C{Agent Router}
      C -->|Customer Message| D[Customer Success Agent]
      C -->|Low Stock| E[Operations Agent]
      C -->|New Booking Request| F[Sales Agent]
      D --> G(Action Card Generator)
      E --> G
      F --> G
      G --> H[(Feed Database)]
      H --> I[Mobile Frontend - The Feed]
  ```

  ### Mobile UX Flow (375px)
  1. **The Feed Screen**: The default view upon opening the app. A vertically scrolling list of "Action Cards".
  2. **Action Card Design**: Clean, translucent glass styling.
     - **Header**: Agent icon (e.g., Customer Success), Timestamp.
     - **Context**: "New DM from @user on Instagram asking about vegan cakes."
     - **Drafted Action**: "Drafted Reply: 'Yes, we have vegan cakes available! Would you like to place an order?'"
     - **Actions**: Massive touch targets for `[ Approve & Send ]`, `[ Edit ]`, `[ Discard ]`.
  3. **Interaction**: Tapping `Approve` instantly executes the action (via webhook/API) and marks the card as resolved, removing it from the active feed.

  ### Data Model
  - `FeedItem` table:
    - `id` (UUID)
    - `tenant_id` (String)
    - `source` (String - e.g., 'instagram', 'inventory')
    - `agent_type` (Enum - e.g., 'customer_success', 'operations')
    - `context_text` (String)
    - `draft_action_payload` (JSONB - The API payload to execute if approved)
    - `status` (Enum - 'pending', 'approved', 'dismissed')
    - `created_at` (Timestamp)

  ## Implementation Prompt
  **Feature Name**: OHC Agentic Feed
  **Target Persona**: Maya the Baker
  **Outcome**: Maya opens the OHC app and sees a feed of actionable cards. She sees an AI-drafted reply to an Instagram DM, taps "Approve", and the reply is sent automatically.

  **Next Actions for Implementer**:
  1. Create the `FeedItem` database schema with Row-Level Security for multi-tenant isolation.
  2. Build the backend API routes to fetch pending `FeedItem`s and a route to execute/approve an action based on its `draft_action_payload`.
  3. Develop the Mobile-First Flutter UI for the "Feed" screen, prioritizing large touch targets and the premium translucent glass design language.
  4. Ensure end-to-end tests cover the flow: Feed item generation -> Display in UI -> Approval action execution -> Item marked resolved.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
