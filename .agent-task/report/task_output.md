issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Research Report: Custom Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp previously integrated with Chatwoot as an external service for omnichannel messaging. We must now fully retire this integration and replace it with our own highly performant, multi-tenant Rust backend capable of supporting omnichannel interactions. The primary goal is to replicate the core Chatwoot feature set natively, establishing a secure, scalable, and fully integrated chat platform for owners and operators without leaving the OHC ecosystem.

  ## Research Findings
  After cloning and analyzing the `chatwoot/chatwoot` repository, we found that its architecture heavily revolves around the following components:
  - **Inboxes**: The central point where messages from different channels converge. It handles settings like auto-assignment and working hours.
  - **Channels**: Abstractions over various messaging platforms (Web Widget, Email, WhatsApp, Facebook, etc.). Each channel holds platform-specific configuration and secrets.
  - **Contacts**: Represents external users interacting with the business through a channel. It stores contact information and context across conversations.
  - **Conversations**: Groups related messages between a contact and agents (or bots) within an inbox. It maintains state (open, resolved, snoozed), assignee, and SLAs.
  - **Messages**: Individual interactions within a conversation. It tracks the sender (agent, contact, bot), content type (text, attachment), and status.

  The existing OHC Rust backend (`src/server/services/chat`) has a very rudimentary implementation of these models and service functions. It needs to be significantly expanded to achieve parity with the foundational features of Chatwoot.

  ## Architecture Design
  ### High-Level Architecture (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ ChatInbox : "has"
      Tenant ||--o{ ChatChannel : "has"
      Tenant ||--o{ ChatContact : "has"
      ChatInbox ||--o{ ChatChannel : "configures"
      ChatInbox ||--o{ ChatConversation : "contains"
      ChatContact ||--o{ ChatConversation : "participates in"
      ChatConversation ||--o{ ChatMessage : "holds"

      ChatInbox {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
          string greeting_message
          boolean working_hours_enabled
      }

      ChatChannel {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string channel_type
          jsonb config
      }

      ChatContact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
          jsonb custom_attributes
      }

      ChatConversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id
          string status
          int priority
      }

      ChatMessage {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type
          uuid sender_id
          string content
          string content_type
          jsonb additional_attributes
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox View**: A clean list of active conversations, prioritizing unassigned or new messages. Swipe actions for quick resolving or snoozing.
  - **Conversation View**: Native-feeling chat interface. Real-time updates. Clear visual distinction between contact messages, agent replies, and system notes. Easy access to contact details (side drawer or bottom sheet).
  - **Settings**: Simple toggles for out-of-office, auto-assignment, and adding new channels (e.g., Web Widget).

  ### AI Agent Integration
  - **Triage Agent**: Automatically categorization of incoming messages.
  - **Responder Agent**: Drafting replies based on context and past conversations.
  - **Routing Agent**: Intelligent assignment to specific human agents based on workload and expertise.

  ## Implementation Prompt
  **User Persona**: Carlos (Field Service Owner, Android phone). He needs to receive inquiries from his website widget, reply quickly, and track the conversation history.

  **Critical User Journey (CUJ)**:
  1. Carlos's customer visits his website and sends a message via the Web Widget.
  2. The message appears instantly in Carlos's OHC Inbox view.
  3. Carlos taps the conversation, reads the message, and sends a reply.
  4. The customer receives the reply on the widget.

  **Acceptance Criteria**:
  1. Extend `ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, and `ChatMessage` models in Rust to support the expanded fields required for parity (e.g., status, priority, content_type, custom_attributes).
  2. Implement necessary database migrations for the updated models.
  3. Enhance `ChatService` with robust CRUD operations and business logic (e.g., changing conversation status, handling attachments).
  4. Ensure strict tenant isolation (`tenant_id` on all queries).
  5. 100% Unit Test coverage for the updated service layer.
  6. E2E Playwright test simulating the CUJ.

  ## Priority
  `P0`

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
