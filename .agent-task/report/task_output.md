issue_title: "[Research] Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ### Title
  Native Rust Omnichannel Chat System to Replace Chatwoot

  ### Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot for omnichannel customer support. This external dependency introduces operational overhead, architectural mismatch (Ruby on Rails vs. Rust), and violates our native-first and multi-tenant row-level isolation principles. For our core personas (e.g., Maya the baker, Carlos the handyman), communication must be seamlessly woven into their core operations (bookings, quotes, orders) without jumping to a separate system. OHC needs a bespoke, high-performance, real-time messaging system natively implemented in Rust, giving owners a unified inbox integrated deeply with AI agents.

  ### Research Report
  - **Chatwoot Source Audit:** We cloned and analyzed `https://github.com/chatwoot/chatwoot`. Core operational models include `Inbox`, `Conversation`, `Message`, `Contact`, and various `Channel` adapters. Chatwoot relies on Rails ActionCable for WebSocket events and complex background jobs for message processing.
  - **Architectural Gaps:** Chatwoot’s database schema fundamentally breaks OHC’s strict multi-tenant row-level security requirement (`tenant_id` required on every table). Furthermore, OHC's requirement for AI agents (Operations, Customer Service, Sales) to seamlessly read and mutate conversation context is hindered by an external API boundary.
  - **Competitive Insights:** Platforms like Shopify Sidekick or Wix Inbox natively integrate messaging into their commerce backends. A native system in OHC allows us to effortlessly convert an Instagram DM directly into a custom order quote within the same transaction and context window.

  ### Design Doc

  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  erDiagram
      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string identifier
          string name
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string message_type
          uuid sender_id
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          string provider
      }

      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
  ```

  **Mobile UX Flow (375px baseline)**
  1. **Unified Inbox List Screen:**
     - A clean list of conversations grouped by status (Unread, Assigned, Resolved).
     - Each item displays the customer's avatar, name, channel icon (WhatsApp, Instagram, Web), a snippet of the last message, and a timestamp.
     - Clean Ubiquiti UniFi modular dashboard card layouts.
  2. **Conversation Thread Screen:**
     - Tapping a conversation opens the chat view.
     - A sticky bottom input composer integrated with the native mobile keyboard.
     - The top header shows the contact name and quick operational actions (e.g., "Create Quote", "Book Appointment").
  3. **AI Agent Assist Overlay:**
     - An "AI Draft" button in the composer instantly generates a suggested reply based on business context and customer history.
     - Displayed via macOS-style Translucent Glass overlay.
  4. **Operations Handoff Flow:**
     - Long-pressing a customer message allows the owner to convert it into a task, order, or booking seamlessly without leaving the chat interface.

  **AI Agent Integration Points**
  - **Customer Assistant:** Automatically drafts context-aware replies to incoming messages (e.g., "Do you do vegan cakes?").
  - **Operations & Sales Assistant:** Extracts intent from messages to suggest creating bookings, updating orders, or sending payment links.
  - **Work Triage Queue:** Analyzes incoming message urgency and dynamically prioritizes them in the owner's daily feed (e.g., highlighting a missed lead).

  **Key Design Decisions**
  - **Native Rust Implementation:** Build the core logic as high-performance Rust services utilizing modern WebSocket architectures for instant message delivery.
  - **Strict Multi-Tenant Isolation:** Enforce `tenant_id` on all tables (`inboxes`, `conversations`, `messages`, `contacts`) integrated with PostgreSQL Row Level Security (RLS).
  - **Unified Domain Connectivity:** Align the chat domain tightly with OHC's operational models to allow zero-friction cross-functional actions (Chat -> Quote -> Payment).

  ### Implementation Prompt
  **Objective:** Implement the backend Rust services and Flutter frontend components for a native Omnichannel Chat System, completely replacing Chatwoot dependencies.
  **User-Facing Outcome:** Owners can view a unified inbox and reply to customer inquiries (from Web, Instagram, WhatsApp) directly from the OHC mobile app. The AI assistant can draft replies for these messages.
  **Acceptance Criteria:**
  - Create Rust backend schemas and gRPC/REST APIs for Inboxes, Contacts, Conversations, and Messages, strictly enforcing `tenant_id` RLS.
  - Implement real-time WebSocket connection handling for instant message delivery and receipt.
  - Build Flutter UI screens for the Unified Inbox List and Conversation Thread, verified on a 375px display viewport, utilizing OHC's translucent glass design tokens.
  - Add an AI draft generation endpoint integrated with the Customer Assistant to suggest replies contextually.
  - Write Playwright E2E tests verifying a message can be received via API, displayed via WebSocket in the Flutter UI, and replied to.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
