issue_title: "Build Native Rust Omnichannel Chat System for OHC (WhatsApp & Web Widget)"
issue_description: |
  **Problem Statement**
  OHC must replace external dependencies on Chatwoot and implement a native omnichannel chat system inside our Rust microservices. The owner/operator (e.g. Maya the baker or Carlos the field tech) needs to interact with customers via WhatsApp and Web Chat directly inside OHC without context switching to another app. It should natively integrate with the OHC Assistant to manage responses and context.

  **Research Report**
  As per the `P0` mandate, Chatwoot integration is retired. I've cloned and examined the open-source Chatwoot repository to understand its data model and features.

  Chatwoot supports numerous channels (WhatsApp, Web Widget, Email, Facebook, Instagram, SMS, Telegram, Line, TikTok, Twitter).
  For our MVP MVP native implementation, the highest impact channels for OHC operators are **WhatsApp** and a **Web Widget**.

  Chatwoot's conversation model includes:
  - Account/Tenant ID
  - Contact ID
  - Inbox ID
  - Status (open, snoozed, resolved)
  - Priority
  - Assignee

  Channel configurations like WhatsApp require:
  - Phone Number
  - Provider Config (credentials, e.g. for WhatsApp Cloud API)
  - Message Templates

  **Design Doc**
  We will introduce a new Rust microservice (or extend the existing API service) with a native Omnichannel Chat Engine.

  1.  **Data Model**:
      -   **`inboxes`**: Represents a channel endpoint (e.g., "Maya's Bakery WhatsApp", "Website Support Widget").
      -   **`conversations`**: Groups messages between a customer contact and an inbox. Contains status and assignment.
      -   **`messages`**: The individual text/media items.
      -   **`channel_whatsapp`**: Stores credentials and templates for WhatsApp API.
      -   **`channel_web_widget`**: Stores configuration for the embeddable web chat.

  2.  **System Flow**:
      -   **Ingestion**: Webhooks from WhatsApp Cloud API or events from the Web Widget hit our API.
      -   **Processing**: The API creates/updates a `conversation` and `message` in PostgreSQL.
      -   **Coordination**: The event is pushed to the AI Job Queue for the "Customer & Relationship Assistant" to evaluate and optionally draft a reply.
      -   **Owner View**: The "Work Triage" frontend feed shows the new message. The owner can review AI drafts or reply manually.
      -   **Egress**: Replies are routed back through the appropriate channel provider.

  **Implementation Prompt**
  Implement the database schema, API endpoints, and internal event routing for the native OHC omnichannel chat system.

  *Acceptance Criteria:*
  - Non-technical owners can connect a WhatsApp business number and configure a web chat widget via the OHC UI.
  - Incoming messages from these channels appear unified in the OHC Work Triage feed.
  - The OHC AI Assistant can draft replies to these messages.
  - Owners can send manual replies that route correctly to the customer's WhatsApp or Web Widget.
  - Provide complete E2E tests validating a conversation lifecycle from webhook ingestion to owner reply.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
