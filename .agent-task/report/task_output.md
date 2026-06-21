issue_title: "[Research] Omnichannel Work Triage & Agentic Event Feed Architecture"
issue_description: |
  # Research Report: Omnichannel Work Triage & Agentic Event Feed Architecture

  ## Problem Statement
  Small-business owners (like Maya the Baker or Carlos the Handyman) suffer from "inbox fragmentation" and "alert fatigue." They receive Instagram DMs, SMS text messages, booking requests, payment confirmations, and system alerts in separate, disconnected silos. Current platforms either offer basic notification lists or require expensive, complex Helpdesk software (like Zendesk) which is completely inappropriate for an independent operator. The owner needs a unified assistant that not only centralizes these inputs but *triages* them, groups them by context, and drafts actionable responses automatically.

  ## Research Report
  - **Shopify / Wix / Squarespace**: These platforms provide standard notification feeds (e.g., "New Order #1001"). However, they do not integrate external communication channels natively (like Instagram DMs or WhatsApp) without third-party apps. Their notifications are passive—they tell the user what happened, but do not do the work to resolve it.
  - **Zendesk / Intercom (Complex Integrators)**: Built for support teams, not single owners. They lack deep integration with the core operational objects (inventory, bookings, payments).
  - **The OHC Opportunity**: Because OHC is an *assistant-first* platform, we can build a unified "Agent Feed." Instead of a passive notification list, the feed acts as an inbox of "Pending Agent Actions." When a DM arrives asking for a custom cake quote, the Customer Assistant agent intercepts it, drafts the quote using the Sales Agent capabilities, and places a single "Approve Quote for Maya" card in the feed.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: IG, SMS, Stripe] -->|Ingress| B[Event Gateway & Normalization]
      B --> C[Postgres: Event Store]
      C --> D[AI Triage Agent Worker]
      D -->|Classifies Intent| E[Agent Dispatch Router]
      E --> F[Customer Assistant Agent]
      E --> G[Operations Agent]
      E --> H[Finance Agent]
      F -->|Drafts Reply| I[Agent Feed Card]
      G -->|Proposes Schedule| I
      H -->|Drafts Invoice| I
      I --> J[Owner Mobile App Feed]
  ```

  ### Mobile UX Flow (375px First)
  1. **The Work Triage View**: The default landing screen is a clean, Apple-styled feed.
  2. **Action Cards**: Each item in the feed is a "Card" containing:
     - The Context (e.g., "New message from @jane_doe on Instagram")
     - The Intent (e.g., "Wants to book a plumbing repair for Tuesday")
     - The Agent Draft (e.g., A drafted reply proposing available times and a deposit link).
  3. **Interaction**: The user sees two primary buttons with 44x44px touch targets: `[ Approve & Send ]` or `[ Edit Draft ]`.
  4. **Resolution**: Once approved, the card animates away, achieving "Inbox Zero" for daily operations.

  ### AI Agent Integration Points
  - **Triage LLM Pipeline**: A fast, low-latency LLM pass classifies incoming events into standard intents (Inquiry, Complaint, Booking Request, Spam).
  - **Department Handoffs**: The Triage LLM routes the task to the appropriate department. For instance, an "Inquiry" goes to the Customer Assistant, while a "Payment Failed" webhook goes to the Finance Assistant to draft a gentle follow-up email.

  ### Key Design Decisions
  - **Postgres SKIP LOCKED for the Agent Queue**: Ensures that multiple background workers do not process the same inbound event simultaneously.
  - **Immutable Event Log**: All inbound events are stored immutably to provide complete auditability for the AI's decision-making process.
  - **Optimistic UI Updates**: When the owner taps "Approve", the UI immediately reflects success while the background worker handles the actual API call to the external channel.

  ## Implementation Prompt
  **Feature Name**: Agentic Work Triage Feed
  **Target Personas**: Maya, Carlos, Priya
  **User-Facing Outcome**: The user logs into the OHC app and sees a prioritized list of drafted actions (replies, quotes, follow-ups) ready for their 1-tap approval, instead of a raw list of notifications.
  **Critical User Journey (CUJ)**:
  1. User opens the OHC mobile app.
  2. User sees an Action Card: "Customer X asked for a quote. I drafted one for $150 based on your pricing guidelines."
  3. User taps "Approve".
  4. The system sends the quote and marks the task complete.
  **Acceptance Criteria**:
  - Implement the `Event Gateway` to ingest and normalize multi-channel inputs.
  - Build the Postgres-backed job queue for the Triage Agent.
  - Create the `Agent Feed Card` UI components in the mobile-first frontend following OHC Premium Design Tokens.
  - Integrate with the Gemini Pro LLM to generate actionable drafts for at least one intent type (e.g., "Pricing Inquiry").

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
