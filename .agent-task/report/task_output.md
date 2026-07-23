issue_title: "Native Rust Omnichannel Chat & Support Engine"
issue_description: |
  # Problem Statement
  OHC currently relies on external Chatwoot services for its omnichannel customer support and chat functionality. This dependency is being 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel chat engine natively in Rust inside `onehumancorp/mono` to ensure seamless mobile-first integration, zero-trust security, and tight coordination with AI Agents. For our non-technical owner/operators (e.g., Maya, Carlos), they expect a unified inbox where Instagram DMs, SMS, and WhatsApp messages effortlessly combine with tasks, bookings, and payments, coordinated by the AI work assistant without manual switching between tools.

  # Research Report
  Based on an audit of Chatwoot's source repository (`https://github.com/chatwoot/chatwoot`), the core architecture requires several primary models mapping to channels, inboxes, and conversations:
  - **Inbox**: Handles channel routing, business information, and auto-assignment.
  - **Conversation**: Ties messages to a specific contact and inbox, tracking status (open/resolved), priority, and assignee.
  - **Message**: The granular unit of communication, including text content, attachments, status, and sentiment analysis.
  - **Contact**: Represents the customer (visitor, lead) across channels with unified identifier tracking.
  - **Channel Adapters**: Diverse integrations handling web widgets, Facebook/Instagram, SMS, and Whatsapp.

  Competitor SaaS architectures demonstrate that a highly-scalable chat system needs real-time WebSocket messaging, persistent storage with strict multi-tenancy, and distributed queue processing for webhook events.

  # Design Doc

  ## Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : routes
      CHANNEL_ADAPTER ||--o| INBOX : provides
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ CONTACT : manages

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          boolean enable_auto_assignment
      }
      CHANNEL_ADAPTER {
          uuid id
          string type
          jsonb config
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string sender_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string identifier
          string email
      }
  ```

  ## AI Agent Integration Points
  - **Customer & Relationship Assistant**: Listens to new `CONVERSATION` and `MESSAGE` creation events. Uses tenant context to draft contextual replies for chat, Instagram, SMS, etc.
  - **Work Triage (Operations Assistant)**: Groups incoming inquiries overnight (e.g., Maya's cake requests). Turns unhandled DMs into prioritized action items in the owner feed.
  - **Background Automation**: An AI queue observes WebSockets/Webhooks and enqueues tasks based on message sentiment or missing SLA.

  ## Mobile UX Flow
  - **375px First Focus**: The Unified Inbox is rendered as a clean Ubiquiti UniFi-style card list on mobile.
  - **Owner Feed**: A single tap on an incoming message card opens a translucent glass modal that shows the customer's history, previous bookings, and AI-drafted reply.
  - **Interactions**: No complex routing rules are exposed to the user. Tapping "Approve Reply" automatically dispatches the message via the native backend.

  ## Key Design Decisions
  - **Row-Level Tenant Isolation**: All models (`inboxes`, `conversations`, `messages`, `contacts`) enforce `tenant_id` at the PostgreSQL level via RLS.
  - **Zero Trust & Security**: Identity uses SPIFFE/SPIRE, ensuring every background sync job or web socket push is mTLS validated.
  - **Native Rust Execution**: Moving away from external API dependence removes latency and potential sync conflicts, keeping the UX responsive on low-end devices and flaky networks.

  **Estimated Scope**: Large

  # Implementation Prompt
  Build the native Rust data models and service layer for the OHC Omnichannel Chat Engine. Create the PostgreSQL schemas with Row-Level Security on `tenant_id` for Inboxes, Conversations, Messages, and Contacts. Implement a unified REST+JSON API layer for the frontend to fetch the prioritized inbox feed. Ensure the service triggers background AI events upon message receipt. Verify the mobile-first UX with full E2E Playwright tests using realistic data (no mocks) from an owner persona's perspective (e.g., replying to a customer inquiry).
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []