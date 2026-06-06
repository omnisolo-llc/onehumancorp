issue_title: "Implement the 'Unified Agent Feed' for Mobile-First Dashboard Management"
issue_description: |
  ## Mission Queue Protocol: Unified Agent Feed (Mobile MVP)

  **Target Persona**: Fatima the Food Cart Operator, Maya the Baker

  **Problem Statement**:
  Business owners running everything from a mobile phone lack an intuitive way to manage complex back-office tasks across different operational areas (Marketing, Operations, Advisory). Current legacy platforms require returning to a desktop web view to manage store configuration or respond to specific insights, which frustrates non-technical users. The modern mobile user expects an experience closer to a social feed where actionable items are presented clearly.

  **Research Report**:
  - Legacy platforms (Shopify, Wix) treat mobile apps as simple dashboards for viewing stats.
  - Link-in-Bio tools (Linktree) succeed via absolute simplicity but lack real business operations.
  - As detailed in `docs/business/market_research/ohc_smb_mobile_first_design_research.md` and `docs/business/market_research/agent_feed_deep_dive.md`, the solution is an "Approval" interface paradigm where the various AI Agents surface proactive Action Cards (e.g., "3 new orders to fulfill", "Drafted Instagram reply").
  - This fundamentally shifts the business owner from "doing the work" to "approving the work."

  **Design Doc**:
  - **Architecture**:
    - **Data Pipeline**: The feed will aggregate `AgentEvent` rows or real-time pub/sub notifications from different agent departments.
    - **Display**: A vertical scroll feed of `ActionCard` components.
  - **Mobile UX Flow (375px)**:
    - User opens the OHC app.
    - Instead of traditional complex navigation, they see a vertical feed.
    - Feed items clearly distinguish agent types (e.g., via icon or badge: Marketing vs. Operations).
    - Each card contains the proposed action/insight and one to three large touch targets (e.g., "Approve & Post", "Edit", "Dismiss").
  - **AI Integration**: The feed serves as the direct presentation layer for AI agent outputs (e.g., the Ambassador agent drafts a reply, and it appears here for Maya to approve).

  **Implementation Prompt**:
  - Create the `AgentFeed` and `ActionCard` UI components.
  - Integrate them into the main mobile dashboard route.
  - Ensure the layout strictly adheres to 375px width constraints (no horizontal scrolling).
  - All interactive elements must have minimum 44x44px touch targets.
  - Use OHC Premium Tokens (Glassmorphism, clean typography).
  - Implement at least 3 mock/sample agent card types (e.g., Operations: fulfill order, Advisory: draft email, Marketing: approve post) that the UI correctly renders.
  - Create a Playwright E2E test verifying the feed renders correctly and interactive elements work.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
