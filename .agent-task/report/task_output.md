issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  # Research Report: Unified Agent Feed (Mobile MVP)

  ## Mission Queue Protocol

  This issue brief details the required work for implementing the Unified Agent Feed, transforming the traditional dashboard into a proactive, AI-driven action center for small business owners.

  ### Problem Statement
  Currently, SMB owners face a "dashboard dilemma." Existing platforms (Shopify, Wix) present dense analytics and complex navigation menus, forcing the user to hunt for what needs attention and then perform multi-step operations to resolve issues. This is particularly painful on mobile devices (375px), where legacy platforms often degrade gracefully at best or break entirely at worst. Non-technical users (like Maya the baker or Carlos the handyman) need an assistant that tells them what matters *now* and offers 1-tap actions.

  ### Research Report
  - **Market Context**: "Link-in-bio" tools (Linktree, Stan Store) succeeded by offering radical simplicity, but they lack the operational depth of a full business platform. Legacy platforms (Shopify) offer depth but fail on mobile simplicity.
  - **The Gap**: OHC must bridge this gap by replacing the static dashboard with a proactive "Agent Feed." Instead of showing a graph of sales, the feed shows actionable cards proposed by various AI agents (e.g., Marketing, Operations, Customer Success).
  - **Competitive Differentiation**: Unlike Microsoft Copilot or Notion AI, which are generic text interfaces, OHC's Agent Feed is structured around concrete business operations with explicit "Approve/Edit/Discard" workflows.

  ### Design Doc

  #### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
      A[Event Sources: Webhooks, DB Changes, Crons] --> B(Event Bus / Queue)
      B --> C{Agent Router}
      C -->|Operations| D[Operations Agent]
      C -->|Marketing| E[Marketing Agent]
      C -->|Advisory| F[Advisory Agent]
      D --> G[Draft Action Card]
      E --> G
      F --> G
      G --> H[(PostgreSQL: Agent Feed Table)]
      H --> I[Mobile API (REST/GraphQL)]
      I --> J[Flutter/Web UI: Unified Agent Feed]
  ```

  #### Mobile UX Flow (375px First)
  1.  **Launch**: User opens the app. The primary screen is the "Agent Feed," not a traditional dashboard.
  2.  **Feed View**: A vertical, scrollable list of "Action Cards." Each card uses OHC Premium Tokens (Glassmorphism, clean typography) and clearly identifies the originating Agent.
  3.  **Action Card Anatomy**:
      -   **Header**: Agent Icon & Title (e.g., "Operations Agent").
      -   **Context**: Brief summary (e.g., "3 new orders need fulfillment").
      -   **Proposed Action**: The drafted response or operation (e.g., "Fulfill all 3 orders and send shipping notifications").
      -   **Controls**: Large, touch-friendly buttons (≥ 44x44px). Primary action (e.g., "Approve"), Secondary actions (e.g., "Review Details", "Dismiss").
  4.  **Interaction**: Tapping "Approve" triggers the backend action immediately. The card transitions to a success state and is removed from the active feed.

  #### AI Agent Integration Points
  -   Agents must publish structured data (not just raw markdown text) to the Feed database table to render the Action Cards correctly.
  -   The system requires a persistent, multi-tenant `agent_feed_items` table to store these pending actions.

  ### Implementation Prompt
  **Objective**: Build the foundational Mobile MVP of the Unified Agent Feed, starting with the UI shell and the backend data structure to support actionable cards.

  **User-Facing Outcome**: When the user logs in, they see a clean, vertical feed of proposed actions on their mobile device (375px width), replacing the complex traditional dashboard.

  **Critical User Journey (CUJ)**:
  1.  Log in as a business owner (e.g., Maya).
  2.  The home screen displays the "Agent Feed".
  3.  The feed shows at least one simulated actionable card (e.g., "Drafted response for customer inquiry").
  4.  The user taps the primary action button (e.g., "Approve").
  5.  The card visually confirms success and disappears from the active view.

  **Acceptance Criteria**:
  -   Create the database schema/entity for `AgentFeedItem` (must support multi-tenancy).
  -   Implement the backend API to fetch and update feed items.
  -   Build the mobile-first (375px) React/Flutter UI for the feed and the Action Card component.
  -   Implement the interaction logic (Approve/Dismiss) that updates the backend state.
  -   Ensure all UI elements use OHC Premium Tokens and have ≥ 44x44px touch targets.
  -   No horizontal scrolling on 375px viewports.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
