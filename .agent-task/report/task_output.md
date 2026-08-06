issue_title: "Implement Native Rust Omnichannel Chat System & Inbox"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Chat System

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and web forms. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox, or legacy Chatwoot) simply aggregate messages without context and require manual typing. The current implementation relies on a retired third-party service (Chatwoot). OHC needs a high-performance, native Rust omnichannel chat system that supports unified multi-channel triage, strict multi-tenant isolation, and proactive AI drafting ("The Ambassador" agent).

  ## Research Report
  - **Competitor Audit (Chatwoot, Shopify Inbox, Zendesk):**
    - Legacy Chatwoot provided robust data models (`Conversation`, `Message`, `Contact`, `Inbox`, `Channel`) and omnichannel webhook ingestion. However, it's an external dependency that doesn't fit OHC's Zero-Trust, native Rust microservice architecture.
    - Shopify Inbox and Wix aggregate messages but lack proactive, autonomous AI agent capabilities to negotiate or draft personalized responses natively.
  - **OHC Architecture Gap:** OHC requires a replacement for Chatwoot built natively in Rust (`onehumancorp/mono`), incorporating the best of Chatwoot's omnichannel data modeling while enforcing our strict `tenant_id` Row Level Security (RLS) in PostgreSQL.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> InboxUI[Unified Inbox UI];
      end

      App -- "gRPC / REST" --> RustServer[Native Rust OHC Server];

      RustServer --> WebhookGateway[Omnichannel Webhook Gateway];
      RustServer --> IdentityResolver[Customer Identity Resolution Engine];
      RustServer --> DB[(PostgreSQL with RLS)];

      subgraph External Channels
          WebhookGateway <--> Instagram[Instagram Graph API];
          WebhookGateway <--> WhatsApp[Meta WhatsApp Cloud API];
          WebhookGateway <--> Twilio[Twilio SMS];
          WebhookGateway <--> WebWidget[OHC Native Web Chat];
      end

      RustServer --> EventMesh[Redis Event Mesh / Go Channels];
      EventMesh --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> AmbassadorAgent[The Ambassador: RAG Context & Draft Reply];
          Agents --> OpsAgent[Operations: Check Inventory/Calendar];
      end
  ```

  ### Data Model & Invariants
  The native Rust implementation must replicate and optimize the core Chatwoot models, ensuring every table has a `tenant_id`:
  - `Contact`: Unifies customer identities across channels (e.g., matching a phone number to an email).
  - `Conversation`: Links a `Contact` to a specific channel/inbox.
  - `Message`: Immutable records of communication, supporting rich media attachments.
  - `ChannelAdapter`: Polymorphic configurations for Instagram, WhatsApp, Twilio, etc.
  - **Invariants:** 100% Row Level Security (RLS) enforced via PostgreSQL on `tenant_id`. All external webhooks must be scrubbed and routed to isolated tenant queues.

  ### Mobile UX Flow (375px First)
  1. **The Inbox View:** The owner opens the OHC mobile app to the "Inbox" tab. A clean, Glassmorphism-styled list of active conversations is displayed.
  2. **AI Action Cards:** Threads where "The Ambassador" has drafted a reply (e.g., confirming vegan cake availability based on live inventory) are pinned at the top with a "Needs Action" badge.
  3. **Interaction:** Tapping the thread shows the conversation history and the AI's drafted response at the bottom.
  4. **One-Tap Execution:** The owner simply taps "Approve & Send" or "Edit" to modify the draft using a native mobile keyboard.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Subscribes to new message events on the Event Mesh. It uses RAG to query the business's knowledge base, past orders, and live inventory to draft accurate, personalized replies.
  - **Zero-Touch Fallback:** If AI confidence is low, the conversation is marked for manual human review, with the AI providing suggested context points (e.g., "This customer's last order was #1234").

  ## Implementation Prompt
  Implement the Native Rust Omnichannel Chat backend and database schema inside `onehumancorp/mono`.
  - **Outcome:** The system must ingest webhooks from simulated external channels (Instagram, WhatsApp), resolve customer identities, map them to `Conversation` and `Message` entities, and trigger "The Ambassador" agent to draft a reply.
  - **CUJ:** A simulated webhook representing an Instagram DM arrives. The Rust backend processes it, saves it to the DB (respecting RLS), and triggers the AI agent. The AI agent drafts a reply. In the mobile UI (Playwright E2E test), the business owner sees the drafted reply on a 375px viewport and clicks "Approve & Send", which marks the draft as sent.
  - **Acceptance Criteria:**
    - PostgreSQL migrations for `contacts`, `conversations`, `messages`, and `channel_adapters` with `tenant_id` and RLS enabled.
    - Rust HTTP/gRPC endpoints for message ingestion and retrieval.
    - E2E Playwright test simulating the webhook, verifying the UI state, and approving the draft.
    - 100% test coverage for the new Rust module.
    - Complete removal/deprecation of any remaining external Chatwoot dependencies.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []