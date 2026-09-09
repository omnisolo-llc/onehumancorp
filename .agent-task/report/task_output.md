issue_title: "Native Rust Omnichannel Customer Support Engine Architecture"
issue_description: |
  **Title**: Architect Native Rust Omnichannel Customer Support & Chat Engine

  **Problem Statement**:
  Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by inbound requests scattered across Instagram DMs, SMS, email, and web chat. They lack a unified inbox that operates instantly without dropping messages. Currently, OHC relies on fragmented integrations or lacks a true real-time, multi-tenant capable central communication nervous system.

  **Research Report**:
  - Benchmarking against Shopify Inbox, WeCom, and HubSpot highlights that a native, embedded omnichannel engine is critical for conversion.
  - Shopify Inbox demonstrates a unified view where Instagram, Facebook, and Web Chat merge.
  - HubSpot uses extensive real-time typing indicators and shared context to prevent agent collisions.
  - Relying on external third-party chat widgets introduces latency and breaks the native macOS-style Glass design requirements of OHC.
  - A Rust-based real-time engine ensures predictable, sub-millisecond dispatching with minimal memory overhead, suitable for edge deployments.

  **Design Doc**:
  - **Architecture**: A native Rust-based real-time messaging server using WebSockets/SSE.
    - `ChannelAdapters`: Rust modules for IG, SMS (Twilio), Email, and Webhooks.
    - `ConversationRouter`: Handles auto-assignment, multi-tenant isolation, and SLA tracking.
    - `StateStore`: PostgreSQL with Row-Level Security for persistent messages; Redis for ephemeral states (typing indicators, presence).
    ```mermaid
    graph TD;
      Client[Mobile/Web Clients] -->|WebSocket/SSE| Gateway[Rust API Gateway];
      Gateway --> Router[Conversation Router];
      Router --> DB[(PostgreSQL RLS)];
      Router --> Cache[(Redis Presence)];
      Channel[IG/SMS/Email Webhooks] --> Adapter[Rust Channel Adapters];
      Adapter --> Router;
    ```
  - **Mobile UX Flow (375px first)**:
    - Viewport width strictly 375px.
    - Bottom navigation -> "Inbox".
    - Unified list of threads (avatar, channel icon badge, preview).
    - Chat view: Full-height, fixed bottom input area (min 44px height). Translucent Glass header.
  - **AI Agent Integration**:
    - AI acts as a silent participant in the thread.
    - Analyzes incoming messages, fetches context (e.g., previous orders), and generates suggested "Draft Replies".
    - Exposes an event stream for agents to observe typing indicators and delay actions to prevent overlapping human replies.

  **Implementation Prompt**:
  - Role: Principal Software Engineer & Canvas (L7)
  - Task: Implement the native Rust core omnichannel inbox architecture.
  - Criteria: Establish a WebSocket server in Rust. Implement a unified conversation model with strict multi-tenant isolation. Provide a 375px mobile-first chat UI prototype displaying simulated incoming messages and AI draft replies, utilizing macOS-style Translucent Glass materials.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
