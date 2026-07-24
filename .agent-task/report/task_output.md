issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Mission Queue Protocol Report**

  ### 1. Problem Statement
  OneHumanCorp (OHC) is retiring Chatwoot as an external dependency and moving towards a custom, native Rust omnichannel chat system. Our non-technical owner/operators (e.g., Maya the baker, Carlos the handyman) need a unified inbox to manage customer communications across different channels (Email, SMS, Web Widget, WhatsApp, Instagram, etc.). Currently, OHC lacks this native capability.

  ### 2. Research Report
  - **Goal**: Architect a native Rust omnichannel customer support & chat engine in `onehumancorp/mono` that achieves 100% feature parity with Chatwoot.
  - **Competitor/Benchmark Analysis**:
    - **Chatwoot**: Its architecture uses Ruby on Rails with PostgreSQL and Redis. Its core entities include Account (tenant), Contact, Inbox, Channel (adapter), Conversation, and Message.
    - **Shopify / Wix**: Both platforms offer an integrated "Inbox" or "Communications" tab. They consolidate store chats, emails, and order inquiries into a single view for the operator. The focus is always on associating the conversation with the business context (orders, items in cart) rather than just being a disconnected chat tool. OHC's implementation must similarly integrate deeply with the owner's operations (bookings, invoices).
  - **Proposed Solution**: Build a high-performance, multi-tenant Rust backend mirroring these core concepts, utilizing PostgreSQL with row-level security (`tenant_id`) and Redis for real-time WebSocket events.

  ### 3. Design Doc
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
        ACCOUNT ||--o{ INBOX : manages
        ACCOUNT ||--o{ CONTACT : has
        ACCOUNT ||--o{ CONVERSATION : tracks
        INBOX ||--o{ CONVERSATION : contains
        CONTACT ||--o{ CONVERSATION : participates_in
        CONVERSATION ||--o{ MESSAGE : contains
        INBOX }|--|| CHANNEL_ADAPTER : uses
    ```
  - **Mobile UX Flow (375px first)**:
    - **Inbox List View**: A scrollable list of conversations. Each list item shows the contact name, avatar, last message snippet, time, and an unread indicator. Minimal padding, highly readable typography.
    - **Conversation View**: Tapping a conversation opens the chat interface. Sticky header with contact info and a "Back" button. Scrollable message history. Sticky bottom input bar with text area, attachment icon, and send button. Native mobile keyboard support.
    - **Channel Setup**: Hidden behind "Advanced Settings". Simple forms to connect channels (e.g., inputting an API key or clicking an OAuth button).
  - **AI Agent Integration Points**:
    - **Customer Assistant**: Listens to new `MESSAGE` events. If a draft reply is needed, the AI agent uses the `CONVERSATION` context to generate a draft message and saves it with a "draft" status or specific `message_type` for the owner to approve.
    - **Work Triage**: Analyzes incoming conversations to determine priority and categorize them as leads, support, or spam.
  - **Key Design Decisions**:
    - **Database**: PostgreSQL with strict Row-Level Security (RLS) on `account_id` (tenant_id) for all entities.
    - **Real-time**: Redis Pub/Sub for routing message events to active WebSocket connections.
    - **API Layer**: gRPC internally, REST/JSON externally for clients.

  ### 4. Implementation Prompt
  Implement the core database schema, Rust backend services, and a basic frontend UI for the native omnichannel chat system.
  - **Backend (Rust)**: Define the core entities (Inbox, Contact, Conversation, Message) with strict multi-tenant isolation. Implement the necessary service endpoints and a WebSocket gateway for real-time message delivery. Include basic channel adapter traits.
  - **Frontend (Flutter/PWA)**: Create the mobile-first (375px) Inbox list and Conversation view. Use the OHC Premium Token library (macOS-style Translucent Glass, UniFi-style layouts). Ensure it's fully functional on mobile without horizontal scrolling.
  - **Acceptance Criteria**: A user can view a list of conversations, open a conversation, send a message, and receive a message in real-time via WebSockets. All data must be properly scoped to the current tenant. The UI must feel premium and pass the "grandmother test". 100% unit test coverage required. 5+ E2E Playwright tests required.
  - **Estimated Scope**: Large
  - **Priority**: P0

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
