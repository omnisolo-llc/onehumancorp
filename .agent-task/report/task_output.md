issue_title: "Build Native Chatwoot Replacement in Rust"
issue_description: |
  # Native Chatwoot Replacement in Rust

  ## Problem Statement
  We are retiring external Chatwoot services and replacing them with our own high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust inside `onehumancorp/mono`. This new module will provide the backend functionality previously handled by an external Chatwoot installation, allowing OHC to fully control and coordinate customer chat and support messages directly.

  ## Research Report
  - **Goal:** To construct a robust, native Rust chat and customer support service inside OHC.
  - **Source Analysis:** We cloned and analyzed Chatwoot's source code at https://github.com/chatwoot/chatwoot. Chatwoot provides an open-source omni-channel customer support solution. Its core includes managing conversations, messages, channels (like Web Widget, API, Facebook, Twitter, WhatsApp), contacts, agents, and teams. The system relies heavily on webhooks for external integrations and WebSockets for real-time updates. Looking at `app/models`, the core domain objects are Conversation, Message, Contact, User, AgentBot, Inbox, Channel, AutomationRule, CannedResponse, etc.
  - **Findings:** To match core functionality, OHC needs a native module that handles:
    - **Conversations & Messages:** Tracking chat threads and individual messages.
    - **Channels/Inboxes:** Interfaces for different sources (Web Widget, Email, API, WhatsApp, etc.).
    - **Contacts & Agents:** Managing the entities involved in the conversations (customers and staff).
    - **Real-time Events:** The backend needs a mechanism to broadcast message updates and conversation state changes in real-time.

  ## Design Doc
  - **Architecture:** We will create a new Rust module `chat` under `src/server/integrations/chat` (as a starting point, recognizing this is internal but fits the modular structure, or potentially as a core service `src/server/chat`).
  - **Data Models:** Define entities for managing conversations, messages, contacts, and inboxes, ensuring multi-tenant isolation.
  - **API Capabilities:** Provide functionality for creating/fetching conversations, sending/retrieving messages, and managing contacts/inboxes.
  - **Real-time Updates:** Integrate with OHC's existing PubSub or NATS infrastructure to broadcast message events, allowing the frontend to update in real-time.

  ## Implementation Prompt
  1. Create the `chat` module directory structure.
  2. Implement the entities for conversation, message, inbox, and contact with multi-tenant support.
  3. Define and implement the service layer for chat functionality.
  4. Ensure all database operations strictly filter by tenant ID for row-level security.
  5. Add unit tests for the core logic and database interactions to achieve 100% coverage for the new module.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
