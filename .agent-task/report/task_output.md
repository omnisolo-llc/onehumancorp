issue_title: "Native Rust Omnichannel Chat: SLA Policies, Automation Rules, and Real-time WebSockets"
issue_description: |
  # Native Rust Omnichannel Chat System Parity

  ## Problem Statement
  We have fully retired the external Chatwoot dependency, per the OHC product mandate, to ensure absolute multi-tenant data isolation and a seamless unified interface for the owner/operator. While we have implemented basic inbox, conversation, and message models in the `src/server/services/chat` layer, we are missing the core capabilities that transform a chat feed into an automated workspace. Currently, Maya (the baker) cannot set an auto-reply rule for after-hours, and Jun (the manager) cannot track SLA breaches when his staff ignores messages.

  ## Research Report
  Reviewing the original Chatwoot architecture (`app/models` and `app/controllers` from `https://github.com/chatwoot/chatwoot`), the true power of their omnichannel system comes from:
  1. **SLA Policies (`app/models/sla_policy.rb`)**: Defining first-response time and resolution time targets.
  2. **Automation Rules (`app/models/automation_rule.rb`)**: Event-driven triggers (e.g., `message_created`, `conversation_created`) running condition checks to perform actions (e.g., assign to team, add label, send email).
  3. **Canned Responses / Macros (`app/models/canned_response.rb`, `macro.rb`)**: Enabling quick replies for operators.
  4. **Real-time WebSockets / ActionCable (`app/channels`)**: Broadcasting typing indicators, presence, and new messages instantly.
  5. **Agent Bot Integrations (`app/models/agent_bot.rb`)**: Allowing AI to intercept and handle conversations before human assignment.

  We need to replicate these features natively in our Rust stack.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Omnichannel Inbox] -->|WebSocket| B(Chat Service - Rust)
      B --> C{Event Bus}
      C -->|conversation_created| D[Automation Engine]
      C -->|message_created| D
      D --> E[SLA Tracker]
      D --> F[Auto-Responder Agent]
      C -->|broadcast| G[WebSocket Manager]
      G -->|Push| H[Owner Dashboard UI]
  ```

  ### Mobile UX Flow (375px)
  - **Inbox View**: Clean, full-width conversation list.
  - **Conversation View**: Sticky header with back button and contact name. Bottom sticky input field with a "+" button to access Canned Responses (Translucent glass sheet pops up).
  - **Automation Settings**: Hidden under the "Advanced Settings" tab for the specific inbox, allowing Maya to set business hours and away messages without seeing JSON.

  ### AI Agent Integration
  - When a message arrives, the Automation Engine can trigger the OHC AI Assistant to draft a reply or handle the conversation entirely (e.g., answering "Do you have vegan cakes?").
  - The AI acts as a `sender_type = 'bot'` in the `chat_messages` table, seamlessly handing off to `sender_type = 'agent'` if confidence is low.

  ### Key Design Decisions
  - **Rust Native**: Built entirely within `src/server/services/chat`.
  - **RLS Enforced**: All new tables (e.g., `chat_sla_policies`, `chat_automation_rules`) must strictly use the `tenant_id` pattern.
  - **WebSocket over gRPC/REST**: Use Axum WebSockets or our existing sync mechanisms to push state to the Tauri frontend.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build out the next phase of the Native Rust Omnichannel Chat system.
  1. Add database migrations for `chat_sla_policies`, `chat_automation_rules`, and `chat_canned_responses`.
  2. Implement the CRUD operations in `src/server/services/chat/service.rs`.
  3. Wire up an event dispatcher in the chat service that can evaluate automation rules when a new message or conversation is created.
  4. Build the UI components in Tauri (`src/ui/tauri`) to manage canned responses and view SLA statuses in the conversation view.
  5. Ensure 100% test coverage and verify the flow using Playwright, simulating Maya setting up a canned response and using it in a chat.

  **Acceptance Criteria:**
  - Migrations run successfully.
  - Unit tests cover the automation rule evaluator.
  - Playwright E2E test proves a user can create a canned response and see it in the chat interface.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
