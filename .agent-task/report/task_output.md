issue_title: "Architectural Gap: Native Rust Unified Omnichannel Inbox Implementation"
issue_description: |
  # Problem Statement
  Small business owners and operators like Maya (Home Baker) and Fatima (Food Cart Operator) receive customer inquiries across fragmented external channels (Instagram, WhatsApp, SMS, Web Chat). While OHC has begun an `omnichannel_service.rs` implementation, it lacks the deep, extensible abstractions found in industry-standard unified inboxes like Chatwoot. Specifically, OHC misses full data models for Channels, Inboxes, Conversations, Messages, and Team routing, which limits the platform's ability to seamlessly transition conversations between AI agents (like The Ambassador) and human operators in real-time, especially on poor networks or 375px mobile viewports.

  # Research Report
  **Chatwoot Source Code Audit:**
  A deep dive into `https://github.com/chatwoot/chatwoot` reveals a robust, proven architecture for omnichannel messaging:
  - **Data Models:** Chatwoot uses `Account` (Tenant), `Contact`, `Inbox`, `Channel::*` (Adapters for Twitter, WhatsApp, API, etc.), `Conversation`, and `Message`.
  - **Event Mesh & Webhooks:** Every action triggers background jobs and webhooks, enabling real-time WebSocket updates to the frontend and SLA enforcement.
  - **Agent Routing:** Conversations can be assigned to teams, bots, or specific agents, with explicit state machines (`open`, `resolved`, `snoozed`).

  **OHC Current State vs. Gap:**
  OHC's `OmniChannelService` currently acts as a basic ingest mechanism (`WorkItem` and `CustomerProfile`), dumping raw payloads into a `PENDING` state. To truly act as an AI-first Assistant, OHC must natively replicate Chatwoot's conversation lifecycle in Rust, ensuring that an incoming Instagram DM creates a real-time `Message` in a `Conversation` tied to a specific `Contact` and `Inbox`, triggering the CS Agent.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph External Channels
          IG[Instagram Graph API]
          WA[WhatsApp Cloud API]
          SMS[Twilio SMS]
          Web[Web Chat Widget]
      end

      IG -->|Webhook| IGAdapter[Channel::Instagram]
      WA -->|Webhook| WAAdapter[Channel::WhatsApp]
      SMS -->|Webhook| SMSAdapter[Channel::Twilio]
      Web -->|WebSocket| WebAdapter[Channel::API]

      IGAdapter --> Router[Omnichannel Router Engine]
      WAAdapter --> Router
      SMSAdapter --> Router
      WebAdapter --> Router

      Router -->|Creates/Updates| DB[(Unified Ledger PostgreSQL)]
      DB --> Contact[Contact / Profile]
      DB --> Inbox[Inbox / Pipeline]
      DB --> Conversation[Conversation]
      DB --> Message[Message]

      Router --> EventMesh[Rust Event Mesh / Redis CRDT]
      EventMesh -->|Trigger| Agents[AI Swarm: The Ambassador]
      EventMesh -->|WebSocket| Mobile[OHC Mobile UI 375px]

      Agents -->|Drafts Reply| Conversation
  ```

  ### Data Model & Invariants (Multi-Tenant)
  - **Inbox:** Logical grouping of channels (e.g., "Support Inbox", "Sales Inbox"). Must have `tenant_id`.
  - **ChannelAdapter:** Rust traits to handle varying vendor payloads, normalizing them into OHC `Message` structs.
  - **Conversation:** Links a `Contact` to an `Inbox`. Tracks status (`pending_ai`, `needs_owner`, `resolved`).
  - **Message:** The actual payload (text, media). Supports `is_private` for internal notes/AI drafts.
  - **Multi-Tenancy Rule:** Row-Level Security (RLS) is strictly enforced on all new tables (`tenant_id`). Distributed locking (Redis Redlock) coordinates AI and human replies to prevent race conditions.

  ### Mobile UX Flow (375px First)
  - **Unified Feed:** The primary 375px viewport displays a Glassmorphism card list of `Conversations`.
  - **AI Drafting:** If a conversation is `pending_ai`, the UI reflects a shimmering loading state or a "Drafting..." indicator. Once The Ambassador drafts a reply, the state becomes `needs_owner`.
  - **Actionable Cards:** The owner taps the conversation card, reviews the AI-generated message alongside the `Contact`'s lifetime value and order history, and taps "Approve" (large 44x44px touch target). The action goes through the event mesh back to the `ChannelAdapter` and out to the customer.
  - **Offline/Flaky Network:** CRDTs ensure that if the owner approves a message while in a subway, it queues locally and syncs to the server upon reconnection, truthfully showing a "Pending Delivery" state.

  ### Zero Trust & Security
  - API and webhook endpoints must validate vendor signatures (e.g., Twilio `X-Twilio-Signature`, Meta App Secret).
  - Webhooks are processed asynchronously by workers to prevent DDoS vectors.

  # Implementation Prompt
  **User-Facing Outcome:** Business owners open their OHC app and see a unified, real-time feed of messages from WhatsApp, SMS, and Instagram. AI agents have already read the incoming messages and drafted contextual replies. The owner taps a single button to approve and send the replies back to the respective platforms.
  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Implement the Rust backend data models (`Contact`, `Inbox`, `Channel`, `Conversation`, `Message`) with strict PostgreSQL RLS (`tenant_id`).
  2. Implement a `ChannelAdapter` trait and at least one concrete adapter (e.g., Twilio SMS or Meta Webhook) to normalize incoming payloads.
  3. Plumb the ingestion path so that an incoming webhook creates a `Message` in the correct `Conversation` and triggers an event on the mesh.
  4. Build Playwright E2E tests: A test script fires a mock Twilio webhook. The backend parses it, creates the conversation, and triggers the AI draft. The UI (in a 375px mobile layout) displays the new conversation with the AI draft. A Playwright user clicks "Approve", verifying the outgoing API call is queued.
  5. Ensure 100% Rust unit test coverage for the new domain and service modules.
  Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
