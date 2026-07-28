issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  Small business owners and operators (Maya the baker, Carlos the handyman, Priya the boutique owner) receive customer inquiries across a multitude of channels: Instagram DMs, WhatsApp, Facebook Messenger, SMS, and Email. Currently, this results in fragmented communication, lost context, and delayed responses. The previous direction was to rely on Chatwoot, but as per the new architectural mandate, external Chatwoot dependencies are 100% RETIRED. OHC must implement a native, high-performance, multi-tenant omnichannel chat engine in Rust to seamlessly integrate with the broader AI Work Assistant ecosystem (like The Ambassador agent).

  # Research Report
  **Findings & Chatwoot Source Code Audit:**
  - **Chatwoot's Approach**: Chatwoot handles omnichannel via polymorphic `channels` (e.g., `channel_whatsapp`, `channel_email`, `channel_instagram`) that tie back to an `Inbox`.
  - **Entities**: The core graph revolves around `Account` (tenant), `Inbox`, `Contact`, `ContactInbox` (joining a contact's channel-specific identifier to an inbox), `Conversation`, and `Message`.
  - **Performance/Scale Constraints in Chatwoot**: Ruby on Rails + PostgreSQL with deep polymorphic associations. For OHC, building this in Rust allows significantly higher throughput for webhooks, tighter integration into the event mesh, and lower memory overhead for thousands of concurrent WebSocket connections for real-time mobile updates.
  - **OHC Opportunity**: By building this natively in Rust, we can embed our Zero-Trust multi-tenancy model directly at the protocol level. We can tightly couple the `Contact` model with our AI context graph, allowing "The Ambassador" (Customer Success Agent) to draft replies instantaneously via event-driven hooks without network hops to an external CRM.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONTACT_INBOX : has
      INBOX ||--o{ CONTACT_INBOX : contains
      CONTACT_INBOX ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
      }
      CONTACT_INBOX {
          uuid id PK
          uuid contact_id FK
          uuid inbox_id FK
          string source_id "e.g. IG handle"
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_inbox_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          uuid tenant_id FK
          string content
          string message_type "incoming/outgoing"
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed (Home Screen)**: Owner sees a prominent card: "New Message: Sarah (Instagram)". The card uses premium Translucent Glass styling.
  - **Unified Inbox View**: Tapping the card transitions to a full-screen chat interface.
    - **Header**: Shows contact name, channel icon (e.g., Instagram), and a back button.
    - **Context Panel**: A collapsible top drawer showing AI-summarized context (e.g., "Sarah ordered a vegan cake on Jan 14th").
    - **Chat History**: Scrollable message list.
    - **Composer**: An AI-drafted reply sits in the composer text area. The owner can tap "Send" instantly, or tap the text to invoke the native mobile keyboard for edits.
  - **Empty State**: If no messages, the inbox shows a beautiful, truthful empty state ("No pending messages. You're all caught up!"). No fake mock messages.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent)**: Subscribes to the event mesh for `MessageCreated` events where `message_type == 'incoming'`.
  - **Action**: It queries the native Rust CRM, retrieves the `Contact` history and the `Tenant` catalog, generates a contextual reply, and inserts a `Message` with status `draft`.
  - **Notification**: The system pushes the draft to the mobile app via WebSocket, updating the Work Triage feed.

  ### Key Design Decisions
  - **Native Rust Impl**: Replaces the Chatwoot Ruby backend entirely. Services will be written in Rust, leveraging Axum for API/Webhooks and Tokio for async event handling.
  - **Multi-Tenant Isolation**: Every table (`inboxes`, `conversations`, `messages`, `contacts`) MUST have a `tenant_id` column. PostgreSQL Row-Level Security (RLS) policies will enforce isolation.
  - **Zero-Mock Policy**: The UI must bind to real gRPC/REST endpoints backed by this Rust service.

  # Implementation Prompt
  **User-Facing Outcome:**
  As an owner (e.g., Maya), I can connect my Instagram and WhatsApp to OHC. When a customer messages me, the message appears instantly in my OHC mobile app feed. The Ambassador agent has already read the message, checked the customer's history, and drafted a perfect reply. I tap "Send Draft" and the response goes back out to Instagram. I never leave the app or type a word unless I want to.

  **CUJ & Acceptance Criteria:**
  1. Define the PostgreSQL schema for the entities: `inboxes`, `channels`, `contacts`, `contact_inboxes`, `conversations`, and `messages`, strictly enforcing `tenant_id` and RLS.
  2. Implement the native Rust service with Axum to handle CRUD operations for these entities and expose Webhook ingress points for external channels (simulated initially).
  3. Implement the WebSocket or SSE push mechanism to notify the frontend of new incoming messages and AI drafts.
  4. Build the Flutter/Web UI (starting at 375px) for the Unified Inbox view following the Translucent Glass design tokens.
  5. The UI must NOT contain mock data; it must fetch real data from the Rust backend.
  6. Provide Playwright E2E tests:
     - A script injects an incoming message via the webhook ingress.
     - The owner logs into the web UI (mobile viewport).
     - The owner sees the message, views the AI draft (or writes a reply), and clicks send.
     - The system records the outgoing message in the database.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []