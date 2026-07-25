issue_title: "[Platform] Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp currently has a dependency gap where it needs a high-performance, multi-tenant omnichannel customer support and chat engine, similar to Chatwoot, but running natively in Rust. The mandate dictates that Chatwoot as an external third-party service must be 100% retired, and its functionality must be replicated natively within our system (`onehumancorp/mono`) to maintain a single integrated experience. We need native capabilities for handling multiple communication channels (Web Widget, SMS, Email, WhatsApp, Instagram, etc.), routing, agent assignment, and SLA policies, but seamlessly built into OHC without external dependencies. Maya, Carlos, and Priya need to manage all their customer interactions natively within OHC without switching tools.

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`):
  - **Core Entities**: The system revolves around Accounts (Tenants), Contacts, Inboxes, Conversations, and Messages.
  - **Channel Adapters**: Inboxes connect to specific "Channels" (e.g., `Channel::WebWidget`, `Channel::Sms`, `Channel::Email`, `Channel::Whatsapp`, `Channel::Instagram`).
  - **Real-time Messaging**: It uses ActionCable (WebSockets) for real-time dispatch of messages to agents/clients.
  - **Background Processing**: Heavy reliance on background jobs (Sidekiq) for webhook processing, email parsing, automation rules, and campaign execution.
  - **Competitor Systems (Stripe, Intercom, Shopify Ping)**: These systems integrate the messaging directly into the merchant's operational context. We need our chat system to have direct access to OHC's Orders, Appointments, and Quotes without API overhead.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|| CHANNEL_ADAPTER : configured_with

      TENANT {
          uuid id
          string name
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone_number
          string identifier
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
          jsonb config
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          uuid assignee_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          uuid sender_id
          string sender_type
          text content
          string message_type
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox List**: The owner opens OHC and taps "Inbox". A clean, single-column list view (UniFi style) shows all recent conversations across all channels. Unread messages are bold with a translucent glass notification badge.
  2. **Conversation View**: Tapping a thread opens the chat. The bottom has a native mobile keyboard input. The top header shows the contact name and channel icon (e.g., SMS, IG).
  3. **Context Drawer**: Swiping left or tapping a header info button opens a translucent glass pane showing the contact's recent orders, upcoming bookings, and lifetime value natively from OHC.
  4. **Action Shortcuts**: Inline buttons to "Send Quote", "Request Payment", or "Create Booking" directly inside the chat flow.

  ### AI Agent Integration Points
  - **Customer Assistant Agent**: Listens to new `MESSAGE` events. If the message is unassigned, it uses tenant-scoped memory to draft a reply or automatically respond (e.g., for FAQs or simple requests like "vegan cakes?").
  - **Work Triage Agent**: Analyzes incoming messages to categorize urgency and suggest the next best action for the owner.
  - **Operations Agent**: Can extract structured data (like dates or order numbers) from unstructured chat to automatically propose creating a booking or updating an order.

  ### Key Design Decisions
  - **Native Rust**: Implemented as a set of Rust crates within `onehumancorp/mono`. This allows for extremely high throughput and low memory footprint compared to Ruby/ActionCable.
  - **Shared Postgres**: Uses the main OHC PostgreSQL database with strict Row-Level Security (RLS) on `tenant_id` for all tables (`inboxes`, `conversations`, `messages`, `contacts`).
  - **WebSocket Server**: A dedicated Rust Axum/Tokio WebSocket server for real-time bidirectional syncing, authenticated via SPIFFE/SPIRE and standard OHC JWTs.
  - **Agent Event Bus**: Instead of HTTP webhooks internally, the system emits events to our Redis/Postgres AI Job Queue so agents can react to messages instantly and reliably.

  ## Implementation Prompt
  **Goal**: Implement the foundational database schema, Rust domain models, and core API endpoints for the native OHC Omnichannel Chat System.

  **Persona**: Maya (Baker) wants to see Instagram DMs and Web Chat messages in a single unified inbox on her phone, without needing a separate Chatwoot app.

  **Acceptance Criteria**:
  1. Create the database migrations for `contacts`, `inboxes`, `conversations`, and `messages`, ensuring strict `tenant_id` multi-tenancy.
  2. Implement the Rust core domain models and repository traits for these entities.
  3. Create REST API endpoints (or gRPC services, following repo standards) to:
     - List all inboxes for a tenant.
     - List conversations for an inbox.
     - Fetch messages for a conversation.
     - Send a new message to a conversation.
  4. Build a basic mock channel adapter mechanism in Rust that can simulate receiving a message from an external source (like a Web Widget).
  5. Provide 100% unit test coverage for the new models and services.
  6. Add a Playwright E2E test verifying that a user can open the UI, view the unified inbox, and send a message.

  *Note: Do not build the specific third-party integrations (IG, WA) yet. Focus on the core native engine (Inboxes, Conversations, Messages) and the Web Widget channel first.*

  **Priority**: P0 (Critical)

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
