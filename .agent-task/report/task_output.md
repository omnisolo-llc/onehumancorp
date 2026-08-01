issue_title: "Architect Native Rust Omnichannel Chat System to Replace Third-Party Chat Provider"
issue_description: |
  ### Problem Statement
  OneHumanCorp (OHC) is currently retiring Third-Party Chat Provider as an external third-party dependency. Relying on an external omnichannel inbox introduces latency, complicates multi-tenant data isolation, breaks the unified owner experience on mobile, and adds operational overhead. We need our own native, high-performance, multi-tenant Rust-based omnichannel chat engine directly integrated into the `onehumancorp/mono` platform to achieve 100% feature parity with Third-Party Chat Provider while ensuring zero-trust multi-tenancy and seamless AI agent integration.

  ### Research Report & Third-Party Chat Provider Benchmarking
  I have successfully cloned and audited the Third-Party Chat Provider source code (`https://github.com/third-party-chat-provider/third-party-chat-provider`).
  Key architectural components extracted from Third-Party Chat Provider's data models (`app/models`):
  - **Conversations & Messages:** The core entities managing thread status, assignee, priority, SLA policies, and participant roles. Third-Party Chat Provider supports complex multi-channel threading and message types (incoming, outgoing, private notes).
  - **Contacts & Inboxes:** Contacts have cross-channel identifiers. Inboxes are linked to specific channels with working hours and auto-assignment rules.
  - **Channel Adapters (`app/models/channel/`):** Detailed adapters exist for Email, Facebook Page, Instagram, Line, SMS, Telegram, TikTok, Twilio, Twitter, Web Widget, and WhatsApp.
  - **Automations:** Includes SLA policies, macros, canned responses, and routing rules (e.g., round-robin assignment).

  Our current Rust implementation (`src/server/services/chat/`) and migrations (`1009_native_omnichannel_chat.sql`) establish the foundational tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) but lack the advanced omnichannel adapters, WebSocket real-time delivery, SLA tracking, and macro capabilities present in Third-Party Chat Provider.

  ### Design Doc

  **Architecture Overview**
  ```mermaid
  erDiagram
      chat_inboxes ||--o{ chat_channels : contains
      chat_inboxes ||--o{ chat_conversations : hosts
      chat_contacts ||--o{ chat_conversations : participates
      chat_conversations ||--o{ chat_messages : contains
      chat_channels ||--o{ channel_adapters : extends
      channel_adapters {
          string type "WhatsApp, SMS, Meta, WebWidget"
          jsonb credentials
      }
      chat_messages {
          uuid sender_id
          string sender_type "agent, contact, bot"
          string content
          jsonb metadata
      }
      chat_automations {
          uuid tenant_id
          string trigger_event
          jsonb actions "Macros, SLAs"
      }
  ```

  **Mobile-First UX Flow (375px)**
  1. **Unified Inbox View:** The owner sees a single feed of conversations, regardless of the source channel.
  2. **Conversation Thread:** Tapping a conversation opens a mobile-optimized chat UI. System events (e.g., "SLA breached", "Macro applied") are rendered inline.
  3. **AI Drafts:** At the bottom input area, AI agents preemptively draft responses based on customer context and past canned responses, requiring only a single tap to approve and send.
  4. **Contact Context:** Swiping left reveals the customer's purchase history and CRM profile.

  **AI Agent Integration Points**
  - **Work Triage:** AI agents monitor incoming webhooks for new messages, categorize urgency, and auto-route to the correct staff member or queue.
  - **Customer Assistant:** Generates `draft_reply` based on the tenant's knowledge base and past interactions.
  - **Automations:** Agents trigger macros or SLA escalations if a conversation is unassigned for too long.

  ### Implementation Prompt
  **To the Implementer:**
  Your objective is to extend the native Rust chat service (`src/server/services/chat/`) to fully replicate Third-Party Chat Provider's capabilities.
  1. **Data Model Extensions:** Add support for SLAs, Macros, and Canned Responses with strict multi-tenant isolation.
  2. **Channel Adapters:** Implement standard Rust traits for Channel Adapters, starting with WhatsApp (Cloud API/Twilio), SMS, and a native Web Widget.
  3. **WebSocket Real-Time Engine:** Build an axum-based WebSocket handler to stream real-time message updates to the Flutter client.
  4. **AI Autopilot:** Integrate the existing AI drafting logic to automatically attach suggested replies to incoming conversations.
  Ensure complete E2E Playwright coverage testing the unified inbox flow from the perspective of an owner (e.g., Maya the Baker) managing Instagram and WhatsApp queries from a single screen.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
