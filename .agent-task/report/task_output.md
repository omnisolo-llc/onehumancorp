issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp currently lacks a native, high-performance omnichannel chat system. The previous external dependency on Chatwoot is 100% retired. We need to implement a native Rust replacement that handles conversations, messages, inboxes, and channel adapters seamlessly for our owner/operator personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun), ensuring they have a unified inbox to manage all customer communications without relying on third-party services.

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture revolves around the following entities:
  - **Account/Tenant**: The multi-tenant boundary.
  - **Inbox**: Represents a unified queue for messages, linking to specific channels (e.g., Email, Web Widget, API). Contains configs like auto-assignment, business hours, and CSAT.
  - **Conversation**: Links a contact to an inbox/account. Tracks status (open, resolved), assignee, priority, and activity timestamps.
  - **Message**: The actual content payload within a conversation. Supports different content types, sender types (agent, contact, system), and private notes.
  - **Channel**: Specific adapters for different communication mediums.

  Competitive analysis of platforms like Shopify Inbox, WeCom, and Stripe shows that a unified, mobile-first inbox with real-time WebSocket updates is critical for operator efficiency.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : has
      ACCOUNT ||--o{ CONVERSATION : has
      ACCOUNT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : receives
      INBOX ||--|| CHANNEL : configured_via
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : participates_in

      ACCOUNT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid account_id
          string name
          string channel_type
          boolean working_hours_enabled
      }
      CONVERSATION {
          uuid id
          uuid account_id
          uuid inbox_id
          uuid contact_id
          string status
          uuid assignee_id
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          uuid account_id
          text content
          string message_type
          string sender_type
      }
      CONTACT {
          uuid id
          uuid account_id
          string name
          string email
          string phone_number
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: The owner opens the OHC app. The first screen shows a prioritized list of active conversations across all channels (Instagram DMs, Web Widget, Email). Unread messages have a clear visual indicator using the OHC Premium Token system (macOS translucent materials).
  2. **Conversation Thread**: Tapping a conversation opens the thread. The header shows the contact name and channel icon. The message history is displayed with clear differentiation between customer messages, agent replies, and private AI notes.
  3. **AI Assistant Integration**: At the bottom of the thread, above the native mobile keyboard, the AI assistant suggests draft replies based on the conversation context and business knowledge base. The owner can tap to approve or edit.
  4. **Action Menu**: A swipe or tap reveals actions: Resolve, Snooze, Assign, or trigger a Workflow (e.g., generate a quote).

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Listens to new `MessageCreated` events on the event bus. Automatically drafts replies for incoming customer messages and saves them as pending suggestions.
  - **Work Triage Assistant**: Analyzes new conversations to update priority, categorize intent (e.g., support, sales inquiry), and alert the owner if urgent.

  ### Key Design Decisions
  - **Native Rust & WebSockets**: Implement the core logic in Rust within `onehumancorp/mono`. Use WebSockets for real-time message delivery to the Flutter frontend, ensuring instant updates.
  - **Strict Multi-Tenancy**: Every database table and API endpoint MUST enforce `account_id` (tenant ID) isolation. Zero-trust security model.
  - **Event-Driven Architecture**: Use a reliable event bus (e.g., Redis Streams or PostgreSQL LISTEN/NOTIFY) to decouple message ingestion from AI processing and notification dispatch.

  ## Implementation Prompt
  **Goal**: Implement the core backend data models and CRUD APIs for the native Rust omnichannel chat system (Inboxes, Conversations, Messages, Contacts) to replace Chatwoot.

  **Persona**: Maya (Baker) needs to see all her Instagram DM inquiries and web widget questions in one place without switching apps.

  **CUJ**: Maya logs into OHC, navigates to the Inbox tab, and sees a unified list of conversations from different channels. She opens a conversation, reads the messages, and sends a reply, which is processed by the new Rust backend.

  **Acceptance Criteria**:
  1. Database migrations created for Inboxes, Conversations, Messages, and Contacts with strict `account_id` multi-tenancy.
  2. Rust gRPC/REST endpoints implemented for listing conversations, fetching message history, and sending new messages.
  3. 100% unit test coverage for the new Rust services.
  4. At least one E2E Playwright test verifying the unified inbox UI renders and can send a message.
  5. The UI must contain ZERO mock data and integrate with the real Rust backend.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
