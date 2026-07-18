issue_title: "Implement the Agent Feed Core and Work Triage Workflow"
issue_description: |
  # Research Report: The Agent Feed & Work Triage Capability

  ## 1. Problem Statement
  Small business owners (e.g., Maya the Baker, Nora the Agency Principal) are currently overwhelmed by fragmented operations. They use separate apps for messaging (Instagram, WhatsApp, Email), separate apps for task management, and separate dashboards for bookings and payments. This leads to the "app tax" and requires constant context switching. Traditional e-commerce and builder platforms (like Shopify or Wix) offer dashboards, but these are passive—they wait for the owner to actively search for information and interpret what needs to be done.

  ## 2. Research Report
  - **Market Context:** Platforms like Shopify have attempted to add AI (e.g., Sidekick), but these are primarily reactive chat interfaces. A merchant must know what to ask. Other platforms require users to stitch together tools (Zapier + Mailchimp + Shopify + Zendesk) to create an automated flow, which is out of reach for most micro-SMEs.
  - **The OHC Opportunity:** The core of the OneHumanCorp promise is to guide users from "unclear work → clear next action in minutes" via "Invisible AI Automation." The Agent Feed is the central architectural piece that makes this possible. It proactively pushes prioritized tasks, drafted responses, and critical business alerts directly to the owner's feed, acting as an intelligent work triage assistant.
  - **Competitor Gaps:**
    - *Shopify:* Passive dashboard; reactive AI; heavy reliance on third-party apps for automated workflows.
    - *Wix/Squarespace:* Basic setup assistants, no proactive daily operational management.
    - *Zendesk/Intercom:* Powerful unified inboxes, but overly complex for a solopreneur and disconnected from core inventory/booking data.

  ## 3. Design Doc (Architecture & UI Flow)
  ### High-Level Architecture
  - **Event Ingestion (Work Triage):** A unified ingestion layer that captures external webhooks (e.g., Stripe payments, Instagram Graph API DMs) and internal domain events (e.g., inventory low stock, new booking request).
  - **Intent & Context Engine:** An LLM-powered classification layer. When an event arrives (e.g., a customer asks about a custom cake), the system queries the tenant's RAG context (inventory, policies, past orders) and the Customer Success Agent drafts a proposed response or identifies the next required action.
  - **Agent Feed Storage:** A new data model in PostgreSQL to store `FeedItem` records (linked to a `tenant_id` and specific entities like `Message`, `Order`, or `Task`).
  - **Push/Sync:** The backend pushes new `FeedItem` updates to the frontend (via WebSockets/SSE or standard polling fallback) to update the owner's feed in real-time.

  ### Mobile UX Flow (375px First)
  1. **The Command Center (Home Screen):** Maya opens the OHC mobile app. The first screen is NOT a static dashboard of charts. It is the **Agent Feed**.
  2. **Action Cards:** The feed consists of "Action Cards" sorted by priority.
     - *Example Card 1:* "Urgent: Customer asked about vegan cakes. [Drafted Reply: Yes, we have 3 left!]" -> Actions: `[Approve]` | `[Edit]`
     - *Example Card 2:* "System: Inventory for Red Dress is below 5. Draft a restock order?" -> Actions: `[Yes, draft it]` | `[Dismiss]`
  3. **One-Tap Resolution:** Maya taps `[Approve]` on the message draft. The feed item visually resolves (e.g., turns green, collapses, or moves to a 'Done' section), and the underlying action is executed asynchronously.

  ## 4. Implementation Prompt
  **Feature Name:** Core Agent Feed & Work Triage (Phase 1)
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya can open OHC on her phone and see a prioritized feed of action items. For this phase, implement the core `FeedItem` data model, the backend API to query the feed, and the mobile-first UI for the "Command Center" feed view with basic interactive Action Cards.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. The user logs into OHC and lands on the "Agent Feed" home screen.
  2. The screen must be perfectly responsive and usable on a 375px mobile viewport (no horizontal scrolling).
  3. The feed should display at least two types of `FeedItem` mock/seed data to demonstrate the UI (e.g., a "Drafted Message Review" card and a "System Alert" card). *Note: The final implementation must use real data from the backend, but seed data is acceptable for the initial PR to build the UI.*
  4. Each card must have clear primary actions (e.g., "Approve", "Dismiss").
  5. Interacting with a card (e.g., clicking "Dismiss") must trigger a backend API call to update the feed item state and optimistically update the UI to remove or resolve the card.
  6. **Mandatory E2E Testing:** Write a Playwright test (`src/e2e/agent-feed.spec.ts`) that logs in, navigates to the feed, verifies the presence of cards, clicks an action button on a card, and verifies the card is handled correctly (e.g., disappears or changes state).

  **Priority:** P0 (Core Platform Requirement)
  **Estimated Scope:** Medium/Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
