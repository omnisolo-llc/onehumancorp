issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: OHC Native Omnichannel Chat Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is currently lacking a native omnichannel chat system. The instructions mandate retiring the external Chatwoot dependency and replacing it with a custom Rust implementation within `onehumancorp/mono`. This system needs to handle customer interactions across multiple channels (Web, Instagram, WhatsApp, Email, etc.) in a unified inbox, providing a seamless experience for SMB owners like Maya, Carlos, and Priya on a 375px mobile viewport.

  ## Research Report
  - **Goal:** Replicate the core features of Chatwoot natively in Rust.
  - **Methodology:** Checked out the `https://github.com/chatwoot/chatwoot` repository to audit its source code.
  - **Key Findings (Chatwoot Architecture):**
    - **Data Models:** Chatwoot uses a robust schema with entities like `Account`, `User`, `Inbox`, `Channel`, `Conversation`, `Message`, `Contact`, `AgentBot`, and `AutomationRule`.
    - **Channels:** Uses specialized models for different platforms (e.g., `Channel::WebWidget`, `Channel::TwilioSms`, `Channel::Email`, `Channel::Whatsapp`).
    - **Real-time:** Relies heavily on WebSockets (ActionCable in Ruby) to broadcast message events to the frontend.
    - **Multi-tenancy:** Uses `Account` as the primary tenant discriminator. In OHC, this maps to `tenant_id` with Row-Level Security (RLS) in PostgreSQL.
    - **AI Integration:** Includes an `AgentBot` concept, which aligns perfectly with OHC's mandate for AI assistance handling first-line triage and replies.

  - **Competitive Analysis:**
    - Shopify Inbox: Simple, focused on sales conversion, integrated tightly with the store.
    - WhatsApp Business API: Good for direct communication, but lacks multi-agent features out of the box.
    - OHC's implementation must be "assistant-first," meaning AI drafts and triages messages before the owner even needs to intervene, reducing cognitive load.

  ## Design Doc
  ### Data Model & Invariants (Rust / PostgreSQL)
  We will introduce new entities managed via `sea-orm`:
  1.  `inbox`: The central container for conversations. Fields: `id`, `tenant_id`, `name`.
  2.  `channel`: Represents a specific communication medium (e.g., Web, Instagram, SMS) connected to an `inbox`. Fields: `id`, `tenant_id`, `inbox_id`, `type`, `credentials` (encrypted).
  3.  `contact`: A customer interacting with the business. Fields: `id`, `tenant_id`, `name`, `email`, `phone`, `custom_attributes`.
  4.  `conversation`: A thread between a `contact` and the business (via an `inbox`). Fields: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, closed, snoozed), `assignee_id` (can be an AI agent).
  5.  `message`: An individual message within a `conversation`. Fields: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, user, agent_bot), `sender_id`, `content`, `content_type`, `created_at`.

  *Multi-tenant Isolation:* Every table must have a `tenant_id` column, and RLS policies must be applied in PostgreSQL to prevent cross-tenant data leakage.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string channel_type
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

  ### Mobile UX Flow (375px First)
  1.  **Unified Inbox View:** A clean list of active conversations, sorted by urgency or recent activity. Unread messages have a distinct visual indicator (Premium Token color).
  2.  **Conversation View:** A chat interface. AI-drafted replies appear as "ghost text" or a clearly marked draft box above the input field. The owner can tap "Send" or edit.
  3.  **Context Panel (Slide-over):** Swiping from the right reveals the `Contact` profile, previous orders, and notes, ensuring the owner has full context without leaving the chat.
  4.  **Action Buttons:** Quick actions for "Create Quote", "Request Payment", or "Schedule Visit" integrated directly into the chat composer.

  ### AI Agent Integration Points
  - **Work Triage:** An AI background worker listens for new `conversation` events. It analyzes the first message, tags the conversation, and sets priority.
  - **Customer Assistant:** When a new `message` is received from a contact, the assistant generates a draft reply and saves it to a temporary `draft_messages` cache (Redis) or as a special message type, notifying the owner.

  ## Implementation Prompt
  **Role:** Backend & UI Implementer
  **Task:** Implement the foundation of the native Rust Omnichannel Chat system, replacing the need for external Chatwoot.
  **Requirements:**
  1.  **Database & Entities:** Create the `sea-orm` entities and PostgreSQL migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`. Ensure strict `tenant_id` RLS is applied.
  2.  **Core Services:** Implement the Rust gRPC/REST services for CRUD operations on these entities.
  3.  **Real-time Infrastructure:** Set up the basic WebSocket infrastructure (using `axum` and `tokio-tungstenite`) to broadcast `message.created` events to connected frontend clients based on `tenant_id`.
  4.  **UI Scaffold (Mobile-First):** Build the foundational Flutter (or Next.js depending on current frontend stack) screens for the unified inbox list and a basic conversation view. Ensure it looks perfect on a 375px screen using translucent glass materials.
  5.  **No Mocks:** All data in the UI must come from the newly created database tables via the Rust API.
  6.  **Tests:** 100% unit test coverage for the new Rust services and at least one Playwright E2E test verifying a message can be sent and received in the UI.

  **Acceptance Criteria:**
  - A user can open the Inbox screen and see an empty state (truthful, no mocks).
  - A test endpoint can simulate an incoming message, which appears in real-time in the UI via WebSockets.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
