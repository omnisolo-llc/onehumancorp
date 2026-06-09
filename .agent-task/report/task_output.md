issue_title: "Design Unified Work Triage & Agent Feed Architecture"
issue_description: |
  # Mission Queue Protocol: Unified Work Triage & Agent Feed Architecture

  ## Problem Statement
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) face a daily barrage of scattered information: Instagram DMs, payment confirmations, booking requests, and inventory alerts. Existing platforms like Shopify or Wix rely on passive dashboards where the owner must hunt for what needs attention. OHC's core promise is to provide a "Tencent Workbuddy-like AI work assistant" that proactively triages work. Without a unified, prioritized Agent Feed, owners lose momentum, miss leads, and suffer from operational cognitive overload. They need an assistant that tells them exactly what needs attention *today* and proposes the next action.

  ## Research Report
  - **Market Context**: Traditional SMB platforms (Shopify, Wix, Squarespace) use static admin panels. Shopify's "Sidekick" is a passive chatbot that gives advice but does not proactively queue work.
  - **The OHC Opportunity**: By acting as the central nervous system, OHC can intercept webhook events (Stripe, Instagram Graph API, local inventory changes), classify their intent using LLMs, and push actionable "Agent Drafts" directly to a unified feed.
  - **Competitor Gaps**: Competitors require merchants to check 5 different apps (email, social media, POS, bookings). OHC integrates these into a single "Work Triage" feed, effectively saving the owner hours of context-switching daily.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **The Morning Briefing**: The user opens the OHC app and sees the "Today" feed.
  2. **Action Cards**: Each feed item is a translucent glass card (OHC Premium Token library, 16px border-radius). It summarizes the event (e.g., "3 new cake inquiries overnight").
  3. **One-Tap Resolution**: The card includes the Agent's drafted response or proposed action (e.g., "Drafted 3 replies. 2 require a deposit link.").
  4. **Interactivity**: The user can tap "Approve All", "Edit Draft", or "Dismiss" (min 44x44px touch targets).

  ### AI Agent Integration Points
  - **Event Ingestion & Router**: An async queue (PostgreSQL `SKIP LOCKED`) receives raw events and routes them to the correct Agent Department (Customer, Operations, Sales).
  - **Intent & RAG**: The assigned agent queries the tenant's memory (past bookings, inventory state, preferences) to establish context.
  - **Drafting Engine**: The agent generates an actionable output (a message draft, a schedule change, an invoice) and publishes it to the Feed Store.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External/Internal Events] -->|Webhooks, Cron| B(Event Ingestion Pipeline)
      B --> C{AI Triage Router}
      C -->|Customer Query| D[Customer Assistant Agent]
      C -->|New Booking| E[Operations Assistant Agent]
      C -->|Payment Pending| F[Sales Assistant Agent]
      D --> G[(Tenant Memory / RAG)]
      E --> G
      F --> G
      D --> H[Draft Action / Reply]
      E --> H
      F --> H
      H --> I[Unified Feed Store DB]
      I -->|Push/Pull| J[Mobile App Feed UI - 375px]
  ```

  ### Key Design Decisions
  - **Push, Don't Pull**: The feed is populated asynchronously by agents. The UI simply renders the state of the Feed Store.
  - **Strict Multi-Tenancy**: Feed items and memory lookups are hard-bounded by `tenant_id` to prevent cross-contamination.
  - **Resilient Mobile Writes**: Approvals are processed optimistically in the UI and synced to the backend with exponential backoff for flaky networks.

  ## Implementation Prompt
  **Implementer Agent Task**: Build the end-to-end Work Triage Feed.
  - **User-Facing Outcome**: As Maya the Baker, I want to open the OHC app on my iPhone (375px wide) and see a single scrolling feed of actionable items (e.g., drafted IG replies, low inventory alerts). I should be able to tap "Approve" on a drafted reply and have it disappear from my feed.
  - **Critical User Journey (CUJ)**:
    1. A mock external event (e.g., a customer inquiry) enters the system.
    2. The backend Agent processes it and creates a Feed Item.
    3. The owner logs into the UI and sees the Feed Item card.
    4. The owner taps the primary action button ("Approve").
    5. The Feed Item updates its state to "Resolved" and disappears from the active feed.
  - **Acceptance Criteria**:
    - 100% responsive on 375px.
    - Uses OHC Premium translucent glass styling.
    - Full E2E Playwright test covering the CUJ (from feed item appearance to resolution).
    - Unit test coverage at 100% for the Feed Store and Router logic.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
