issue_title: "[Research] OHC Native Rust Chat Engine Architecture"
issue_description: |
  ## Problem Statement
  OneHumanCorp currently has a dependency on external/third-party services for omnichannel messaging, and the mandate is to fully retire Chatwoot and build a native, high-performance omnichannel chat system in Rust. Small business owners (like Maya the baker and Carlos the handyman) need unified communications (Instagram DMs, WhatsApp, SMS, Web Chat) seamlessly integrated into their daily work feeds without complex external integrations or latency.

  ## Research Report
  Based on an audit of the `chatwoot` open-source repository:
  - **Core Entities:** Account (tenant), Inbox, Conversation, Message, Contact, Channel (WebWidget, API, Email, FB/IG, WhatsApp, SMS, Line, Telegram, etc.), AgentBot, AutomationRule.
  - **Data Model:** Highly relational, heavily tenant-scoped (account_id). Conversations aggregate messages from a specific contact on a specific inbox/channel.
  - **Real-time:** WebSockets (ActionCable in Ruby) push events to the frontend.
  - **Extensibility:** Webhooks and AgentBots for automated replies.

  ### Comparison: Chatwoot (Ruby) vs. OHC Native (Rust)
  - **Performance:** Rust will drastically reduce latency and memory footprint compared to Ruby.
  - **Concurrency:** Rust's async model is ideal for handling thousands of concurrent WebSocket connections for real-time messaging, a critical requirement for a responsive unified inbox.
  - **Integration:** A native Rust engine integrates directly with OHC's AI agents, allowing them to intercept, analyze, and draft replies instantly without traversing external APIs.

  ## Design Doc

  ### Architecture
  The Chat Engine will be a native Rust microservice (or integrated into the main Rust monolith) communicating via gRPC internally and exposing WebSocket/REST endpoints to the frontend.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ CONTACT : has

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string provider_type
          json credentials
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string identifier
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          text content
          string sender_type
          uuid sender_id
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Feed:** The home screen features a prioritized "Work Triage" feed. New messages appear as actionable cards.
  2. **Conversation View:** Tapping a card opens a full-screen chat interface.
  3. **Smart Replies:** Above the keyboard, AI-generated smart reply drafts are presented as chips.
  4. **Context Switcher:** A simple toggle (or swipe) reveals the customer's profile, past orders, and notes without leaving the chat context.

  ### AI Agent Integration
  - **Customer Assistant Agent:** Listens to new `MessageCreated` events on the event bus. If it's a customer query, it retrieves context from the Memory/Knowledge agent and drafts a reply, which is inserted as a pending action in the Work Triage feed.

  ## Implementation Prompt
  **Goal:** Implement the foundational data models, repositories, and a basic gRPC/REST API for the native Rust Chat Engine to replace Chatwoot.
  **Persona:** Maya (Baker) needs to see all her Instagram DMs and Web Chat inquiries in one place, instantly.
  **Acceptance Criteria:**
  - Define protobuf schemas for `Tenant`, `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
  - Implement Rust structs and PostgreSQL migrations (with RLS for `tenant_id`) for these entities.
  - Implement basic CRUD gRPC endpoints for these entities.
  - Ensure 100% unit test coverage for the repository layer.
  - Ensure all database queries strictly enforce `tenant_id` isolation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
