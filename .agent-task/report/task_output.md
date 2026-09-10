issue_title: "Architecture Deep Dive: Omnichannel Customer Support & Communication Engine"
issue_description: |
  **Title**: Universal Omnichannel Customer Support & Chat Engine

  **Problem Statement**:
  Business owners like Maya, Carlos, Priya, Leo, and Fatima struggle with fragmented customer communications. They receive inquiries across Instagram DMs, WhatsApp, web chat, email, and SMS. Currently, tracking these messages and responding contextually requires logging into multiple platforms, losing the overarching view of a customer's journey and order history. They need a single, unified inbox that acts as a command center for all customer interactions, with AI agents capable of drafting responses, summarizing context, and triggering workflows (like bookings or quotes) directly from the conversation.

  **Research Report**:
  - **Market Context**: Platforms like Intercom, Zendesk, and Shopify Inbox provide unified messaging, but are often too complex or segmented for single-operator or small-team businesses.
  - **Competitive Analysis**:
    - *Shopify Inbox*: Good e-commerce integration, but lacks broader service/booking support.
    - *Tencent Workbuddy / WeCom*: Excellent at unifying internal and external comms, strong inspiration for OHC's "command center" feel.
    - *Lark/Feishu*: High extensibility, but learning curve is steep.
  - **Identified Gap**: OHC currently lacks a native, high-performance omnichannel communication engine that seamlessly integrates with our AI harness (to auto-draft replies, extract intent, and manage state) while maintaining strict multi-tenant isolation.
  - **Proposed Solution**: Build a unified Omnichannel Chat Engine natively in Rust, providing a central `Conversation` and `Message` model, real-time WebSocket/SSE delivery, and channel adapters for external networks.

  **Design Doc**:
  - **Architecture**:
    ```mermaid
    erDiagram
      TENANT ||--o{ CONVERSATION : owns
      CUSTOMER ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL_ADAPTER ||--o{ CONVERSATION : routes
      MESSAGE }o--|| AI_INTENT : triggers

      TENANT {
        uuid id PK
        string name
      }
      CUSTOMER {
        uuid id PK
        uuid tenant_id FK
        string unified_profile_id
      }
      CONVERSATION {
        uuid id PK
        uuid tenant_id FK
        uuid customer_id FK
        string channel_type
        string status
      }
      MESSAGE {
        uuid id PK
        uuid conversation_id FK
        string sender_type
        text content
        timestamp created_at
      }
      CHANNEL_ADAPTER {
        string id PK
        string protocol
      }
    ```
  - **Mobile UX Flow (375px)**:
    1. **Unified Inbox View**: A simple list view of active conversations, tagged by channel (e.g., IG, WhatsApp, Web). Unread indicators are prominent.
    2. **Conversation Thread**: Tapping a thread opens a chat interface. The top bar shows customer context (e.g., "Active Order #123").
    3. **AI Assistant Overlay**: A persistent but unobtrusive bottom sheet or floating button offers "Draft Reply", "Summarize Context", or "Extract Task" based on the latest messages.
    4. **Quick Actions**: Inline action buttons (e.g., "Send Payment Link", "Book Appointment") integrate directly into the chat flow.
  - **AI Integration Points**:
    - **Intake Triage**: Automatically classify incoming messages (e.g., Support, Sales, Inquiry).
    - **Draft Generation**: Harness workers propose contextual replies based on the tenant's knowledge base and customer history.
    - **Intent Extraction**: Identify actionable items (e.g., a request for a quote) and prompt the owner to confirm creation.
  - **Key Design Decisions**:
    - Build natively in Rust for high-throughput, low-latency WebSocket connections.
    - Strict `tenant_id` enforcement via RLS for all chat data.
    - Use popular open-source libraries (e.g., `tokio` for async runtime, `axum` for web/WebSocket, `sqlx` for DB) rather than bespoke solutions.

  **Implementation Prompt**:
  "Implement the core Rust backend for the Universal Omnichannel Communication Engine. Define the data models for `Conversation` and `Message` with strict multi-tenant isolation (`tenant_id`). Build the WebSocket endpoint for real-time message delivery. Create the foundational `ChannelAdapter` interface and implement a basic `WebChat` adapter. Ensure all endpoints are covered by comprehensive unit and E2E tests, verifying that messages are correctly routed and persisted. Do not implement the AI harness integration in this phase; focus on the core messaging infrastructure."

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
