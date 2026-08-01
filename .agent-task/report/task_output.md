issue_title: "Architect Native Rust Omnichannel Chat System for OHC"
issue_description: |
  **Title:** Architect Native Rust Omnichannel Chat System for OHC

  **Problem Statement:**
  As a business owner like Maya (the baker) or Carlos (the handyman), I receive customer messages from various platforms like Instagram DMs, SMS, WhatsApp, and email. Currently, it's difficult for me to manage all these interactions in one place without needing third-party tools like Chatwoot, which is disconnected from my core operational data in OneHumanCorp. I need a single unified inbox right inside my assistant to view, manage, and reply to all customer messages, and where AI can help me draft responses automatically.

  **Research Report:**
  - The Chatwoot source code reveals a robust omnichannel architecture based around Inboxes, Channels, Conversations, Contacts, and Messages.
  - It supports multiple channel adapters: API, Email, Facebook Page, Instagram, LINE, SMS, Telegram, TikTok, Twilio, Twitter, Web Widget, WhatsApp.
  - It uses WebSocket real-time messaging for instant updates and agent routing algorithms for assignment.
  - Competitors like Shopify Inbox, Wix Inbox, and HubSpot unify messages natively so that commerce data (orders, inventory, bookings) can be attached directly to conversations.
  - Our current `src/server/services/chat` is highly nascent, with just basic schemas. Chatwoot integration is being fully retired as per standard guidelines. We need to design a feature-rich, multi-tenant Rust-based chat system replacing Chatwoot completely while providing native integrations with the rest of OHC (e.g., attaching quotes, products, or bookings to a message).

  **Design Doc:**
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
        TENANT ||--o{ INBOX : owns
        INBOX ||--o{ CHANNEL : configures
        CHANNEL ||--o{ CONVERSATION : receives
        CONTACT ||--o{ CONVERSATION : participates
        CONVERSATION ||--o{ MESSAGE : contains
        MESSAGE }|--|| SENDER : sent_by
        INBOX ||--o{ INBOX_MEMBER : has_staff
    ```
  - **UI Wireframes / Screen Flow (375px first)**:
    - **Screen 1**: Unified Inbox Feed. A list of recent conversations. Each row shows contact name, last message snippet, time, and channel icon (e.g., WhatsApp, IG). Unread indicator.
    - **Screen 2**: Conversation View. Sticky header with contact name & back button. Scrollable message history. Bottom input bar with text field, attachment button, and "Ask AI to draft" button.
  - **Mobile UX Flow**:
    - Owner receives push notification of a new message.
    - Taps notification -> Opens directly to Conversation View.
    - Reads message, sees AI-suggested draft above input.
    - Taps draft to edit or sends instantly.
  - **AI Agent Integration Points**:
    - Customer Service Agent listens to new incoming `MESSAGE` events via background job.
    - Generates auto-replies or suggests drafts to the owner based on context (previous messages, FAQ, product knowledge base).
    - Categorizes conversations (e.g., "sales inquiry", "support").
  - **Key Design Decisions**:
    - Native multi-tenant data model over independent Chatwoot instances.
    - Real-time updates via WebSockets, but designed offline-first on the client using a local database (e.g. SQLite via Flutter).
    - Channel adapters pattern: each external integration (WhatsApp, Instagram, Email) implements a common interface to ingest and dispatch messages, normalizing payloads before they hit the core `ChatConversation` layer.

  **Implementation Prompt:**
  Implement the core native omnichannel chat engine for OneHumanCorp.
  - **User-Facing Outcome**: The owner can connect channels (e.g. Web Widget, Email) to an Inbox, receive messages from customers, and reply. AI drafts are suggested for incoming messages. All works seamlessly on mobile (375px).
  - **Critical User Journey (CUJ)**:
    1. Owner logs in and goes to "Inbox" tab.
    2. Owner views a message from a customer.
    3. Owner types a reply and hits send. The customer receives the message.
  - **Acceptance Criteria**:
    - All existing Chatwoot third-party integrations are removed or deprecated.
    - Core domain entities (Inbox, Channel, Contact, Conversation, Message) are expanded to support full omnichannel features (e.g. channel-specific configs, read receipts).
    - AI Service integration for drafting replies.
    - WebSocket infrastructure for real-time delivery to connected clients.
    - High test coverage (unit and Playwright E2E testing the UI chat flow).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
