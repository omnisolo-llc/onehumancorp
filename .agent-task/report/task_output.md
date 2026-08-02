issue_title: "Implement Native Rust Omnichannel Chat System Core Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context.

  OHC previously relied on Chatwoot as an external dependency, but as per the architectural guidelines, Chatwoot is 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Architecture:** Chatwoot uses an `Account` (Tenant) -> `Inbox` -> `Conversation` -> `Message` model. It also separates users into `Contact` (Customer) and `User` (Agent). We must replicate this data model natively in our Rust backend, but tailored to our unified workspace structure.
  - **Data Models:**
    - `conversations`: Needs tracking of status (open, closed), priority, snooze, agent assignees.
    - `messages`: Needs content types (text, attachment), sender types, read receipts.
    - `inboxes`: Needs channel types (WhatsApp, Web Widget, Email).
  - **Implementation Strategy:** Build matching native Rust microservices, crates, and frontend UI components in OHC to achieve 100% feature parity with Chatwoot, starting with the core data schemas and the gRPC API layer.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox Feed:** The main screen displays a unified list of conversations across all channels. Each item shows an avatar, the last message, channel icon (WhatsApp, Instagram, etc.), and an unread indicator.
  - **Conversation View:** Tapping a conversation opens a standard chat UI, but with the top context panel showing the customer's previous purchase history or appointment bookings, powered by The Ambassador AI agent.
  - **Smart Replies:** Instead of empty text boxes, the bottom of the screen proactively shows AI-generated draft replies based on context.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to new message events. When a new message arrives, it retrieves customer history and drafts a contextual response.

  ## Implementation Prompt
  **User-Facing Outcome:** A business owner can view a unified inbox of all their customer conversations natively within OHC without any external Chatwoot iframe or dependency.
  **CUJ & Acceptance Criteria:**
  1. Define the PostgreSQL data schema (migrations) for `inboxes`, `conversations`, and `messages`, ensuring Row Level Security (`tenant_id`) is strictly enforced.
  2. Implement the gRPC and REST API endpoints in Rust to support creating inboxes, starting conversations, and sending messages.
  3. Replicate the core features of Chatwoot's data models (`app/models/conversation.rb` and `app/models/message.rb`) in the native Rust service.
  4. Build Playwright E2E tests verifying that a user can open a conversation and send a message.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
