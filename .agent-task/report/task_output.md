issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is officially retiring Chatwoot as an external third-party dependency. To ensure we provide our non-technical owner/operator personas (Maya, Carlos, Priya, Leo, Fatima) with a secure, highly-performant, and unified "Work Triage" experience, we must implement our own omnichannel chat system natively in Rust within the `onehumancorp/mono` repository.

  The current native chat implementation in `src/server/services/chat/service.rs` is too primitive. It lacks real-time WebSocket capabilities, SLA policies, unified inbox abstractions, and true channel adapters (WhatsApp, IG, Email, Web Widget).

  ## Research Report
  I cloned and audited the official Chatwoot repository (`https://github.com/chatwoot/chatwoot`) to benchmark their Ruby-on-Rails architecture against our Rust stack:
  1. **Data Models**: Chatwoot relies on heavily decoupled models like `Account` (Tenant), `Inbox`, `Channel::*`, `Contact`, `Conversation`, and `Message`.
  2. **Real-time Pipeline**: Uses ActionCable for WebSocket events (`message.created`, `conversation.read`, etc.). We will replicate this using Rust's `axum::extract::ws`.
  3. **AI / Automation**: Relies on `AgentBot`, `CannedResponse`, and `Macro`. We can vastly improve this by deeply integrating our OHC AI Job Queue, allowing our specialized AI Assistants (Customer, Operations, Sales) to act as silent co-pilots in the `Conversation` thread, drafting replies for the owner.
  4. **Multi-tenancy**: Chatwoot isolates by `account_id`. We will use our standard PostgreSQL Row-Level Security (RLS) with `tenant_id`.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      MobileApp[Flutter / Next.js PWA 375px] --> |WebSocket / REST| Ingress
      WebWidget[Customer Web Widget] --> |WebSocket / REST| Ingress
      Ingress --> RustChat[Rust Axum Omnichannel Service]
      RustChat --> |Pub/Sub| Redis[Redis: Event Bus & Locks]
      RustChat --> |CRUD| DB[(PostgreSQL: RLS Enforced)]
      RustChat --> |Enqueue Draft/Automations| AIQueue[PostgreSQL AI Job Queue]
      AIQueue --> AIAgents[OHC Customer / Sales Assistants]
      AIAgents --> |Post Drafts| RustChat
  ```

  ### Mobile UX Flow (375px First)
  * **Unified Work Feed**: Owners open OHC on their phone and see a single "Needs Attention" list. A WhatsApp DM from a customer and an Instagram comment appear identically as grouped `Conversation` cards using premium Apple/Ubiquiti-style translucent materials.
  * **Quick Actions**: Swipe left to "Mark Resolved". Tap to view the thread.
  * **AI Drafts**: If a customer asks "Do you have vegan options?", the AI Customer Assistant prepares a draft message. The owner sees a distinct visual token (e.g., a glowing border or translucent badge) indicating "AI Draft Ready". One tap to approve and send.

  ### Key Design Decisions
  1. **Zero External Chat Dependencies**: All messaging logic lives in Rust. We control the data, meaning zero compliance or latency overhead from a third-party chat SaaS.
  2. **Channel Adapters as Traits**: Like Chatwoot's `app/models/channel/`, we will define a Rust `ChannelAdapter` trait that Meta (WhatsApp, IG) and Web Widget integrations implement to standardize incoming payloads into `Message` structs.
  3. **Strict RLS**: Every query must include the `tenant_id`. Redis locks (`ohc:lock:{tenant_id}:conversation:{id}`) will prevent race conditions when multiple agents/owners access a thread.

  ## Implementation Prompt
  **To the Implementer Agent:**
  1. **Scaffold the Domain**: Create the new native chat schemas (Inboxes, Contacts, Conversations, Messages, Channel Adapters) in `src/server/migrations/` using `tenant_id` for RLS.
  2. **Build the Rust Service**: Replace/upgrade `src/server/services/chat` with an Axum-based WebSocket server that handles omnichannel routing, maintaining 100% unit test coverage.
  3. **Implement UI Widget**: Create a 375px-optimized Next.js/Flutter chat widget mimicking the real-time functionality of Chatwoot but styled with OHC Premium Tokens (translucent glass, clear hierarchy).
  4. **Verification**: Write at least 5 Playwright E2E tests simulating a real non-technical owner (like Maya the Baker) receiving a Web Chat message, seeing the AI draft in her unified inbox, and approving it. Ensure NO mock data is used; route entirely through the real backend API and database.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
