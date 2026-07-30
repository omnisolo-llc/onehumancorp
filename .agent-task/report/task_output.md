issue_title: "Architecture: Native Rust Multi-Tenant Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  ### Title
  Architecture: Native Rust Multi-Tenant Omnichannel Inbox (Chatwoot Replacement)

  ### Problem Statement
  OHC is an assistant-first work OS for owners and operators. Previously, omnichannel messaging relied on external integrations like Chatwoot. Chatwoot is now 100% RETIRED as a dependency. OHC requires a native, high-performance, multi-tenant Rust-based omnichannel chat engine. The owner (e.g., Maya the Baker, Carlos the Handyman) needs a unified inbox on their 375px mobile screen to manage Instagram DMs, WhatsApp messages, emails, and web inquiries without switching apps. AI agents need native access to this message stream to draft replies, understand context, and turn conversations into actionable business objects like quotes, tasks, and bookings.

  ### Research Report
  Based on an exhaustive source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`), their architecture centers around the following core entities mapping to a unified inbox:
  - `Account` (Corresponds to OHC's `Tenant`)
  - `Inbox` (Logical grouping of channels)
  - `Channel::*` (Adapters for API, Email, Facebook, Instagram, WhatsApp, Twilio SMS, Web Widget, etc.)
  - `Contact` (The customer/user interacting)
  - `Conversation` (A threaded communication session)
  - `Message` (Individual message payload)
  - `AgentBot`, `AutomationRule`, `CannedResponse`

  **Competitor Analysis**:
  - *Shopify Inbox*: Heavily optimized for e-commerce, tightly coupled to cart context.
  - *Wix Inbox*: Integrated with basic CRM but lacks deep AI automation.
  - *Zendesk/Intercom*: Too heavy, requires extensive administration, and violates OHC's "Radical Simplicity" core value.

  OHC's native implementation must be simpler for the end user but more powerful under the hood, natively integrated with PostgreSQL Row-Level Security (RLS) for tenant isolation, Redis for pub/sub WebSocket messaging, and the AI Job Queue for asynchronous agent analysis and drafting.

  ### Design Doc

  **1. Architecture Diagram**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : manages
      TENANT ||--o{ CONTACT : serves
      INBOX ||--o{ CHANNEL_CONFIG : has
      CHANNEL_CONFIG ||--o{ CONVERSATION : sources
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ AI_DRAFT : triggers
      MESSAGE }|--|| CHANNEL_ADAPTER : routed_via
  ```

  **2. Key Design Decisions**
  - **Zero-Trust Multi-Tenancy**: All tables (`inboxes`, `channels`, `contacts`, `conversations`, `messages`) MUST enforce Row-Level Security (`tenant_id`) in PostgreSQL.
  - **Trait-Based Channel Adapters**: Rust `ChannelAdapter` trait to standardize `send_message`, `receive_webhook`, and `get_metadata` across WebWidget, Email, and WhatsApp.
  - **AI Hook Integration**: Messages inserted into the DB will emit a Redis pub/sub event that triggers the "Work Triage" AI agent via the PostgreSQL `SKIP LOCKED` job queue.
  - **Real-Time WebSocket**: Rust Tokio-based WebSocket server authenticating via SPIFFE/SPIRE identity, subscribing to Redis channels for real-time mobile push.

  **3. Mobile UX Flow (375px First)**
  1. The owner opens the OHC app (375px mobile viewport).
  2. Taps the "Unified Inbox" icon in the bottom navigation.
  3. Views a single scrollable feed of active conversations, with small translucent glass badges indicating the source (WhatsApp, IG, Web).
  4. Taps a conversation. The chat UI uses native keyboard handling, spacious 44x44px touch targets for attachments, and UniFi-style clean typography.
  5. The AI "Customer Assistant" provides a suggested draft reply inline, which the owner can tap to approve or edit.
  6. An action menu allows the owner to instantly convert the thread into a "New Quote" or "Booking".

  **4. AI Agent Integration Points**
  - **Work Triage**: Subscribes to new conversation events, prioritizes the inbox feed, and tags the intent (e.g., "Pricing Inquiry", "Support").
  - **Customer Assistant**: Reads conversation history and `tenant` context to generate `AI_DRAFT` messages.
  - **Sales/Operations Assistant**: Analyzes text for dates and services to auto-populate Quote or Booking UI forms when the owner hits the action menu.

  ### Implementation Prompt
  "Implement the foundational Rust data models, database migrations, and REST/gRPC APIs for the native omnichannel inbox.
  1. Create PostgreSQL migrations with RLS (`tenant_id`) for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.
  2. Implement the corresponding Rust domain models in `src/server/ohc/domain/inbox` utilizing Serde and SQLx.
  3. Define the `ChannelAdapter` trait and implement a basic `WebWidget` channel adapter.
  4. Create the API endpoints to list, read, and send messages for a conversation.
  5. Build the 375px Mobile Inbox UI (Flutter or PWA/Playwright tested) that displays the unified feed using macOS-style Translucent Glass and UniFi layout patterns.
  6. **Mandatory**: Use NO mocks in the UI; data must flow from the real PostgreSQL database. Include at least 5 Playwright E2E tests validating the end-to-end CUJ of receiving a message, viewing it in the UI, and sending a reply. Load relevant `superpowers` skills before starting."

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
