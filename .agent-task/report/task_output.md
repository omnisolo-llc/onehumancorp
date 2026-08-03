issue_title: "Architect & Implement Native Rust Omnichannel Chat System & Action Required Feed"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for an owner-operator.

  Additionally, OHC is retiring the external Chatwoot dependency and requires a lightning-fast, highly-scalable native Rust implementation for its omnichannel customer support & chat engine.

  # Research Report

  **Findings & Competitive Analysis:**

  - **Chatwoot Source Code Audit:** Checked out and analyzed `github.com/chatwoot/chatwoot`. Chatwoot uses a robust schema with entities such as `Account`, `Inbox`, `Channel::*` (FacebookPage, Whatsapp, TwilioSms, Email, WebWidget), `Contact`, `Conversation`, and `Message`. It leverages Redis and Sidekiq for async operations and ActionCable for WebSockets.
  - **Shopify / Wix Inbox:** Aggregate chat and email but rely heavily on manual responses or basic, rigid auto-replies. They do not proactively draft contextual responses based on full customer history across all channels.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Omnichannel Gateway - Rust)
      B --> C{Customer Identity Resolution Engine}
      C -->|Lookup / RLS| D[(Unified Customer Graph DB - PostgreSQL)]
      C --> E[Event Mesh / Redis Queue]
      E --> F[The Ambassador Agent / Triage Worker]
      F -->|Query Context| D
      F -->|Draft Reply| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> A
  ```

  ### Data Model & Invariants (Native Rust Migration)

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Channel : has
      Contact ||--o{ Conversation : has
      Inbox ||--o{ Conversation : tracks
      Conversation ||--o{ Message : contains

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      Channel {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string channel_type
          jsonb config
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type
          uuid sender_id
          text content
      }
  ```
  - **Multi-Tenancy:** Row Level Security (RLS) via `tenant_id` MUST be enforced on all tables.
  - **Identity:** `tenant_id` ensures zero cross-tenant leakage. SPIFFE/SPIRE for internal service auth.

  ### Mobile UX Flow (375px First)

  - **Home Feed (Mobile):** The top card in the feed shows "1 New Message from Sarah (Insta DM) - Action Required".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Premium macOS-style Translucent Glass materials. Blurred background to maintain focus, native keyboard integration if editing. Touch targets are at least 44x44px. No horizontal scrolling.

  ### AI Agent Integration Points

  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh (PostgreSQL `SKIP LOCKED` job queue). Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions

  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response before the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Contact` entity per tenant.
  - **Native Rust & Complete Chatwoot Retirement:** OHC does not rely on external Chatwoot services. The chat stack is implemented purely in Rust (`src/server/services/chat`).

  # Implementation Prompt

  **User-Facing Outcome:** As a business owner, when a customer messages me on WhatsApp or Instagram, I open the OHC app to find a pre-written, perfectly accurate response already drafted based on their purchase history. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. A simulated external message is ingested by the Native Rust Omnichannel Gateway webhook endpoints.
  2. The system correctly identifies or creates the `ChatContact` in the database, enforcing `tenant_id`.
  3. The message is persisted as a `ChatMessage` and routed to the Agent Triage worker queue.
  4. The Ambassador Agent queries the context and drafts a reply, creating an entry in the Action Required Queue.
  5. The frontend mobile UI (375px optimized, translucent glass layout) displays the drafted message card.
  6. The owner taps "Approve", and the system dispatches the message back to the mocked external channel.
  7. Provide complete 100% unit test coverage for the new Rust services and at least 5 Playwright E2E tests verifying the end-to-end CUJ. No mock data in the UI; all data must flow from the backend.
  8. `bazel test //...` must remain 100% green.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat, rust]
assignees: []
