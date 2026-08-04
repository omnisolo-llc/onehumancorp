issue_title: "Architecture: Implement Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**
  Currently, OneHumanCorp (OHC) lacks a native, high-performance, multi-tenant omnichannel customer support & chat engine. The goal is to completely replace external dependencies like Chatwoot with a bespoke, deeply integrated solution written in Rust within `onehumancorp/mono`. This system needs to empower personas like Maya (baker), Carlos (handyman), and Priya (boutique owner) to seamlessly manage inbound communications (Instagram DMs, SMS, WhatsApp, Web Widget) from a single, unified, mobile-first interface without configuring complex third-party tools.

  **Research Report**
  We audited the Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`) to understand the requirements for a robust omnichannel engine. Key discoveries include:
  - **Core Entities**: Accounts (Tenants), Inboxes (Channels), Conversations, Messages, Contacts, and Users/Agents.
  - **Channel Adapters**: Need support for Web Widget, SMS (Twilio), WhatsApp, Email, Instagram, Facebook Page, Telegram, Line, etc.
  - **Real-time Capabilities**: WebSockets are essential for live message delivery (PubSub architecture).
  - **Automation & Routing**: Macros, SLA policies, round-robin assignment, and canned responses.
  - **Multi-tenancy**: Row-level security or strict logical isolation by `tenant_id` (Account ID) is non-negotiable.

  By implementing this natively in Rust, OHC can achieve superior performance, tighter integration with the AI Assistant (for auto-drafting replies, categorization), and simplified infrastructure (no separate Ruby on Rails/Sidekiq/Redis stack to manage for chat).

  **Design Doc**

  *Architecture Overview:*
  The new Chat system will be built as a set of Rust crates/microservices within the OHC monorepo, communicating via gRPC with other services and providing REST/GraphQL/WebSocket endpoints for the frontend.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ USER : employs
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : has
      USER ||--o{ MESSAGE : sends
      CHANNEL_ADAPTER ||--|| INBOX : implements
  ```

  *Components:*
  1.  **Core Data Models (Rust + PostgreSQL):**
      - `Tenant` (already exists, need to ensure isolation).
      - `Inbox`: Configuration for a specific channel (e.g., "Support Email", "Main Web Widget", "Carlos' SMS").
      - `ChannelAdapter`: Trait/Interface for specific integrations (Twilio, Meta Graph API, Web).
      - `Contact`: The external customer interacting with the business.
      - `Conversation`: A threaded context linking a Contact, an Inbox, and Messages. Includes state (Open, Snoozed, Resolved).
      - `Message`: Individual payloads (Text, Images, Audio, Attachments) with delivery status.
  2.  **Real-Time Engine (Rust + WebSockets):**
      - A high-performance WebSocket server to push new messages and conversation updates to the owner's active sessions instantly.
  3.  **AI Coordination (KAIROS integration):**
      - Integration with the KAIROS AI Job Queue to automatically categorize incoming conversations, draft suggested replies, and update Contact context based on message content.

  *Mobile UX Flow (375px First):*
  - **Unified Inbox View**: A single feed showing active conversations across all channels, sorted by priority/urgency.
  - **Conversation View**: Clean chat interface, clearly indicating the source channel (e.g., an Instagram icon next to the customer name).
  - **AI Assistant Drawer**: A non-intrusive pull-up drawer or inline suggestion block offering drafted replies based on business knowledge.
  - **Offline/Flaky Network Tolerance**: Messages sent while offline are queued locally and synchronized automatically when connectivity is restored, with clear visual indicators (e.g., a "sending" spinner or "pending" icon).

  *Key Design Decisions:*
  - **Strict Multi-Tenancy**: Every database query MUST filter by `tenant_id` to prevent data leakage.
  - **Idempotency**: All webhook processors (e.g., receiving an SMS from Twilio) must use idempotency keys to handle retries gracefully without duplicating messages.
  - **Zero Trust**: SPIFFE/SPIRE for service-to-service authentication.

  **Implementation Prompt**
  Implement the core Rust backend services for the native Omnichannel Chat system.
  - **CUJ**: A business owner (e.g., Carlos) receives a new SMS inquiry. The system must ingest the webhook, create a Contact (if new), start a Conversation in the designated SMS Inbox, and store the Message. The owner then views this Conversation in the mobile-first Web UI and sends a reply, which is routed back out via the appropriate Channel Adapter.
  - **Acceptance Criteria**:
    - Rust crates defined for `inbox`, `conversation`, `message`, and `contact` management.
    - Basic Postgres schema defined with `tenant_id` isolation.
    - A mock or basic implementation of at least one Channel Adapter (e.g., Web Widget or SMS).
    - WebSockets infrastructure stubbed or implemented for real-time updates.
    - 100% unit test coverage for new Rust code.
    - 5+ Playwright E2E tests simulating the ingestion and reply flow via the UI.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
