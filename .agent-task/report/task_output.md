issue_title: "Implement Native Rust Omnichannel Chat System Replacements for Chatwoot"
issue_description: |
  **Problem Statement**:
  OHC relies on Chatwoot as an external third-party service for omnichannel communication. However, as part of the new "Chatwoot Retirement" mandate, we are removing this external dependency. OHC requires a high-performance, native Rust omnichannel chat system inside `onehumancorp/mono` that achieves 100% feature parity with Chatwoot, ensuring tenant isolation (RLS), tight integration with OHC triage agents, and robust mobile/desktop UX for business owners.

  **Research Report**:
  We cloned and audited the Chatwoot Ruby on Rails source code (https://github.com/chatwoot/chatwoot) to understand its architecture. Key observations:
  - **Data Models**: Chatwoot relies heavily on `Conversation`, `Message`, `Contact`, `Inbox`, and `Account` (which maps to our `Tenant`).
  - **Channels**: Chatwoot supports diverse adapters (`whatsapp`, `web_widget`, `facebook_page`, `twitter_profile`, `email`, `sms`, etc.). For MVP, OHC needs to replicate the `whatsapp` (Meta Webhooks) and `web_widget` (WebSocket) channels.
  - **Real-time Messaging**: Uses ActionCable (WebSockets). Our Rust implementation will likely need a high-performance WebSocket server (e.g., using `tokio`/`axum` or `actix-web`) and Redis Pub/Sub for cross-node event distribution.
  - **Automation & Routing**: Chatwoot has `agent_bot`, `automation_rule`, `macro`, and `inbox_assignment_policy`. OHC's unique value prop is that AI agents will seamlessly intercept and handle unassigned messages (Agentic Negotiator & Booker), creating quotes and booking appointments.

  **Design Doc**:
  - **Architecture Diagram (Mermaid)**:
    ```mermaid
    graph TD
        Client[Web Widget / Meta Webhook] --> API Gateway
        API Gateway --> RustChatEngine[Rust Chat Service]
        RustChatEngine --> Redis[Redis Pub/Sub & Caching]
        RustChatEngine --> DB[(PostgreSQL with RLS)]
        RustChatEngine --> AI_Agent[Agentic Triage & Response]
        AI_Agent --> OwnerFeed[Owner Feed UI]
    ```
  - **Data Models**:
    - `Inbox`: Represents a channel endpoint (e.g., specific WhatsApp number or Website domain). Includes `tenant_id`.
    - `Contact`: Represents the customer interacting. Includes `tenant_id`.
    - `Conversation`: Links `Contact` and `Inbox`. Tracks status (open, snoozed, resolved, assigned_to_agent).
    - `Message`: Contains content, `message_type` (incoming, outgoing, template), `content_type` (text, image, audio), and status (sent, delivered, read).
    - `ChannelAdapter`: Polymorphic configuration for channel-specific secrets (e.g., WhatsApp API key).
  - **Mobile UX Flow (375px first)**:
    1. Owner opens OHC app on phone.
    2. "Work Triage" tab aggregates all incoming messages across channels.
    3. Conversations handled by AI have a distinct translucent glass "Handled by AI" chip.
    4. Tapping a conversation opens the unified message timeline, supporting rich payloads (images, quotes, payment requests).
    5. Bottom input bar supports native mobile keyboards and quick-action macros.
  - **AI Agent Integration**:
    - When a new `Message` is created where `message_type == incoming` and `status == unassigned`, an event is pushed to the Redis AI Job Queue.
    - AI Agent reads the conversation history, formulates a response or generates a draft Quote/Booking, and uses the internal gRPC/REST API to insert a `Message` (either directly sent or drafted for owner approval).

  **Implementation Prompt**:
  Implement the backend core data models and service layer for the native Rust Chat System.
  1. Define the SQL schema and Rust entities for `inbox`, `contact`, `conversation`, and `message` using our PostgreSQL DB with Row Level Security (RLS) on `tenant_id`.
  2. Implement the `WhatsApp` channel adapter: Create the webhook endpoint to receive Meta's API payload, parse it, create the corresponding `Contact` and `Message`, and respond to Meta. Implement the outbound sending logic via Meta's Graph API.
  3. Implement the `Web Widget` channel: Create a WebSocket endpoint using `axum`/`tokio` that allows a client to connect, authenticate (using a guest token), and exchange real-time messages.
  4. Ensure all new messages publish an event to Redis to trigger the AI Triage Agent.
  Acceptance criteria: Unit tests (100% coverage) for all models, webhook parsing, and WebSocket handling. Integration tests for the full message lifecycle (receive -> DB -> event publish -> send reply). Ensure strict tenant isolation. No UI work required in this ticket.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, backend]
assignees: []