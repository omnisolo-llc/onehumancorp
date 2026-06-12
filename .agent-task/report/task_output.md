issue_title: "Research: Mobile-First Agentic Workflows for Local Services"
issue_description: |
  # Research Report: Mobile-First Agentic Workflows for Local Services

  ## Problem Statement
  Small business owners operating local service businesses (like Carlos the handyman or Leo the tutor) lack the tools to effectively manage operations while on the go. Existing solutions (Shopify, Wix) are desktop-first, highly complex, and require the owner to act as the primary operator for administrative tasks. Local service owners need an assistant-first mobile experience where AI agents can proactively handle booking, quoting, deposits, and customer follow-ups from a 375px mobile screen.

  ## Research Report
  - **The Mobile-First Mandate**: The core demographic (field service, local tutors, food carts) operates almost entirely from a mobile phone (often Android). Desktop-first SaaS platforms fail because the interface is not designed for one-handed operation or offline-tolerant usage.
  - **Agentic vs. Manual Workflows**: Traditional platforms rely on the user manually configuring apps (e.g., Klaviyo for abandoned carts, Acuity for scheduling). An agentic workflow involves AI natively understanding the state (e.g., a missed lead or pending quote) and drafting a response or creating a task for the owner to review.
  - **Competitive Landscape**:
    - *Shopify/Wix*: Not designed for service businesses. Hard to manage bookings or field operations without expensive third-party plugins.
    - *Housecall Pro/Jobber*: Excellent for operations but lack true conversational AI or automated customer relationship recovery. They act as databases, not active assistants.
  - **OHC Opportunity**: Provide an integrated "Work Triage" mobile feed where incoming service requests, pending quotes, and automated customer follow-ups are surfaced by agents. The owner can approve, modify, or reject AI-drafted actions with a single tap.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px] -->|Work Feed| B(API Gateway)
      B --> C{Work Triage Service}
      C --> D[Operations Agent - Bookings & Tasks]
      C --> E[Customer Success Agent - Follow-ups]
      C --> F[Sales Agent - Quotes & Deposits]
      D --> G[(PostgreSQL - Multi-Tenant)]
      E --> G
      F --> G
      G --> H[Event Bus]
      H --> I[AI Job Queue]
  ```

  ### Mobile UX Flow (375px)
  1. **The Daily Command Center**: The app opens to a single, unified "Work Feed" rather than a dashboard of charts.
  2. **Actionable Cards**: Each card in the feed represents an item needing attention (e.g., "New Quote Request from Sarah").
  3. **Agent Recommendations**: Below the request, the AI agent provides a drafted response or proposed quote.
  4. **One-Tap Approval**: The owner can tap "Approve & Send," "Edit Draft," or "Dismiss."
  5. **Offline Tolerance**: Actions taken in poor network conditions are queued and synchronized when connectivity is restored.

  ### AI Agent Integration
  - **Work Triage**: Unifies messages, tasks, and alerts into a prioritized feed. Explains why an item is in the feed.
  - **Operations Assistant**: Automatically suggests calendar slots for new requests based on existing commitments and location routing.
  - **Sales Assistant**: Drafts initial estimates based on historical data for similar jobs and automatically follows up on unaccepted quotes after 48 hours.

  ## Implementation Prompt
  Implement the core "Work Feed" mobile UI and the backend `Work Triage Service`.
  - **Frontend**: Create a Flutter/PWA mobile-first layout (375px target) featuring a scrollable list of actionable cards. Implement the "macOS-style Translucent Glass" design tokens.
  - **Backend**: Build a unified API endpoint that aggregates pending tasks, messages, and agent recommendations into a single stream.
  - **AI Integration**: Ensure the `Customer Success Agent` can insert draft responses into the feed for owner review. The system should not auto-send until approved.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
