issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol Brief
  **Title**: Native Rust Omnichannel Chat System (Chatwoot Replacement)
  **Priority**: P0
  **Estimated Scope**: Large

  ## Problem Statement
  Small business owners need to manage communications across multiple unlinked channels (Instagram DMs, WhatsApp, SMS, web chat) seamlessly. OHC previously relied on Chatwoot as an external service for omnichannel communications. However, relying on a third-party dependency breaks our architectural goals of deep native multi-tenancy, Row Level Security (RLS), and zero-trust data isolation. Chatwoot as an external dependency is now 100% RETIRED. We need to implement a native, high-performance omnichannel chat system in Rust that matches Chatwoot’s capabilities while integrating deeply into our core PostgreSQL + Bazel + Rust/Go architecture and supporting our AI-first Ambassador workflows.

  ## Research Report
  **Chatwoot Source Code Audit & Feature Benchmarking:**
  Based on an audit of the `chatwoot/chatwoot` repository, the core systems required for parity include:
  1. **Omnichannel Data Models**:
     - `Account` (Tenant)
     - `User` / `Contact`
     - `Inbox` (Channel aggregation point)
     - `Conversation` (Thread of messages)
     - `Message` (Individual payload with attachments)
     - `Channel` adapters (`Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Sms`)
  2. **WebSocket & Real-time Inbox Architecture**: Chatwoot relies heavily on ActionCable for real-time WebSocket events. Our Rust implementation must use an event mesh (e.g., Tokio + Redis Pub/Sub) to broadcast message events to the frontend in real-time.
  3. **Controllers & Webhooks**: Chatwoot exposes endpoints for receiving Meta/Twilio webhooks and parsing incoming payloads into structured `Message` entities.

  **OHC AI Agent Integration Opportunity**:
  Unlike Chatwoot, which requires manual owner response or basic macros, our native Rust chat system will automatically feed incoming messages to the `Event Mesh`, triggering **The Ambassador Agent** to proactively draft a contextual response based on the unified customer graph before the owner even opens the app.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Incoming Webhooks: Meta/WhatsApp/IG] --> B(Rust Omnichannel Gateway)
      C[Web Widget] -->|WebSocket| D(Rust Real-Time Chat Server)
      B --> E{Incoming Event Parser}
      D --> E
      E -->|Save Message| F[(PostgreSQL: Conversations & Messages RLS)]
      E -->|Publish Event| G[Redis Pub/Sub Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query DB| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[OHC Mobile UI 375px]
      J -->|Approve| K(Rust Dispatcher)
      K -->|Send via API| A
  ```

  ### Mobile UX Flow (375px First)
  - **Triage Feed**: The owner opens the app (375px) and sees a priority card: "1 New Message from Sarah (WhatsApp)".
  - **Context View**: Tapping the card opens a translucent glassmorphism modal. The top half shows Sarah's purchase history (unified context).
  - **AI Draft**: The bottom half shows The Ambassador's pre-drafted reply.
  - **Action**: One-tap "Approve & Send" primary button. Secondary "Edit" button opens the native keyboard.

  ### AI Agent Integration Points
  - **The Ambassador**: Listens to the `conversation.message.created` event on Redis Pub/Sub. When triggered, it fetches context, generates a drafted reply using the unified customer identity graph, and saves a `Message` with status `draft`.

  ### Key Design Decisions
  - **Native Rust**: The backend will be implemented natively in Rust within the `onehumancorp/mono` repo to ensure high performance and tight Bazel build integration.
  - **RLS & Multi-tenancy**: All chat tables (`inboxes`, `conversations`, `messages`, `contacts`) will enforce strict PostgreSQL Row Level Security (RLS) on `tenant_id`.
  - **Event-Driven**: The system relies on a central Redis Pub/Sub mesh to decouple webhook ingestion from AI processing and UI WebSocket pushes.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer sends a message via WhatsApp, the owner receives a notification in the OHC mobile app. The AI has already drafted a perfectly accurate, context-aware reply, allowing the owner to respond in 1 second with a single tap.

  **CUJ & Acceptance Criteria:**
  1. Implement the Rust data models and PostgreSQL schemas for `inboxes`, `conversations`, `messages`, and `contacts` with RLS.
  2. Build a Rust webhook receiver (e.g., using Axum/Actix) to accept mock Meta/WhatsApp messages.
  3. Emit an event to Redis Pub/Sub when a message is received.
  4. Build the WebSocket server to push real-time updates to connected clients.
  5. The Ambassador agent successfully consumes the event, drafts a response, and stores it as a draft `Message`.
  6. **E2E Test Requirement:** Write a Playwright E2E test where a simulated WhatsApp webhook hits the system, the mobile UI updates in real-time, displays the draft, the owner taps "Approve", and the drafted message is dispatched (mocked external API call).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
