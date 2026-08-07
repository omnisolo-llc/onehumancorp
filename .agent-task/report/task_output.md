issue_title: "Implement Core Chatwoot Domain Models in Rust (Native Omnichannel Chat)"
issue_description: |
  **Problem Statement**
  OneHumanCorp (OHC) is replacing Chatwoot with a high-performance native Rust implementation for its omnichannel customer support and chat engine. OHC currently relies on external Chatwoot services for omnichannel support, which conflicts with the architectural mandate to retire external Chatwoot dependencies and keep OHC assistant-first and owner-centered inside its own multi-tenant environment. Small-business owners (like Maya the baker and Carlos the handyman) need a seamless, invisible assistant to handle chat contexts natively.

  **Research Report**
  We examined the Chatwoot `app/models` codebase to understand its domain model for a conversational support CRM. Key entities include:
  - **Account**: Top-level multi-tenant context.
  - **Inbox**: A channel configuration (e.g. Email, WhatsApp, Instagram).
  - **Contact**: The customer or lead communicating with the business.
  - **Conversation**: An ongoing thread of communication within an Inbox and associated with a Contact.
  - **Message**: Individual payloads within a Conversation.

  Competitor architectures like Shopify Inbox or Zendesk emphasize real-time multi-channel aggregation grouped by the customer. OHC needs this structure embedded in its core Rust domain, utilizing PostgreSQL with row-level security (`tenant_id` for isolation).

  **Design Doc**
  - **Architecture**:
    - We will implement the core entities in Rust within a new `src/server/domain/chat/` module or alongside existing structs if they are partial.
    - Entities:
      - `Inbox`: Includes config for different channel types, auto-assignment rules.
      - `Contact`: Represents the customer, identifying fields like email, phone, identifier.
      - `Conversation`: Links Inbox, Contact, and Agent assigned. Tracks status (open, resolved, snoozed).
      - `Message`: Contains content, type (incoming, outgoing, template), and links to Conversation.
    - **Database**: Each model will correspond to a PostgreSQL table enforcing `tenant_id` at the row level for isolation.

  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : has
      ACCOUNT ||--o{ CONTACT : has
      ACCOUNT ||--o{ CONVERSATION : has
      ACCOUNT ||--o{ MESSAGE : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : holds

      ACCOUNT {
          uuid tenant_id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string channel_type
          string name
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string email
          string phone_number
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type
      }
  ```

  - **Mobile UX Flow (375px)**:
    - Flow: Open OHC App -> Triage Dashboard -> Tap "Urgent Chats" -> Unified Inbox View (375px) -> Tap Conversation -> Chat Thread View.
    - In the Unified Inbox view (375px), conversations are presented as full-width cards with a slight translucent material background. Avatars of the Contact are on the left, name and last message snippet (truncated at 1 line) in the middle, and timestamp/status (e.g. "Action Required") aligned right.
    - Inside the Chat Thread View (375px), the owner's messages align right (blue gradient) and customer messages align left (subtle gray). Input field at the bottom has a native mobile keyboard trigger and an "Ask AI to draft" action button floating nearby.

  - **AI Agent Integration**:
    - AI agents will hook into the `Message` creation lifecycle. An agent (like Customer Assistant) can read the `Conversation` context and draft a `Message` reply for the owner's review.
    - Using Redis Redlock (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) to prevent race conditions if multiple background events try to trigger an AI draft simultaneously.

  **Implementation Prompt**
  Implement the foundational Rust data structures and PostgreSQL DDL (or equivalent ORM definitions) for the native Chatwoot replacement.
  1. Define the models `Contact`, `Inbox`, `Conversation`, and `Message` in Rust.
  2. Ensure each model strictly adheres to multi-tenancy requirements, containing a `tenant_id` (or `account_id`) to ensure data isolation via PostgreSQL Row Level Security.
  3. Include key fields discovered from Chatwoot (e.g. `status`, `channel_type`, `content`, `message_type`).
  4. Write unit tests ensuring that structs can be serialized/deserialized and represent the intended schema.

  **Priority**: P0 (critical structural replacement for Chatwoot retirement mandate).
  **Estimated Scope**: Medium.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
