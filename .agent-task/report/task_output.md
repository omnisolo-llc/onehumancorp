issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The external unified inbox dependency is fully retired. OHC currently lacks an integrated, multi-tenant omnichannel chat and customer support system that functions natively in Rust. Maya, Carlos, and Priya need a unified inbox to manage customer inquiries across channels (web chat, Instagram DMs, etc.) without relying on external SaaS tools or managing separate interfaces. The system must natively fit within OHC's architecture, leveraging the existing Rust server, Postgres data models, and the built-in AI agents.

  **Research Report**
  As mandated, OHC has retired its previous external customer service dependency. I audited typical open-source omnichannel repository structures, focusing on their core data models (Conversations, Messages, Inboxes, Contacts, Channel Adapters) and WebSocket-based real-time event distribution.
  Competitor architectures like Shopify Inbox and Stripe also emphasize native, embedded communication channels tightly coupled to the primary CRM/transactional data model.
  Our implementation will mirror these concepts but strictly in Rust, utilizing our current stack (Axum/Tonic, PostgreSQL with tenant isolation).

  **Design Doc**
  *Architecture Diagram (Mermaid.js)*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone
          string email
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string content
          string sender_type
      }
  ```

  *Mobile UX Flow (375px First)*
  - **Inbox List View**: Simple list of active conversations with unread indicators. Large touch targets (min 44x44px).
  - **Chat View**: Standard chat interface. Input box at the bottom. Messages scroll up. Clear visual distinction between user messages, AI agent drafts, and sent messages.
  - **Translucent Glass**: Apply macOS-style translucent styling to headers and action bars.

  *AI Agent Integration Points*
  - The AI assistant monitors the Inbox.
  - When a new `Message` arrives, the agent drafts a response (marked as draft).
  - The human owner (e.g., Maya) can review the draft and click "Send".

  **Implementation Prompt**
  Implement the core native Rust omnichannel chat system for OHC.
  1. Define the database schema (Contacts, Inboxes, Conversations, Messages) ensuring row-level multi-tenant isolation.
  2. Create the Rust service layer in `src/server/services/chat/` for CRUD operations on these entities.
  3. Implement REST/gRPC APIs to expose these operations.
  4. Build a mobile-first (375px) chat UI in the Tauri app (`src/ui/tauri/`) using OHC's translucent glass design tokens.
  5. Ensure the UI can fetch and display conversations and messages from the Rust backend.
  6. Add comprehensive unit tests (100% coverage) and Playwright E2E tests for the chat flow.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
