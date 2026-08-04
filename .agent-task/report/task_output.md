issue_title: "Native Rust Omnichannel Chat System & Ambassador Agent"
issue_description: |
  # Title
  Native Rust Omnichannel Chat System & Ambassador Agent

  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. We previously relied on a third-party omnichannel provider, but as an external service, it is now 100% retired. OHC must replace it with a native, high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside the `onehumancorp/mono` repository, integrated seamlessly with the Ambassador Agent.

  # Research Report
  - **Shopify Inbox & Wix Inbox:** Basic aggregation, manual responses, lacks autonomous context-aware AI drafting.
  - **Zendesk/Intercom:** Too complex/expensive for single-person SMBs.
  - **Legacy Ruby System Source Code Audit:** I reviewed the legacy system's core architecture by cloning its repository and inspecting its Ruby on Rails implementations (`app/models`). Key findings that OHC needs to replicate in Rust:
    - `Conversation` model tracking `status`, `snoozed_until`, `assignee_agent_bot_id`, `contact_last_seen_at`.
    - `Inbox` model with configurations for `auto_assignment_config`, `channel_type`, `working_hours_enabled`.
    - Diverse `Channel` models (e.g., `whatsapp`, `twitter_profile`, `instagram`, `email`, `api`, `web_widget`) demonstrating a polymorphic association pattern where the Inbox belongs to a Channel.
    - Webhooks and WebSocket real-time event dispatch for live typing and message delivery.
  - **OHC Opportunity:** Implement the legacy system's feature parity natively in Rust. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Rust Omnichannel Gateway)
      B --> C{Rust Channel Adapters}
      C --> D{Customer Identity Resolution Engine}
      D -->|Lookup/Create| E[Unified Customer Graph DB - Postgres/RLS]
      D --> F[Rust Event Mesh / Message Bus]
      F --> G[The Ambassador Agent]
      G -->|Query Context| E
      G -->|Draft Reply| H[Action Required Queue - SKIP LOCKED]
      H --> I[Mobile App Feed 375px - Flutter PWA]
      I -->|1-Tap Approve| J[Rust Omnichannel Dispatcher]
      J --> C
      C --> A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing. Ubiquiti UniFi modular dashboard card layouts.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Native Rust Implementation:** Full replacement of the legacy omnichannel service utilizing Rust for the message processing engine, webhook receivers, and WebSocket real-time dispatch, ensuring zero-trust multi-tenancy with PostgreSQL RLS.
  - **Data Models:** Create `Inboxes`, `Conversations`, `Messages`, and `Contacts` mirroring the legacy schema but strongly typed in Rust and scoped strictly by `tenant_id`.
  - **Proactive Drafting:** Move from read-reply to read-approve. AI drafts the response before the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points.

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the native Rust Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier to an existing customer record.
  3. The Ambassador Agent is triggered and successfully queries the customer's past orders and the current product catalog.
  4. The Agent generates a draft reply and places it in the ActionRequiredQueue for the specific tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed (375px), taps "Approve," and the system dispatches the message back to the mocked external channel.
  6. The backend components MUST be implemented natively in Rust within the `onehumancorp/mono` repo, achieving full legacy feature parity (inboxes, conversations, channels) without external dependencies.
  7. All new APIs must have 100% unit test coverage in Rust, and the end-to-end flow verified in Playwright.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
