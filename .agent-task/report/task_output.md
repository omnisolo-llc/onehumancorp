issue_title: "Agent Feed Draft Action UI/UX implementation"
issue_description: |
  **Problem Statement**
  The "Agent Feed" is a core differentiation for OHC. The research doc `docs/business/market_research/agent_feed_deep_dive.md` mentions that "Users receive 'Action Cards' in their feed containing the drafted message/action and simple 'Approve', 'Edit', or 'Discard' buttons." While the backend handles `process_event` and `PENDING_APPROVAL` lifecycle state logic, there is no corresponding user-facing UI in the mobile-first Flutter application. Non-technical users cannot currently see or approve drafted actions.

  **Research Report**
  The gap identified in `docs/business/market_research/agent_feed_deep_dive.md` highlights that instead of requiring business owners to search for tasks, OHC proactively pushes "Action Cards" (Agent Feed items) to the user's mobile device for approval. Competitor apps (Shopify Inbox, Wix Inbox) aggregate messages but lack proactive AI draft action resolution. The missing link is the UI where users review drafts and approve them. The Go backend exposes these drafts via endpoints that are not yet wired up in the front-end Flutter app.

  **Design Doc**
  - **Architecture Diagram (Mermaid)**:
  ```mermaid
  graph TD
      A[Backend Go /api/agent_feed] --> B[Flutter UI Fetcher]
      B --> C[Mobile First Feed Screen 375px]
      C --> D[Action Card Component]
      D --> E{User Interaction}
      E -->|Approve| F[Call /api/agent_feed/update_state]
      E -->|Edit| G[Open Modal/Inline Edit]
      E -->|Discard| H[Call /api/agent_feed/update_state]
  ```
  - **Mobile UX Flow (375px first)**: The user opens the OHC app. The first tab is "Feed". A list of Action Cards is displayed using a translucent glass style background (`backdrop-filter: blur(30px) saturate(210%)`, `background: rgba(255, 255, 255, 0.65)`). Each card shows the `event_source` (e.g., "Instagram DM"), `intent`, and the `draft_action`. The card has large touch targets (>= 44x44px) for "Approve", "Edit", and "Discard".
  - **AI Agent Integration Points**: The UI should reflect the status as `PENDING_APPROVAL` and, upon interaction, update the lifecycle state in the database, allowing the backend's dispatch agent to execute the action.

  **Implementation Prompt**
  Implement the "Agent Feed" screen in the Flutter front end. Create a mobile-first list view that fetches items from `/api/agent_feed` and displays them as Translucent Glass cards. Each card must display the AI-drafted response and have "Approve", "Edit", and "Discard" buttons. Tapping "Approve" should call the update endpoint. Include Playwright E2E tests simulating a user (Maya) logging in, seeing a drafted response card in the feed, and tapping "Approve". Ensure touch targets are at least 44x44px and the layout works perfectly on a 375px viewport.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
