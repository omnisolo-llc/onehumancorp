issue_title: "[Architecture] Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Title: Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  OHC currently relies on external systems (or has a conceptual gap) for multi-tenant, omnichannel customer support and chat functionality. Relying on an external service like Chatwoot violates our Zero-Trust and self-contained architectural principles, creates multi-tenancy data sync nightmares, and increases operational complexity for our target personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun). We need a unified, native Rust chat and customer support engine that is fully integrated into OHC's ecosystem and AI agents, enabling the owner to instantly see, triage, and reply to customer inquiries directly from their phone or browser.

  ## Research Report
  Based on an audit of the Chatwoot source code (`app/models/conversation.rb`, `message.rb`, `inbox.rb`, etc.), Chatwoot's core abstractions map perfectly to OHC's needs but carry legacy Rails overhead.
  - **Conversations**: The central entity linking messages to a contact and an inbox.
  - **Inboxes**: Channels (Web widget, Email, WhatsApp, Facebook, API) that route into the system.
  - **Contacts**: The customer profile.
  - **Messages**: Individual payloads within a conversation.
  - **AI / Automation**: SLA policies, macros, and canned responses.

  By implementing this natively in Rust using our PostgreSQL + Row-Level Security architecture, we can ensure 100% data isolation (`tenant_id` on every table) and lightning-fast real-time syncing via WebSockets without third-party dependencies.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  - **Inbox List Screen**: A unified feed of all active conversations. Clean, unread indicators, avatar for contact. Swipe actions for "Resolve" or "Snooze".
  - **Conversation Screen**:
    - Header: Contact name and channel icon (e.g., WhatsApp, Web).
    - Body: Chat bubbles. System events (e.g., "Agent assigned") inline.
    - Footer: Message composer with native keyboard support, quick attachments, and AI "Draft Reply" button.
  - **Mobile Flow**: Owner taps notification -> opens Conversation Screen -> reviews AI-drafted reply -> taps send.

  ### AI Agent Integration Points
  - **Triage Agent**: Automatically assigns priority and categorizes the conversation based on the initial message intent.
  - **Customer Assistant**: Drafts replies by reading conversation history and contact notes. Displays the draft as a pending message in the composer for the owner to review or send automatically if confidence is high.

  ### Key Design Decisions
  - **Rust Native**: Eliminate external Chatwoot dependencies. Built directly into the `server` binary to reduce deployment footprint.
  - **Row-Level Security**: Every chat entity (Inbox, Conversation, Message, Contact) will strictly enforce `tenant_id` for security.
  - **WebSocket Real-Time**: Axum + WebSockets will provide live updates to the Tauri/PWA clients, ensuring the mobile app feels instant.

  ## Implementation Prompt
  Implement the core database schema, models, and Axum API routes for the native Rust Omnichannel Chat system.
  1. Create the necessary PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring all tables have a `tenant_id` and RLS enabled.
  2. Implement the CRUD API endpoints for these resources.
  3. Integrate basic WebSocket support for real-time message broadcasting to clients.
  4. Ensure 100% unit test and Playwright E2E coverage for the inbox view and sending a message.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
