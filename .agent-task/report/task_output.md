issue_title: "Architecture & Implementation Plan: Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp previously relied on an external third-party service for omnichannel customer support and chat functionality. Relying on an external Ruby on Rails monolith creates latency, limits our multi-tenant row-level security architecture, complicates Zero-Trust deployments (SPIFFE/SPIRE), and fractures the experience for our core owner/operator personas (Maya, Carlos, Priya). To deliver a unified, assistant-first experience where AI agents can seamlessly intercept, triage, and draft replies to customer messages across Instagram, WhatsApp, and Web, OHC must natively implement a high-performance, multi-tenant omnichannel chat engine in Rust.

  ## Research Report
  **Source Code Audit Findings:**
  - **Data Models:** The previous external system uses a hierarchical structure centered around `Account` (tenant), `Inbox`, `Conversation`, `Message`, and `Contact`. It supports polymorphic `Channel` adapters (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Api`).
  - **Real-time Messaging:** WebSockets broadcast events like `message.created` and `conversation.updated` to connected clients (both agents and customers).
  - **AI & Automation:** Agents/Bots, automation rules, and macros trigger off message and conversation lifecycle events.
  - **Competitor Benchmarking:** Shopify Inbox, Wix Inbox, and Stripe's unified messaging all natively integrate chat into their core data graphs. A native Rust implementation in OHC allows us to bind conversations directly to OHC `Orders`, `Appointments`, and `Customers` without brittle webhook synchronizations.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL_ADAPTER ||--o{ INBOX : provides
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ### Architecture Diagram - System Level
  ```mermaid
  sequenceDiagram
      participant Customer (Web/IG/WA)
      participant Rust Chat Service
      participant PostgreSQL (RLS)
      participant Redis PubSub
      participant OHC AI Agent (Work Triage)

      Customer (Web/IG/WA)->>Rust Chat Service: Send Message
      Rust Chat Service->>PostgreSQL (RLS): Persist Message
      Rust Chat Service->>Redis PubSub: Publish `message.created`
      Redis PubSub->>OHC AI Agent (Work Triage): Trigger Triage & Draft
      OHC AI Agent (Work Triage)->>Rust Chat Service: Save Draft Reply
      Rust Chat Service->>Customer (Web/IG/WA): Broadcast UI Update
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Work Command Center (Home):** The top card displays "3 Needs Attention". Tapping it enters the unified Inbox.
  - **Unified Inbox List (375px):** Clean list of conversations, combining Instagram, WhatsApp, and Web. Avatars show the source icon (e.g., small WhatsApp logo in corner). Unread dots and "AI Drafted" badges are prominent.
  - **Conversation View:** Standard chat bubbles. If an AI draft exists, it appears in a translucent glass container at the bottom above the native mobile keyboard, with a one-tap "Approve & Send" or "Edit" button.
  - **Visual Design:** macOS-style Translucent Glass materials for the header and AI draft containers. Clean UniFi modular dashboard card layouts for contact info drawers (accessible via a swipe or top-right info icon).

  ### AI Agent Integration Points
  - **Work Triage:** Listens to `message.created` events via Redis PubSub, evaluates urgency, and groups related inquiries.
  - **Customer Assistant:** Automatically drafts replies based on tenant context (e.g., store hours, inventory) and saves them as `Draft` messages in the Rust service.
  - **Operations Assistant:** Parses intents (e.g., "book an appointment") and links the conversation to actionable OHC tasks.

  ### Key Design Decisions
  - **Rust for Performance & Safety:** Leveraging Rust (`tokio`, `axum`) ensures minimal memory footprint, memory safety, and high-throughput WebSocket handling for real-time chat.
  - **PostgreSQL Row-Level Security (RLS):** Every table (`inboxes`, `conversations`, `messages`) will have `tenant_id` and enforce RLS to guarantee strict data isolation.
  - **Polymorphic Channels:** The channel implementation will use Rust traits to define a standard interface for receiving and sending messages, allowing easy addition of new platforms (e.g., Messenger, SMS).

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the core native Rust Chat microservice replacing the external dependency.
  1. Define the PostgreSQL schema for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` is present for RLS.
  2. Implement a Rust web service (e.g., using `axum`) with REST endpoints for creating/fetching messages and a WebSocket endpoint for real-time updates.
  3. Integrate Redis PubSub to broadcast events when new messages arrive.
  4. Build the Flutter frontend unified inbox view (mobile-first, 375px) that connects to the Rust WebSocket and displays conversations with "AI Draft" overlays using our Translucent Glass design tokens.
  5. Ensure the Critical User Journey (CUJ) passes: A customer sends a message via a mock channel, the AI drafts a reply, and the owner sees the draft in the mobile UI and approves it.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
