issue_title: "Native Rust Omnichannel Chat System"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System

  **Problem Statement**:
  OHC currently lacks a native omnichannel chat system, and the external legacy dependency is officially 100% RETIRED. Our non-technical owner/operators (Maya the baker, Carlos the handyman) need a unified inbox where they can seamlessly interact with customers across various channels (Instagram DMs, Web Widget, WhatsApp, SMS) without ever knowing the underlying system. Currently, this capability is missing, leading to fragmented communication and missed business opportunities.

  **Research Report**:
  - The legacy architecture relied on a third-party service which introduced latency, complexity, and broke our strict multi-tenant isolation and security (SPIFFE/SPIRE) invariants.
  - Analyzing the source code of the legacy service (`app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb`), we identified key architectural patterns: Conversations, Messages, Inboxes, Contacts, and Channel Adapters (Web, IG, FB, etc.).
  - A native Rust implementation using our existing PostgreSQL backend and multi-tenant row-level security (RLS) is required.
  - Competitor analysis (Shopify Inbox, Wix Inbox) shows that successful implementations integrate chat deeply with commerce actions (sending quotes, products, booking links).

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o{ CHANNEL_ADAPTER : uses
    ```
  - **Mobile UX Flow (375px first)**:
    - **Unified Inbox View**: A simple list view showing recent conversations with unread indicators and avatars. Swiping left archives the chat.
    - **Chat View**: Standard chat interface. Messages grouped by date. Input area with + button for attaching quotes, product links, and scheduling links.
    - **Agent Assistant**: AI drafts replies inline; the owner taps to approve or edit before sending.
  - **AI Agent Integration Points**:
    - **Customer Assistant (Operations Dept)**: Listens to new messages via pub/sub (Valkey), drafts replies based on tenant context (store hours, inventory), and updates the UI with suggested responses.
    - **Triage**: Tags conversations (e.g., "Urgent", "Lead", "Support") automatically on creation.
  - **Key Design Decisions**:
    - Build in Rust leveraging `tokio` and gRPC/REST APIs.
    - Implement real-time updates using WebSockets/SSE integrated with Valkey for pub/sub across instances.
    - Strict multi-tenancy: Every table (`inboxes`, `conversations`, `messages`, `contacts`) must have `tenant_id` with RLS enforced.

  **Implementation Prompt**:
  *Objective*: Implement the foundational data models and core API for the Native Rust Omnichannel Chat System, replacing the legacy third-party service.
  *CUJ*: As Maya (home baker), I want to see a new message in my OHC app when a customer texts my business number, so I can reply instantly from my phone and have the AI suggest a response based on my previous cake orders.
  *Acceptance Criteria*:
  1. Define Rust structs, Protobuf definitions, and PostgreSQL migrations for `Inbox`, `Conversation`, `Message`, and `Contact` entities.
  2. Implement strict multi-tenant isolation (`tenant_id`) with RLS for all new tables.
  3. Create core REST/gRPC endpoints to create and list conversations and messages.
  4. Ensure all database interactions go through our shared-database persistence layer.
  5. Provide 100% unit test coverage and Playwright E2E tests for the basic message sending/receiving flow.
  6. The UI must contain zero mock data and function perfectly on a 375px width screen.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
