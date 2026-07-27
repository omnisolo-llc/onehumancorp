issue_title: "Implement Native Rust Omnichannel Chat Engine (Chatwoot Parity)"
issue_description: |
  **Title**: Implement Native Rust Omnichannel Chat Engine (Chatwoot Parity)

  **Problem Statement**:
  OHC currently lacks a unified, multi-tenant omnichannel communication platform natively built in Rust. Following the retirement of Chatwoot as an external service/dependency, OHC needs a powerful, native replacement to handle real-time customer conversations across various channels (web widget, SMS, Email, Instagram, WhatsApp, Facebook, Twilio). Non-technical owners (like Maya the Baker or Carlos the Handyman) need a seamless "Work Triage" and "Customer Inbox" experience that unifies all demand sources into a single feed without managing multiple chat platforms or paying for external SAAS integrations. The system must support agent/bot handoffs, macros, and SLA policies while maintaining strict PostgreSQL row-level multi-tenant isolation.

  **Research Report**:
  - **Source Context**: As part of the OHC Engineering Standards, the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) was cloned and audited to benchmark its core models, API structures, and architecture.
  - **Key Capabilities Discovered**:
    - Omnichannel adapters: Models exist for Facebook Page, Web Widget, Instagram, Telegram, TikTok, SMS, Line, Twitter Profile, WhatsApp, and Twilio SMS (`app/models/channel/*`).
    - Core entities: `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `AgentBot`, `Macro`, and `SlaPolicy`.
    - Message dispatch: ActionCable WebSockets push real-time updates to connected clients (agents/owners).
    - Event orchestration: Robust background jobs for notifications, webhook dispatch, and automated assignment.
  - **Competitive Analysis**: Shopify Inbox, Wix Inbox, and Stripe's communication tools all embed chat natively. None require third-party integrations for core business-to-customer messaging. OHC must offer this built-in.

  **Design Doc**:

  **1. Architecture Design (Mermaid.js)**
  ```mermaid
  graph TD
      Client[Customer Web/Mobile] -->|WebSocket/REST| Gateway[OHC Rust API Gateway]
      Gateway --> Auth[SPIFFE/SPIRE Auth & Tenant Isolation]
      Auth --> ChatSvc[Native Rust Chat Engine]

      ChatSvc -->|Publish| Redis[Redis Pub/Sub & Caching]
      Redis -->|Subscribe| WSManager[WebSocket Connection Manager]
      WSManager -->|Push| OwnerApp[Owner Dashboard / Mobile App]

      ChatSvc -->|Read/Write| Postgres[(PostgreSQL DB)]

      subgraph Channel Adapters
          ChatSvc --> Email[Email Adapter]
          ChatSvc --> WhatsApp[WhatsApp Adapter]
          ChatSvc --> Insta[Instagram Adapter]
          ChatSvc --> Twilio[Twilio SMS Adapter]
      end

      subgraph Background Processing
          DBQueue[Postgres SKIP LOCKED Queue] --> Workers[Rust Async Workers]
          Workers --> |Triggers| AI[AI Customer Assistant]
          Workers --> |Evaluates| SLAPolicies[SLA Policies & Macros]
      end
  ```

  **2. Mobile UX Flow (375px First)**
  - **Inbox List**: A clean, unified list of conversations spanning all channels. Channels are indicated by small, subtle icons. Unread indicators and SLA breach warnings (e.g., "Overdue") use OHC Premium Tokens (red/orange text, translucent badges).
  - **Conversation Thread**: Native mobile chat feel. Incoming customer messages on the left, outgoing owner/AI messages on the right.
  - **AI Copilot Integration**: A persistent "AI Draft" button floating near the text input. Pressing it triggers the AI to suggest a reply based on conversation history and store context.
  - **Quick Actions**: Swiping left on an inbox row reveals "Snooze" and "Resolve". Swiping right reveals "Mark Unread".
  - **Design Language**: Translucent Glass materials for headers/nav bars, maintaining the clean Ubiquiti/Apple style.

  **3. AI Agent Integration Points**
  - **Triage Agent**: Listens to incoming messages via the internal job queue. Automatically assigns priority and categorizes the intent (e.g., "Support", "Sales Inquiry").
  - **Customer Assistant Agent**: Suggests reply drafts. When a customer initiates contact, the agent can auto-respond with a customized greeting or knowledge-base article if configured.
  - **Operations Agent**: Can extract structured data (like appointment times or product requests) from unstructured chat text to generate internal tasks or bookings.

  **4. Key Design Decisions**
  - **Native Rust**: Rebuilding Chatwoot functionality in Rust ensures memory safety, high performance (critical for real-time WebSockets), and tight integration with OHC's auth/tenant systems.
  - **PostgreSQL Row-Level Security**: Every chat table (`messages`, `conversations`, `inboxes`) must have `tenant_id` and strict RLS policies to guarantee tenant isolation.
  - **Asynchronous WebSockets**: A dedicated Axum WebSocket handler backed by Redis Pub/Sub will manage real-time presence and message delivery to owners.

  **Implementation Prompt**:
  *Objective*: Implement the core database schema, domain logic, and a REST/WebSocket API layer in Rust for the new Native Omnichannel Chat Engine inside `onehumancorp/mono`.
  *CUJ*: An owner (e.g., Maya the Baker) opens her OHC app on her phone. She sees a new message in her unified inbox from a customer asking about vegan cakes (which came in via the web widget). She opens the thread, the WebSocket connection marks her as "online", she clicks "AI Draft" to generate a polite response confirming vegan options, and she sends it. The customer immediately receives the reply via the web widget.
  *Acceptance Criteria*:
  1. Define Rust structs, Protobuf definitions (if using gRPC internally), and PostgreSQL migration files for `Tenant`, `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` entities, ensuring `tenant_id` isolation.
  2. Implement an Axum WebSocket route that allows clients to subscribe to conversation updates.
  3. Create standard CRUD endpoints for Inboxes and Messages.
  4. Build a basic adapter trait/interface for channels, with an initial implementation for the "Web Widget" channel.
  5. Include full Playwright E2E tests verifying the real-time send/receive flow in the browser UI.

  **Priority**: P0 (critical)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []