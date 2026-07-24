issue_title: "Implement High-Performance Rust Omnichannel Gateway for Core Work Triage"
issue_description: |
  # Problem Statement
  Small business owners like Maya the Baker or Carlos the Handyman are overwhelmed by customer communications scattered across Instagram DMs, WhatsApp, SMS, and email. The current OHC architecture lacks a unified, high-performance inbound gateway to ingest and route these messages into the "Work Triage" queue. Relying on external third-party services like Chatwoot is fundamentally against OHC's product vision and has been explicitly retired. We need a native, multi-tenant Rust service capable of terminating webhooks from multiple channels, performing identity resolution, and queuing these events for the AI agents (like The Ambassador) to draft responses. Without this foundational layer, the "Ask one assistant" promise of OHC cannot be realized.

  # Research Report
  **Findings & Competitive Analysis:**
  -   **Legacy unified inboxes (Shopify Inbox, Wix Inbox)** simply aggregate messages for manual reading. They lack the architectural depth to feed messages instantly to AI agents with rich context.
  -   **Chatwoot Architecture (Retired)**: Chatwoot relied on a Ruby on Rails backend with Sidekiq for background jobs and ActionCable for WebSockets. While functionally complete, it is too resource-heavy and slow for our edge-first, AI-native approach.
  -   **The Native Rust Approach**: By building the omnichannel gateway in Rust, we achieve microsecond latency, minimal memory footprint, and guaranteed memory safety. This aligns perfectly with the requirement for a resilient, high-scale intake system that can run efficiently in our Kubernetes clusters.

  **Core Missing Capabilities Identified:**
  1.  **Webhook Termination Layer**: A highly available Rust service to receive inbound messages from WhatsApp Cloud API, Instagram Graph API, and generic email webhooks.
  2.  **Omnichannel Data Model**: We lack the `Conversation`, `Message`, and `Contact` multi-tenant data models in our Rust backend, modeled after the successful parts of Chatwoot but optimized for our AI-first approach.
  3.  **Job Queuing Integration**: Inbound messages must be instantly queued (using PostgreSQL SKIP LOCKED or Redis) for the AI Job Queue so that "The Ambassador" agent can immediately begin drafting a reply based on the unified customer graph.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp Webhook] -->|JSON Payload| B(Rust Omnichannel Gateway)
      C[Instagram Webhook] -->|JSON Payload| B
      D[Email Webhook] -->|JSON Payload| B
      B --> E{Identity & Multi-Tenant Context}
      E -->|Verify Tenant ID| F[(PostgreSQL: Unified Identity Graph)]
      E --> G[Event Queue / PgBouncer]
      G --> H(AI Job Worker - Go/Rust)
      H -->|RAG + Draft| I[(PostgreSQL: Drafted Replies)]
      I --> J[Mobile App Feed 375px - Work Triage]
      J -->|Owner Taps 'Approve'| B
      B -->|Dispatch| A/C/D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  -   **Work Triage Feed (Mobile View)**: When Maya opens the app, the first card she sees is an "Action Required" item.
  -   **Card Content**: "New Instagram DM from @sarah_bakes: 'Do you have vegan cakes?'"
  -   **AI Draft Section**: Below the message, a distinct visual block (using OHC Premium translucent glass tokens) shows the drafted reply: "Yes Sarah, we have 3 vegan chocolate cakes left for this Saturday! Would you like me to hold one for you?"
  -   **Interaction**: A primary, full-width (minimum 44px height) button labeled "Approve & Send". A secondary icon button for "Edit". No horizontal scrolling is permitted.

  ### AI Agent Integration Points
  -   The Rust Omnichannel Gateway acts as the *source* of the event. It does not perform LLM calls itself.
  -   It inserts a standard `OmnichannelMessage` event into the high-performance PostgreSQL job queue.
  -   The existing (or to-be-built) AI orchestrator picks up this job, fetches the conversation history and customer profile, and invokes Gemini to draft the response.

  ### Key Design Decisions
  -   **Language**: Rust (using `axum` for the web framework) for absolute performance and safety in parsing untrusted webhook payloads.
  -   **Database Isolation**: Strict row-level security (RLS) enforcement based on the `tenant_id` extracted from the webhook verification step.
  -   **Stateless Ingestion**: The gateway must be entirely stateless, allowing infinite horizontal scaling during traffic spikes.
  -   **Idempotency**: Webhook processing must handle duplicate deliveries gracefully using unique provider message IDs.

  # Implementation Prompt
  **User-Facing Outcome:** The user (owner) never sees this infrastructure directly. The outcome is that when a customer sends a message on WhatsApp or Instagram, the owner opens the OHC mobile app and immediately sees the message along with a perfect, context-aware AI-drafted reply ready for 1-tap approval in their Work Triage feed.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1.  Implement a new Rust service (e.g., in `src/server/omnichannel_gateway`) using `axum`.
  2.  Create endpoints to receive simulated incoming webhooks (e.g., `/api/v1/webhooks/whatsapp`, `/api/v1/webhooks/instagram`).
  3.  Define the Rust structs (models) for `IncomingMessage`, ensuring multi-tenant `tenant_id` is parsed or inferred from the webhook configuration.
  4.  Implement logic to securely parse the payload, perform basic validation, and insert a normalized `MessageEvent` record into the PostgreSQL database.
  5.  Write unit tests with 100% coverage for the parsing and validation logic.
  6.  Write at least one Playwright E2E test that simulates sending a webhook to the Rust gateway and verifies that the message eventually appears in the mobile UI's "Work Triage" feed (this may require mocking the AI drafting step for the test, but the data flow must be real).
  7.  Ensure the solution adheres strictly to Zero-Trust principles (validate webhook signatures).

  **Priority:** P0 (Critical foundational architecture for the Work Triage promise)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
