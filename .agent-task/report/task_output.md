issue_title: "Native Rust Omnichannel Chat System Replication & Chatwoot Retirement"
issue_description: |
  # Mission Queue Protocol: Omnichannel Chat Replication

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) interact with customers across numerous channels (Instagram DMs, WhatsApp, SMS, Web Widgets, email). Currently, they lack a unified system that seamlessly consolidates these conversations while integrating proactive AI assistance. Relying on an external third-party service like Chatwoot introduces latency, fragmented multi-tenancy, and disjointed agent integration. Owners need a lightning-fast, native inbox that automatically aggregates messages, maintains context across all channels, and seamlessly hands off tasks to AI agents to draft replies, check inventory, and create bookings without opening multiple apps.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit**: Benchmarked Chatwoot's architecture (`https://github.com/chatwoot/chatwoot`), specifically its `app/models/conversation.rb`, `inbox.rb`, and `channel/` adapters (WhatsApp, Web Widget, SMS, etc.). The existing models heavily rely on Ruby on Rails abstractions that can be optimized in Rust for higher concurrency and lower memory footprint, which is critical for OHC's multi-tenant architecture.
  - **Shopify Inbox**: Provides a unified chat interface but lacks deep, proactive AI agent integration out-of-the-box. It functions mainly as a passive aggregator.
  - **Wix Inbox**: Consolidates messages well but lacks robust multi-channel context retention across different disconnected platforms without manual bridging.
  - **OHC Opportunity**: By retiring the external Chatwoot dependency and building a native Rust omnichannel engine, OHC can enforce strict multi-tenant Row Level Security (RLS) directly in PostgreSQL, drastically reduce webhook latency, and deeply integrate "The Ambassador" (Customer Success Agent) to draft contextual replies proactively.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / SMS] -->|Webhooks| B(Rust Omnichannel Gateway)
      C[Web Widget WebSocket] <--> B
      B --> D{Tenant Isolation & RLS Engine}
      D --> E[(Unified Customer Identity Graph)]
      D --> F[(Native Inboxes & Conversations DB)]
      B --> G[Event Mesh]
      G --> H[AI Agent Triage / The Ambassador]
      H -->|Query Catalog & History| E
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| B
      B -->|Dispatch| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile)**: The primary dashboard presents a prioritized list of actionable items ("Action Required: Approve Reply for Maya").
  - **Unified Inbox View**: Tapping an item opens a single conversation thread merging all past interactions (SMS, Web, IG) for that unified customer profile.
  - **Interaction**: The bottom half displays an AI-drafted reply based on context.
  - **Visual Design**: Uses OHC Premium Token library with translucent glass materials, strong typography, and clear status indicators.
  - **Action**: A prominent, thumb-reachable primary button "Approve & Send" and a secondary "Edit" button. Native mobile keyboard is invoked if "Edit" is selected.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success)**: Listens to the Event Mesh for new incoming messages. Queries the Customer Identity Graph to provide highly contextual draft replies based on past orders and interactions.
  - **The Manager (Operations)**: Triggered if the message involves scheduling or inventory changes, verifying availability before The Ambassador finalizes the draft.

  ### Key Design Decisions
  - **Native Rust Implementation**: Complete retirement of Chatwoot as an external service to guarantee low latency, robust WebSocket handling for web widgets, and tight coupling with OHC's internal Auth and AI queue.
  - **Proactive Drafting over Passive Reading**: The UI is designed to focus the owner on approving AI drafts rather than manually typing responses, saving significant time.
  - **Strict Multi-Tenancy**: All messages, inboxes, and customer records must enforce PostgreSQL RLS using `tenant_id`.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, when a customer sends a WhatsApp message and an Instagram DM, I see both in a single unified thread. The OHC assistant has already drafted a perfect response based on their past purchase history. I just tap "Approve" from my phone to reply.

  **CUJ & Acceptance Criteria:**
  1. Implement the native Rust models and database schemas for Inboxes, Conversations, Messages, and Unified Contacts with strict `tenant_id` RLS.
  2. Implement the Omnichannel Gateway to ingest messages via webhooks (e.g., simulating WhatsApp) and WebSockets (Web Widget).
  3. Ensure the AI Agent Triage consumes incoming message events and generates draft replies.
  4. Ensure the Flutter frontend on a 375px viewport displays the unified conversation with the AI-drafted reply and an "Approve & Send" button.
  5. Provide exhaustive Playwright E2E tests: A user logs in, receives a simulated cross-channel message, views the unified thread on a mobile layout, taps "Approve & Send", and the message is successfully dispatched via the Native Rust backend.
  6. **ZERO external Chatwoot dependencies** must remain.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
