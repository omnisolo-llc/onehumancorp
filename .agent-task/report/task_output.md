issue_title: "[Research] AI-Driven Centralized Operations Hub for Physical & Service-based SMBs"
issue_description: |
  # Research Report: AI-Driven Centralized Operations Hub

  ## Problem Statement
  Small business owners and operators (Maya, Carlos, Priya, Fatima, etc.) currently lack a centralized operations hub that natively understands multi-modal workflows (orders, bookings, physical inventory, service routes). Existing tools force them into siloes: Shopify for e-commerce, Calendly for bookings, Square for POS, making unified mobile operations impossible. They need an AI-driven hub on a 375px mobile screen that groups actionable work, coordinates across capabilities, and anticipates next steps natively.

  ## Research Report
  - **The Missing Link**: Legacy platforms (Shopify, Wix) treat services and physical goods distinctly, creating fragmentation. The mobile apps are essentially read-only dashboards for desktop interfaces.
  - **Competitive Analysis**:
    - *Shopify*: Strong for physical products, poor for services/bookings. Requires desktop for complex configuration.
    - *Square*: Better physical/service blend but lacks proactive agentic coordination.
    - *HubSpot*: Great CRM, weak on physical operations and native mobile-first execution for SMBs.
  - **Opportunity**: OHC can differentiate by acting as the unified orchestration layer. By treating "work" as a unified feed of intent (a booking, an order, an inquiry) and having Agents coordinate the downstream modules, the operator stays in a single mobile-first command center.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[Mobile UI 375px] --> B[Unified Agent Feed API]
      B --> C[Work Triage Engine]
      C --> D[Customer Assistant Agent]
      C --> E[Operations Assistant Agent]
      C --> F[Sales & Revenue Agent]
      D --> G[(PostgreSQL Tenant DB)]
      E --> G
      F --> G
  ```
  ### Mobile UX Flow (375px)
  1. User opens the app.
  2. The Home screen displays a "Unified Agent Feed".
  3. Cards display grouped, actionable items (e.g., "3 pending cake inquiries", "1 missed lead from Carlos").
  4. User taps "Approve" or "Modify" on the card.
  5. The relevant AI Agent executes the action in the background.

  ### AI Agent Integration
  - **Work Triage Engine**: Ingests signals from all channels (DMs, orders, bookings) and groups them into actionable cards.
  - **Agents**: Draft responses, schedule routes, update inventory, and present the final plan for user approval.

  ## Implementation Prompt
  Implement the "Unified Agent Feed" mobile UI and the backend Work Triage Engine that aggregates signals into actionable cards. The UI must be fully functional on a 375px screen with large touch targets. The backend must ingest events (e.g., a new Instagram DM, a new booking request) and generate a prioritized feed item. Provide full E2E testing using Playwright to simulate a user opening the feed and approving an action. Do not prescribe specific database schemas or API endpoints; design the architecture to support multi-tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
