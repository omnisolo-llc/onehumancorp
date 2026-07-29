issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Title: Implement Custom Rust Omnichannel Chat System to Replace Chatwoot

  ## Problem Statement
  As per the OHC Engineering Standards, the Chatwoot external dependency is 100% RETIRED. The `src/` directory shows that OHC currently uses external webhooks to simulate omnichannel capabilities (e.g., `/api/v1/omnichannel/webhook`) but lacks a comprehensive, native implementation to manage omnichannel interactions. Maya (the baker) and Carlos (the handyman) need a unified inbox that integrates seamlessly with their workflow without relying on a disconnected third-party tool.

  ## Research Report
  Based on auditing the Chatwoot source code (`app/models/message.rb`, `app/models/conversation.rb`, `app/models/inbox.rb`), OHC's native implementation requires core data models for Inboxes, Conversations, Messages, and Contacts. Leading competitors (like Shopify Inbox) provide a single API gateway to ingest multi-channel messages and an AI-driven routing engine to assign them.

  ## Design Doc
  1. **Data Models (PostgreSQL + RLS)**:
     - **Inboxes**: Defines the entry points for channels (email, web widget, WhatsApp, Instagram).
       - Attributes: `account_id` (mapped to OHC's `tenant_id`), `name`, `channel_type`, `auto_assignment_config`.
     - **Conversations**: Groups messages between a contact and agents.
       - Attributes: `tenant_id`, `inbox_id`, `contact_id`, `assignee_id` (human or AI bot), `status` (open, resolved, snoozed), `priority`.
     - **Messages**: Individual messages within a conversation.
       - Attributes: `tenant_id`, `conversation_id`, `inbox_id`, `message_type` (incoming, outgoing, internal note), `content`, `content_type`, `sender_type`.
     - **Contacts**: Omnichannel customer profiles.

  2. **Microservices (Rust in `src/server/ohc/omnichannel/`)**:
     - **Ingestion Gateway**: A WebSocket/webhook API gateway that handles real-time messages from multiple channels.
     - **Routing Engine**: AI-driven assignment rules that match inbound conversations with specific human or AI agents (e.g., Operations, Sales).
     - **Integration Layers**: Adapters to external messaging networks (WhatsApp, IG, SMS via Twilio).

  3. **Architecture Diagram**:
  ```mermaid
  erDiagram
      INBOXES ||--o{ CONVERSATIONS : "has"
      CONTACTS ||--o{ CONVERSATIONS : "participates in"
      CONVERSATIONS ||--o{ MESSAGES : "contains"

      INBOXES {
          uuid tenant_id
          uuid id
          string name
          string channel_type
      }

      CONVERSATIONS {
          uuid tenant_id
          uuid id
          uuid inbox_id
          uuid contact_id
          uuid assignee_id
          string status
      }

      MESSAGES {
          uuid tenant_id
          uuid id
          uuid conversation_id
          string content
          string message_type
      }

      CONTACTS {
          uuid tenant_id
          uuid id
          string name
          string phone
          string email
      }
  ```

  4. **Mobile UX Flow (375px Target)**:
     - **Unified Inbox Feed (Home)**: A bottom navigation tab showing grouped conversations by priority. Tapping a conversation opens the chat view.
     - **Chat View (Thread)**: A full-screen overlay (375px width, flexible height).
       - Top bar: Customer name, channel icon (e.g., IG, WhatsApp), back button.
       - Middle: Scrollable list of messages (Customer messages on left, agent/owner replies on right in OHC premium tokens).
       - Bottom: Sticky input area with native keyboard support and action buttons (e.g., "Draft Proposal", "Request Payment").
     - **Context Pane (Swipe/Drawer)**: A right-side drawer (or swipe-left action) revealing customer tags, order history, and AI-suggested next steps.

  ## AI Agent Integration
  The Routing Engine will hand off context to specific agents (Operations, Customer Success) natively using OHC's internal event bus instead of relying on external SLA configurations.

  ## Implementation Prompt
  1. Set up the PostgreSQL schema for `inbox_conversations`, `inbox_messages`, `inbox_channels`, and `inbox_contacts` with `tenant_id` for row-level security.
  2. Develop a Rust service under `src/server/ohc/omnichannel` handling WebSocket connections for real-time delivery and webhook endpoints for inbound ingestion.
  3. Replicate the routing logic to auto-assign conversations based on AI/human rules.
  4. Build a Flutter/Next.js frontend component to replace any mock chat interfaces.
  5. Validate using Playwright with end-to-end multi-channel simulations.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
