issue_title: "[Research] OHC Native Rust Chat System - Architecture & Data Model"
issue_description: |
  ## Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support & chat engine. The goal is to completely retire external Chatwoot dependencies and build a matching native Rust microservice, crates, and frontend UI components in OHC to achieve 100% feature parity. The system must support core entities like Contacts, Inboxes, Conversations, Messages, and ContactInboxes, while guaranteeing strict multi-tenant isolation and working beautifully on a 375px mobile screen.

  ## Research Report
  - We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its data model and architecture.
  - The core entities are:
    - `Contact`: Represents a user or customer interacting with the business.
    - `Inbox`: Represents a channel (e.g., Email, SMS, Web Widget, WhatsApp) configured for a specific business/tenant.
    - `Conversation`: Represents a thread of messages between a Contact and Agents within an Inbox.
    - `Message`: Represents an individual message within a Conversation.
    - `ContactInbox`: Represents the association between a Contact and a specific Inbox, storing channel-specific identifiers (e.g., phone number for SMS, email address, external source ID).
  - Multi-tenancy in Chatwoot is primarily handled via the `account_id` field on almost all entities. In OHC, this translates to our `tenant_id` pattern.
  - Chatwoot utilizes a robust event-driven architecture, including ActionCable (WebSockets) for real-time updates and ActiveJob for background processing (e.g., webhook delivery, automation execution).

  ## Design Doc
  ### Data Model (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : "has"
      TENANT ||--o{ CONTACT : "has"
      TENANT ||--o{ CONVERSATION : "has"
      TENANT ||--o{ MESSAGE : "has"

      CONTACT ||--o{ CONTACT_INBOX : "participates in"
      INBOX ||--o{ CONTACT_INBOX : "receives from"

      CONTACT ||--o{ CONVERSATION : "initiates"
      INBOX ||--o{ CONVERSATION : "hosts"

      CONVERSATION ||--o{ MESSAGE : "contains"

      INBOX {
          uuid tenant_id FK
          uuid id PK
          string name
          string channel_type
          jsonb settings
      }

      CONTACT {
          uuid tenant_id FK
          uuid id PK
          string name
          string email
          string phone_number
      }

      CONTACT_INBOX {
          uuid tenant_id FK
          uuid id PK
          uuid contact_id FK
          uuid inbox_id FK
          string source_id "Channel specific ID"
      }

      CONVERSATION {
          uuid tenant_id FK
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          uuid assignee_id "Agent ID"
      }

      MESSAGE {
          uuid tenant_id FK
          uuid id PK
          uuid conversation_id FK
          string content
          string sender_type "contact, agent, bot"
          uuid sender_id
      }
  ```

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      Client(Mobile App / Web) --> API(Rust gRPC/REST API)
      Client --> WS(Rust WebSocket Server)

      API --> Auth(Auth/Tenant Context via SPIFFE)
      WS --> Auth

      Auth --> Controller(Chat Service Controllers)

      Controller --> DB[(PostgreSQL)]
      Controller --> Queue[(Redis/Postgres Job Queue)]

      Queue --> Worker(Rust Background Workers)
      Worker --> Integrations(External Channels: SMS, Email, WA)
      Worker --> AI(AI Agents)
  ```

  ### Mobile UX Flow (375px first)
  1. **Inbox List**: A unified view of all open conversations, prioritized by recent activity or urgency. Each item shows the contact's avatar, channel icon (e.g., IG, SMS), and the latest message snippet.
  2. **Conversation View**: A standard chat interface. Messages are bubbled. Clear indication of who sent the message (Contact, Agent, or AI). Sticky input area with quick reply buttons (AI drafted).
  3. **Contact Details Drawer**: A slide-over panel on the right (or a separate screen on mobile) showing contact info, past orders, and tags.

  ### AI Agent Integration Points
  - **Auto-Drafting**: When a new message arrives, an AI agent can analyze the intent and draft a reply for the human agent to review and send.
  - **Triage**: An AI agent can automatically categorize and assign conversations based on content (e.g., routing billing questions to a specific queue).
  - **Summarization**: An AI agent can generate a brief summary of a long conversation for a human agent taking over the thread.

  ## Implementation Prompt
  Implement the core Rust data models and database migrations for the new OHC omnichannel chat system, mirroring the capabilities of Chatwoot but adhering to OHC's architecture.
  1. Define the PostgreSQL schema (DDL) for Inboxes, Contacts, ContactInboxes, Conversations, and Messages. Ensure `tenant_id` is present on every table and Row Level Security (RLS) is enabled.
  2. Implement the corresponding Rust struct definitions (Entities) with proper serialization/deserialization.
  3. Provide basic CRUD operations via Rust repository patterns for these entities.
  4. Write comprehensive unit tests validating the data models and multi-tenant constraints.

  **Acceptance Criteria:**
  - Migrations execute successfully.
  - Rust models compile and accurately represent the schema.
  - Unit tests achieve 100% coverage for the new repository layer.
  - The design fully supports the omnichannel relationships described in the research report.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
