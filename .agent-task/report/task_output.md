issue_title: "Research: Mobile-First Agentic Workflows for SMB Operators"
issue_description: |
  # Research: Mobile-First Agentic Workflows for SMB Operators

  ## Problem Statement
  SMB operators like Maya (baker), Carlos (handyman), and Fatima (food cart) run their businesses predominantly on mobile devices. They need an AI work assistant that operates seamlessly in a 375px viewport, turning incoming demand and operational chaos into clear, actionable next steps. Currently, many platforms force SMBs into complex, desktop-centric admin dashboards that are slow, hard to navigate on mobile, and require too much manual data entry.

  ## Research Report
  - **Market Context**: Platforms like Tencent Workbuddy and WeCom succeed because they integrate business operations into a unified mobile interface.
  - **Competitor Analysis**: Shopify and Wix have capable mobile apps, but they still feel like "admin portals". OHC must feel like a "work assistant". The assistant should proactively coordinate messages, calendar, documents, and payments.
  - **Identified Gap**: OHC lacks a unified, highly responsive mobile-first (375px) "Work Triage" feed that natively integrates with background AI agents (Operations, Customer Service, Sales).

  ## Design Doc
  - **Architecture**:
    - **Frontend**: Flutter PWA optimized for 375px. Uses OHC Premium Token library (translucent materials, clean card layouts).
    - **Backend**: Go gRPC API serving the triage feed.
    - **Agent Coordination**: Background agents process incoming webhooks (e.g., Stripe, Instagram DMs) via PostgreSQL `SKIP LOCKED` job queue and publish prioritized action cards to the tenant's feed.
  - **Mobile UX Flow**:
    1. **Home Screen (375px)**: The "Work Triage" feed. Top card is always the highest priority next action (e.g., "Draft reply to new cake order from Maya").
    2. **Interaction**: Tap to review the AI-drafted reply or proposed action. One-tap to approve and send/execute.
    3. **Background**: Agent executes the approved action and updates the feed asynchronously.
  - **AI Integration**:
    - `Work Triage Agent`: Analyzes incoming data (messages, payments, bookings) and generates a prioritized `ActionCard` for the feed.
    - `Action Execution Agent`: Handles the execution of approved actions.

  ## Implementation Prompt
  Implement a "Work Triage" mobile-first feed for the OHC Flutter app and the backing Go gRPC endpoints.

  **Requirements**:
  - Build a highly responsive UI in Flutter using the OHC Premium Token library, optimized for a 375px viewport.
  - Implement a Go gRPC service (`WorkTriageService`) with an endpoint `GetTriageFeed(TenantID) -> List<ActionCard>`.
  - The UI must render `ActionCard` items cleanly, allowing one-tap approval or dismissal.
  - No mock data: The feed must be populated from a PostgreSQL table (`triage_items`) and updated via a backend worker process that simulates incoming demand (e.g., a dummy HTTP endpoint to trigger a new item).
  - Include 100% unit test coverage for Go endpoints and at least 5 Playwright E2E tests validating the mobile (375px) UI flow.
  - Ensure zero trust/multi-tenancy: The Go endpoint MUST filter by `tenant_id` from the context.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
