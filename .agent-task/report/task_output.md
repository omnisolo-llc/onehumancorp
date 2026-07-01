issue_title: "Implement Unified Agent Feed in the mobile dashboard"
issue_description: |
  **Problem Statement**
  The legacy dashboard in OHC attempts to show stats and various data points in a way that requires the user to proactively discover information. As observed in our Mobile-First Operations Paradigm research, non-technical small business owners (like Maya the baker and Fatima the food truck owner) need an "Approval" interface paradigm where the system pushes curated action cards (drafted messages, approvals, operational alerts) to them in a centralized feed. This Unified Agent Feed brings the "Invisible AI Automation" vision to life on a mobile-first (375px) display.

  **Research Report**
  Competitors like Shopify and Wix rely on complex admin interfaces that break down on mobile screens. We discovered that mobile-first operators need a Chat & Approval UI paradigm rather than nested menus and stat graphs.
  The `docs/business/market_research/ohc_smb_mobile_first_design_research.md` document highlights the need for a Unified Agent Feed that aggregates action cards from various agents (Marketing, Operations, Advisory).
  The endpoint `/api/ui/dashboard/unified-agent-feed` exists and returns `pending_approvals`, `entries`, and `agent_feed`. However, the current UI in `src/ui/tauri/src/ui/dashboard.html` fetches `/api/ui/dashboard/unified-feed` which contains legacy data shapes and merges them incorrectly, or misses rendering them properly.

  **Design Doc**
  - **Architecture diagram:**
    ```mermaid
    graph TD;
      DB[(PostgreSQL)] --> FetchAgentFeed[fetch_unified_agent_feed_data];
      FetchAgentFeed --> Cache[UI_UNIFIED_AGENT_FEED_CACHE];
      Cache --> API[/api/ui/dashboard/unified-agent-feed];
      API --> UI[dashboard.html Mobile View];
      UI --> ActionCards(Rendered Approval/Dismiss Cards);
    ```
  - **Mobile UX Flow:**
    1. User opens the dashboard.
    2. The app fetches `/api/ui/dashboard/unified-agent-feed`.
    3. The `unified-agent-feed-section` renders a list of Glassmorphic Action Cards for each pending approval and agent feed item.
    4. Each card contains clear context and "Approve" / "Dismiss" buttons with touch targets >= 44x44px.
  - **AI Agent Integration:**
    The AI agents deposit draft responses into the `agent_action_requests` and `agent_feed_items` tables, which the backend routes to the UI. The UI just needs to surface these and wire up the approval buttons.
  - **Key Design Decisions:**
    - We will update `dashboard.html` to fetch the correct endpoint (`/api/ui/dashboard/unified-agent-feed`) instead of the legacy `unified-feed`.
    - We will update the `loadUnifiedFeed` function to handle the new data structure (`pending_approvals`, `agent_feed`).
    - The `renderTriageItems` function will be updated to correctly parse the fields coming from the new endpoint.

  **Implementation Prompt**
  **Objective**: Update the mobile dashboard to consume the `/api/ui/dashboard/unified-agent-feed` endpoint and render the Unified Agent Feed Action Cards properly.

  **CUJ**:
  1. Open the app as a business owner on a mobile screen.
  2. In the "Command Center" (Unified Agent Feed section), see actionable cards with drafted emails, messages, or operational alerts.
  3. Be able to click "Approve" or "Dismiss" on these cards.

  **Acceptance Criteria**:
  1. The API call in `dashboard.html`'s `loadUnifiedFeed` function should point to `/api/ui/dashboard/unified-agent-feed`.
  2. The `agentFeedItems` array should correctly merge or use the `pending_approvals` and `agent_feed` arrays returned from the new endpoint.
  3. Action cards must render properly, adhering to 375px mobile constraints and having 44x44px minimum touch targets for buttons.
  4. Ensure any failing Playwright tests (e.g. `unified-agent-feed.mobile.spec.ts`) pass after these changes.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
