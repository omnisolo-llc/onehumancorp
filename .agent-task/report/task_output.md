issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Title: Implement Custom Rust Omnichannel Chat System to Replace Chatwoot

  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur. Furthermore, OHC previously relied on Chatwoot as an external third-party service, which has been 100% retired and is no longer an acceptable dependency.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **Chatwoot Source Code Audit:** Chatwoot's source code (`https://github.com/chatwoot/chatwoot`) reveals a mature omnichannel architecture with dedicated models for `Account` (Tenant), `Contact`, `Conversation`, `Message`, and various `Channel` adapters (e.g., WhatsApp, Web Widget, API). It uses ActionCable (WebSockets) for real-time updates and webhook endpoints for channel ingestion.
  - **OHC Opportunity:** Implement our own high-performance, native Rust omnichannel chat system inside `onehumancorp/mono` that achieves feature parity with Chatwoot's core data model and webhook/websocket architecture, but supercharged with our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph, and proactively drafts a complete, accurate response.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Rust Omnichannel Gateway API)
      C[WhatsApp] -->|Webhook| B
      D[Web Widget WebSocket] --> B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup / Upsert| F[Unified Customer Graph DB - Postgres RLS]
      B --> G[Message Ingestion & Event Mesh]
      G --> F
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Data Model & Invariants (Rust / SeaORM)
  - `Tenant` (Account): Row Level Security (RLS) via `tenant_id` on all tables.
  - `Channel`: Configuration for WhatsApp, Web Widget, Instagram, etc.
  - `Contact`: Unified customer identity across channels.
  - `Conversation`: Thread linking a `Contact` and a `Channel`.
  - `Message`: Individual message within a `Conversation`. Includes `status` (Draft, Sent, Delivered) and `sender_type` (Customer, Agent, AI).

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context. Bottom half shows the AI-drafted reply.
  - **Action:** Prominent primary button "Send Draft" and secondary "Edit". Native keyboard integration if editing. Glassmorphism cards and blurred backgrounds.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Triggered by incoming messages. Uses RAG against tenant catalog and customer history to draft personalized replies.

  ### Key Design Decisions
  - **Native Rust Implementation:** Replacing Chatwoot entirely with internal Rust modules (e.g., in `src/server/integrations/chat`).
  - **Proactive Drafting:** Move from read-reply to read-approve.
  - **Zero-Touch Fallback:** Human-only reply with suggested data points if AI confidence is low.

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram or WhatsApp, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it. The entire chat infrastructure runs natively within OHC.
  **CUJ & Acceptance Criteria:**
  1. Define and implement the SeaORM database schema (Entities) in Rust for `Contact`, `Conversation`, `Message`, and `Channel`, ensuring strict `tenant_id` RLS.
  2. Implement the Rust webhook handlers (Omnichannel Gateway) in `src/server/integrations/chat` to ingest incoming messages (e.g., simulating WhatsApp or Web Widget).
  3. Implement the Customer Identity Resolution Engine to match incoming identifiers (phone, handle) to a `Contact` or create a new one.
  4. Integrate the event mesh to trigger "The Ambassador Agent" upon message ingestion, which drafts a reply and saves it as a `Draft` message.
  5. Expose necessary GraphQL/REST/gRPC APIs for the Flutter mobile app to fetch the "Action Required" feed and approve drafts.
  6. Provide Playwright E2E tests: A test webhook sends a message, the user logs into the UI (mobile viewport), sees the drafted message card, taps "Approve", and the system dispatches the message.
  7. Achieve 100% unit test coverage for new Rust modules.

  **Priority**: P0 (Critical)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
