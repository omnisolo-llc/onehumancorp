issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement:**
  OneHumanCorp's mission is to be an owner-first AI work assistant. Currently, OHC lacks a robust, natively built omnichannel chat system to interact with customers. The mandate strictly prohibits the use of external services like Chatwoot for customer chat. Our personas (like Maya, Carlos, and Fatima) need a unified inbox to manage DMs, SMS, emails, and web chats from a single, responsive interface on their mobile devices (375px wide). The absence of this feature forces owners to juggle multiple apps, breaking the core promise of a unified work triage experience.

  **Research Report:**
  - **Codebase Audit:** OHC's backend is Rust-based. There are existing directories for `chat` under `src/server/` but a complete omnichannel data model and WebSocket support are missing.
  - **Chatwoot Source Code Audit:** Chatwoot relies heavily on `Conversation`, `Message`, `Inbox`, and `Contact` models. These models map well to our need for a unified multi-tenant architecture. Chatwoot handles various channels (web, email, SMS, FB, IG) via specialized adapters, and leverages WebSockets for real-time updates.
  - **Competitor Analysis:** Shopify Ping and Wix Inbox provide similar functionality. They focus on unifying disparate communication channels into a single, mobile-optimized feed that supports quick actions (e.g., sending quotes or payment links directly in chat).

  **Design Doc:**

  *Architecture:*
  - **Core Models (Rust/PostgreSQL):**
    - `Inbox`: Represents a channel endpoint (e.g., a specific Instagram account or Web Widget) associated with a tenant.
    - `Contact`: Represents a customer across channels.
    - `Conversation`: Links a `Contact` and an `Inbox`, holding the state of a specific interaction thread.
    - `Message`: The individual message within a `Conversation`. Includes `content`, `content_type` (text, image, rich-action), and `sender_type` (Contact, Agent, Bot).
  - **Real-time Layer:** Native Rust WebSocket implementation for live message delivery to the web/mobile clients.
  - **Channel Adapters:** A trait-based system in Rust to handle incoming/outgoing messages for different platforms (Web Widget, Email, Instagram).
  - **AI Integration:** The `AI Agent` operates as a distinct `sender_type`. The `Work Triage` AI department hooks into the `Conversation` lifecycle to auto-draft replies or suggest actions based on context.

  *Architecture Diagram:*
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : manages
    TENANT ||--o{ CONTACT : has
    INBOX ||--o{ CONVERSATION : contains
    CONTACT ||--o{ CONVERSATION : participates_in
    CONVERSATION ||--o{ MESSAGE : contains
    MESSAGE }|--|| CONTACT : sender_is
    MESSAGE }|--|| TENANT : sender_is_agent
  ```

  *Mobile UX Flow (375px):*
  1.  **Work Feed (Home):** The owner sees a prioritized list of active `Conversations` that need attention. Unread messages have clear visual indicators using the OHC Premium Token library.
  2.  **Conversation View:** Tapping a conversation opens a chat view. Sticky header with the `Contact`'s name and channel icon. Scrollable message history. Bottom input bar with a native mobile keyboard and quick-action buttons (e.g., "Draft AI Reply", "Send Quote").
  3.  **Translucent Glass:** Popovers (like contact details or action menus) use translucent glass styling over the chat view.

  *Key Design Decisions:*
  - **Strict Multi-tenancy:** All queries must enforce `tenant_id` boundaries.
  - **Offline-Tolerant Reads:** The Flutter frontend will cache recent conversations locally so the owner can view them on slow networks. Writes (sending a message) will queue locally and sync when online.

  **Implementation Prompt:**
  As an Implementer agent, your task is to build a unified inbox experience that allows our business owner personas (like Maya, Carlos, and Fatima) to manage all customer communication from a single mobile-first interface (375px width).

  *User-Facing Outcome:*
  - The business owner must be able to open the app, immediately see a list of unread messages from different channels (e.g., a web widget or email), and reply to them natively without leaving the application.
  - When the owner replies, the message should appear instantly in the UI for the customer on the web widget and vice-versa.

  *Critical User Journey (CUJ):*
  1.  The owner navigates to the "Unified Inbox" section.
  2.  They select an ongoing conversation with a customer.
  3.  The owner views the chat history and types a reply.
  4.  The owner hits send, and the message instantly appears in the conversation stream without needing a page refresh.

  *Acceptance Criteria:*
  - The unified inbox UI must be fully functional on a 375px wide mobile view and follow the macOS-style translucent glass and UniFi layouts.
  - The chat interface must correctly display message threads and dynamically update when new messages arrive.
  - Complete backend capability to handle the routing and persistent storage of messages securely isolated by tenant ID.
  - Test coverage (unit and E2E via Playwright) verifying that an owner can successfully send and receive a message in the unified inbox.

  **Priority:** P0 (Critical for core product offering)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
