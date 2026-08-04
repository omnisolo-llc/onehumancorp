issue_title: "Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC previously relied on Chatwoot as an external service/dependency for omnichannel customer support and chat functionality. Relying on an external third-party service breaks our strict multi-tenant isolation rules, complicates our deployment model, and introduces external latency and reliability risks. Chatwoot has been 100% RETIRED as an external dependency. We need a native Rust implementation of a high-performance, multi-tenant omnichannel customer support & chat engine natively integrated into OHC to achieve 100% feature parity with Chatwoot, while maintaining strict Zero-Trust multi-tenant isolation, SPIFFE/SPIRE identity, and mobile-first UX.

  ## Research Report
  - **Market Context**: Modern SMBs (like Maya the baker and Carlos the handyman) receive customer inquiries across multiple channels (Instagram, WhatsApp, Web, SMS). A unified inbox is critical for them to manage these conversations efficiently.
  - **Competitor Analysis**: Tools like Chatwoot, Zendesk, and Shopify Inbox provide unified inbox capabilities. Chatwoot's architecture relies on a robust data model with Inboxes, Conversations, Messages, Contacts, and Channel Adapters (e.g., WhatsApp, Web Widget, Email).
  - **Current OHC Deficiencies**: Without a native chat engine, OHC cannot provide the unified inbox experience required by our core personas without relying on external services, which violates our architectural constraints.

  ## Design Doc
  ### Architecture
  - **Core Components**:
    - `Inbox Service`: Manages unified inboxes for tenants.
    - `Conversation Service`: Handles conversation lifecycle and routing.
    - `Message Service`: Manages message persistence and retrieval.
    - `Channel Adapters`: Implementations for specific channels (e.g., Web, WhatsApp, Instagram).
    - `WebSocket Gateway`: Real-time bidirectional communication for live updates.
  - **Data Model (Mermaid)**:
    ```mermaid
    erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : participates
      CHANNEL ||--|{ INBOX : configures
    ```
  - **AI Agent Integration**:
    - **Customer Assistant Agent**: Automatically drafts replies for incoming messages, utilizing tenant-scoped memory and preferences.
    - **Operations Assistant Agent**: Triggers actions (e.g., creating a booking or task) based on conversation context.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View**: A simple, unified list of conversations across all channels, prioritized by the Work Triage agent. Clean UI using OHC Premium Token library with translucent materials.
  - **Conversation View**: Real-time chat interface with clear distinctions between customer messages, agent drafts, and automated system messages. Touch-friendly action buttons (44x44px min) for quick actions (e.g., "Send Draft", "Create Quote").

  ### Key Design Decisions
  - **Native Rust**: Ensures high performance, memory safety, and seamless integration with the existing OHC backend infrastructure.
  - **Row-Level Security**: Enforce strict tenant isolation at the database level for all chat-related tables (`ENABLE ROW LEVEL SECURITY`).
  - **Real-Time WebSockets**: Critical for the live chat experience, managed natively in Rust for scalability.

  ## Implementation Prompt
  **Goal**: Implement the core data models, REST/gRPC APIs, and WebSocket gateway for the Native Rust Omnichannel Chat Engine.
  **CUJ**: As a tenant (e.g., Maya), I want to receive a message from a customer via the web widget, see it instantly in my unified inbox, and reply to it, with the reply being delivered back to the customer in real-time.
  **Acceptance Criteria**:
  - Implement Rust structs and PostgreSQL schemas for `Inbox`, `Conversation`, `Message`, and `Contact` with strict tenant isolation.
  - Implement gRPC/REST APIs for CRUD operations on these entities.
  - Implement a WebSocket gateway for real-time message delivery and typing indicators.
  - Ensure 100% unit test coverage for the new Rust services.
  - Write Playwright E2E tests covering the unified inbox CUJ (receiving and replying to a message).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
