issue_title: "[Research] Implement Mobile-First Unified Agent Feed UI"
issue_description: |
  **Problem Statement**
  Small business owners running physical services and retail locations manage their operations predominantly from mobile phones while on the go. Traditional legacy platforms require complex web dashboards to perform key actions. OneHumanCorp (OHC) agents are proactively identifying critical tasks (e.g. approving a new social media post, responding to abandoned carts, reviewing AI-drafted replies), but currently, there is no centralized, mobile-first feed for these high-value operational approvals in the canonical App.

  **Research Report**
  As identified in `docs/business/market_research/agent_feed_deep_dive.md` and `ohc_smb_market_dynamics_agentic_workflows.md`, competitor systems (Shopify, Wix) treat mobile apps merely as read-only companions for analytics, forcing desktop usage for operational configuration. OHC differentiates by replacing complex "configuration" with simple agent "approvals" directly on a 375px display. Currently, the Unified Agent Feed has been prototyped in the legacy Next.js web interface (`src/ui/next/src/app/dashboard/UnifiedAgentFeed.tsx`), and the underlying `api/agents/approvals` endpoints in Rust exist, but this core experience must be standardized and elevated into the primary application interface.

  **Design Doc**
  - **Architecture:** The canonical Tauri App (which is the main focus going forward as per `README.md`) must implement a `UnifiedAgentFeed` view.
  - **Mobile UX Flow:** Open the app -> The initial dashboard is a vertical feed of Agent Proposal cards. Each card displays the department (Operations, Marketing, etc.), the risk level, contextual data (e.g., potential revenue lost), and primary actions ("Approve", "Edit", "Decline").
  - **UI/UX specifics:** Target a 375px mobile viewport. Incorporate the OHC Premium Token design system (Glassmorphism styling, clean Typography, >44px touch targets).

  **Implementation Prompt**
  Implement the "Unified Agent Feed" in the active frontend experience for OHC.
  1. Translate the Next.js `UnifiedAgentFeed` component logic into the primary mobile-first frontend flow.
  2. The feed must fetch from `/api/agents/approvals` and render actionable cards.
  3. Clicking "Approve" must hit the backend decision endpoint and optimistically remove the card from the UI.
  4. The implementation must be verified via Playwright E2E tests focusing on the 375px mobile experience.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
