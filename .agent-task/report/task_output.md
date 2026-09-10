issue_title: "Architectural Gap: Universal Multi-Harness Shared Context & Omnichannel Real-time Engine"
issue_description: |
  Problem Statement
  Small business owners like Carlos the handyman and Maya the baker operate across highly fragmented channels (WhatsApp, Instagram, Email, SMS, Web Chat). Currently, OneHumanCorp (OHC) lacks a native, high-performance omnichannel message synchronization layer that seamlessly feeds conversational data into our Universal Multi-Harness agent ecosystem. Without a unified underlying context ledger, agents operating on different harnesses (e.g., OmniSolo, DeepSeek, OpenCode) fail to share real-time customer state, leading to redundant work, context loss, and uncoordinated AI actions. The owner is left with fragmented UI views and duplicated AI drafting efforts, preventing OHC from acting as a single cohesive assistant.

  Research Report
  - Shopify Inbox & Wix: Aggregate basic text but lack agentic multi-harness state sharing. They require human intervention for any complex multi-step request (e.g., cross-referencing inventory and scheduling).
  - Zendesk & Intercom: Offer advanced context sharing but are excessively complex, expensive, and not built native-first for a 375px mobile experience.
  - Open Source Agent Frameworks (LangGraph, AutoGen): Have established patterns for state graphs, but OHC requires a unified Rust-native event mesh to ensure low-latency sync between our Universal Provider Facade (gpt-5.6-luna) and Local Services bundle (omnisolo.memory, omnisolo.workspace).
  - Conclusion: OHC must implement a Native Rust Omnichannel Real-Time Engine (WebSocket/SSE) coupled with a Universal Multi-Harness Context Ledger. This architecture will ingest all inbound communications, resolve customer identity, and maintain a shared SessionCapsule that all active agent harnesses can read/write simultaneously.

  Design Doc
  Architecture Diagram
  graph TD
      A[WhatsApp/IG/SMS/Web] -->|Webhooks & Polling| B(Rust Omnichannel Gateway)
      B --> C{Identity Resolution Engine}
      C -->|Maps to tenant_id| D[(PostgreSQL Shared Ledger)]
      B --> E[Real-Time Event Mesh]
      E --> F[SessionCapsule Context Manager]
      F --> G(Universal Multi-Harness Runtime)
      G -->|Harness A: Operations| H[operations.agent]
      G -->|Harness B: Customer| I[customer.agent]
      H --> F
      I --> F
      E -->|WebSocket/SSE| J[Flutter Mobile UI 375px]

  Mobile UX Flow (375px First)
  - Unified Feed Screen: The app opens to a clean, translucent glass-styled list of active customer threads across all channels.
  - Thread View: Tapping a thread shows the customer's history. A sticky bottom bar displays the agent-drafted proposed reply (e.g., Drafted by CS Agent: Yes, we have 3 left!).
  - Action: A single prominent primary button (e.g., Send & Reserve) allows the owner to approve a multi-agent action (replying + updating inventory) with zero context switching.
  - Visuals: Apple/Ubiquiti-style hierarchy, restrained translucent materials, distinct status tokens for human vs. AI-drafted messages. Touch targets minimum 44x44px.

  AI Agent Integration Points
  - SessionCapsule Injection: The omnisolo.memory local service intercepts all inbound events from the Rust Gateway and updates a real-time SessionCapsule.
  - Universal Provider Facade: Routes the SessionCapsule to the appropriate AI department (Operations vs. Customer Success) based on intent classification.
  - Multi-Harness Coordination: Agents lock resources via Redis Redlock (ohc:lock:{tenant_id}:conversation:{id}) to ensure only one agent modifies the SessionCapsule or drafts a reply at a time.

  Key Design Decisions
  - Rust Native Real-Time Engine: Chosen over Node.js/Go for predictable tail latencies and memory safety when handling high-throughput webhook streams and WebSocket connections for thousands of tenants.
  - Centralized SessionCapsule: Eliminates context drift between different AI models/harnesses. All agents read from the exact same real-time state.
  - Zero-Trust Multi-Tenancy: Row-Level Security (RLS) mandated in PostgreSQL via tenant_id for all ledger and conversation tables.

  Implementation Prompt
  User-Facing Outcome: The business owner receives an Instagram DM. They open the OHC app and immediately see a unified thread with an accurate AI-drafted reply that perfectly incorporates the customer's purchase history and real-time inventory, ready to be sent with one tap.
  CUJ & Acceptance Criteria:
  1. Build a Rust-native OmnichannelGateway service that accepts inbound webhook payloads and normalizes them into a standard EventDeliveryEnvelope.
  2. Implement the SessionCapsule state manager that persists these envelopes into a PostgreSQL multi-tenant ledger (enforcing RLS with tenant_id).
  3. Create the omnisolo.memory service integration to allow any spawned AI harness worker to retrieve and update the current SessionCapsule via the Universal Provider Facade.
  4. Ensure all cross-agent mutations utilize Redis Redlock to prevent race conditions on shared customer context.
  5. Provide complete unit tests and Playwright E2E tests validating that an inbound webhook correctly updates the mobile UI (375px layout) via WebSocket and displays a generated draft.

  Priority: P0
  Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
