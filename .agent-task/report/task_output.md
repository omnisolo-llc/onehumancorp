issue_title: "Implement Unified Agent Feed (Mobile-First Operations Hub)"
issue_description: |
  # Issue Brief: Unified Agent Feed

  ## Problem Statement
  Business owners (like Maya the baker and Carlos the handyman) are overwhelmed by complex admin dashboards that are only usable on a desktop browser. When they open OHC on their 375px phone screen, they need to instantly see what needs attention (e.g., pending order replies, discount suggestions, automated task approvals). They don't have time to navigate multiple menus; they need a single, unified "Agent Feed" where AI agents push drafted work to them for simple 1-tap approvals.

  ## Research Report
  - **Competitive Analysis**:
    - **Shopify & Wix**: Built fundamentally around a desktop-first dashboard paradigm. Their mobile companion apps are good for viewing stats and fulfilling orders but fail at complex operations (like setting up workflows or discount rules) on a small screen.
    - **Squarespace & GoDaddy**: Focus mostly on fast initial AI site generation but lack proactive, agent-driven operations hubs.
    - **Link-in-Bio Tools (Stan Store, Linktree)**: Capitalize on mobile-first creators but lack robust inventory, backend business logic, and agentic workflows.
  - **OHC Differentiator**: OHC uses an "Approval" interface paradigm. Agents do the heavy lifting (drafting emails, monitoring inventory, scheduling tasks) and push these "Action Cards" to a unified feed on mobile.
  - **Codebase Findings**: The existing `src/ui/tauri/` desktop app (canonical UI) is missing the Agent Feed capability that was prototyped in the deprecated Next.js app (`UnifiedAgentFeed.tsx`, `AgentActionCard.tsx`).

  ## Design Doc

  ### Architecture diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Event Pipeline (Webhooks, Cron)] --> B[Agent Workflow Engine]
      B --> C[Intent & Context Resolution LLM]
      C --> D[Draft Generation & Agent Feed DB]
      D --> E[Mobile-First UI 375px]
      E --> F{User Action on Card}
      F -- Approve --> G[Execute Action API]
      F -- Dismiss/Edit --> H[Update Feed State]
  ```

  ### UI wireframes or screen flow description (375px first)
  - **Home View**: A vertical scroll feed of translucent, macOS-style glassmorphism cards.
  - **Card Anatomy**:
    - **Agent Avatar/Icon**: Distinguishes the department (Marketing, Ops, Support).
    - **Context Summary**: E.g., "3 new custom cake orders need your reply."
    - **Proposed Action/Draft**: Expandable drafted message or scheduled task.
    - **Primary Action Button**: Massive (min 44x44px) "Approve & Send" or "Publish" button.
    - **Secondary Button**: "Edit" or "Dismiss".

  ### Mobile UX flow
  1. Owner opens the OHC app and lands directly on the Unified Agent Feed (no deep menu navigation).
  2. Owner scrolls through today's prioritized cards (e.g., pending quotes, drafted Instagram DMs).
  3. Owner taps "Approve" on a drafted DM.
  4. The card transitions to a success state ("Message Sent") and visually collapses, promoting the next card up.
  5. The UI remains clean, locked to 375px without horizontal scrolling.

  ### AI agent integration points
  - The UI feed subscribes to the AI job queue or feed state where agents (e.g., Marketing Agent, Operations Agent) publish action payloads.
  - The feed must handle varying payload types (e.g., a "Draft Email" payload vs. a "Discount Proposal" payload) and render the appropriate card variant.

  ### Key design decisions and why
  - **Mobile-First Absolute**: The primary testing viewport is 375px. Complex forms are replaced by Agent-proposed action cards because owner operators do not have the time or precision to navigate multi-step configurations on their phone.
  - **One-Tap Approvals**: Business logic is executed by the agent; the human merely signs off, preserving owner control while maximizing velocity.
  - **Glassmorphic Tokens**: UI will follow OHC Premium Tokens (translucent layers) to ensure the app feels like a high-end assistant, not a clunky admin portal.

  ## Implementation Prompt
  Implement the "Unified Agent Feed" in the canonical Tauri UI (`src/ui/tauri/src/ui/`).

  **User-Facing Outcome & CUJ**:
  1. The user launches the app (simulated 375px width).
  2. They see a vertical feed of Agent Action Cards (e.g., drafted reply, suggested promo).
  3. They can interact with the card (e.g., click "Approve" on a drafted message), and the card visually resolves and updates the backend state.

  **Acceptance Criteria**:
  - The feed and cards must be 100% responsive and optimized for a 375px viewport with no horizontal scrolling.
  - Touch targets for all buttons must be at least 44x44px.
  - Visual design must apply OHC Premium Tokens (macOS-style glassmorphism).
  - Write robust Playwright E2E tests (`bazelisk test //src/e2e:playwright`) covering the initial rendering of the feed, card interaction (Approve/Dismiss), and correct API calls or state updates.
  - Ensure zero mock data is present in the final UI code. Data should flow from actual backend APIs.
  - Validate functionality by opening the running UI and navigating the feature.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
