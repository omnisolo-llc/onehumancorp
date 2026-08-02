issue_title: "Implement Native Rust Omnichannel Chat Models (Replace Chatwoot)"
issue_description: |
  ## OHC Native Rust Omnichannel Chat Architecture

  ### 1. Business Journey Mapping
  **Target Persona:** Maya (Baker) & Carlos (Handyman)
  **User Journey:**
  1. **Acquisition:** A customer sends a DM on Instagram or WhatsApp to Maya, asking "Do you do vegan cakes?"
  2. **Onboarding:** The message arrives instantly in Maya's OHC app.
  3. **Activation:** Maya sees a consolidated "Inbox" regardless of the channel (WhatsApp, IG, Email, Web Widget).
  4. **Retention:** The OHC AI Assistant intercepts the message and drafts a response automatically ("Yes, we do! Here is the pricing...").
  5. **Revenue:** The customer clicks a payment link sent via the chat to pay the deposit.

  ### 2. Data Model & Invariants

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CHANNEL : has
      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL ||--o{ INBOX : configures

      TENANT {
          uuid id PK
          string name
      }

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean greeting_enabled
          string channel_type
      }

      CHANNEL {
          uuid id PK
          uuid tenant_id FK
          string provider "e.g. Whatsapp, Instagram, Twilio"
          jsonb credentials "Encrypted API keys"
      }

      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }

      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          datetime created_at
      }

      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          text content
          string sender_type "agent, contact, bot"
          datetime created_at
      }
  ```

  ### 3. Multi-Tenant Isolation & Zero Trust
  - Every table MUST have a `tenant_id` column.
  - Row Level Security (RLS) MUST be enabled on all tables in PostgreSQL.
  - SeaORM Entity configurations must implicitly inject `WHERE tenant_id = ?` into all read/write operations via a shared DB connection pool wrapper.
  - Inter-agent coordination (e.g., the AI Assistant drafting a reply) uses SPIFFE/SPIRE identity tokens to verify the agent's authority to act on behalf of the `tenant_id`.

  ### 4. AI Department Coordination
  ```mermaid
  sequenceDiagram
      participant Webhook as Channel Webhook (IG/WA)
      participant API as OHC Rust API
      participant DB as Postgres (SeaORM)
      participant AI as AI Customer Support Agent
      participant Mobile as OHC Flutter App

      Webhook->>API: POST /webhooks/instagram (Message)
      API->>DB: Lookup Tenant & Channel
      API->>DB: Insert Contact & Conversation (if new)
      API->>DB: Insert Message (sender: contact)
      API->>AI: Trigger "Draft Reply" Job
      API->>Mobile: WebSocket Push (New Message)
      AI->>DB: Read Conversation History
      AI->>DB: Insert Message (sender: bot, status: draft)
      AI->>Mobile: WebSocket Push (Draft Ready)
      Mobile->>API: Approve Draft
      API->>Webhook: Send Reply to Customer
  ```

  ### 5. Mobile-First UX Flow (375px)
  - **Bottom Navigation:** Inbox, Calendar, Customers, Settings.
  - **Inbox List:** Unified view showing contact name, channel icon (IG, WA), snippet, and unread dot.
  - **Conversation View:** Standard chat bubbles. The AI draft appears as a glowing, translucent bubble at the bottom with a primary "Send" button and secondary "Edit" button. Touch targets must be 44x44px minimum.

  ### 6. Implementation Prompt
  **Task:** Implement the native Rust Omnichannel Inbox and Conversation SeaORM models, replacing the retired Chatwoot dependency.
  **Requirements:**
  - Implement `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` entities using SeaORM.
  - Ensure strict multi-tenant RLS isolation in database migrations and SeaORM queries.
  - Implement the WebSocket event dispatcher for real-time mobile updates when messages arrive.
  - Do NOT mock the database or API calls in your E2E tests. Add a Playwright E2E test verifying a user can receive a message and approve an AI draft.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
