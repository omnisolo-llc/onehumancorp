issue_title: "Native Rust Omnichannel Chat System to Replace Chatwoot Dependency"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur. Furthermore, OHC currently relies on the external 3rd-party Chatwoot service which adds complexity, limits deep multi-tenant integration with OHC’s AI agents, and breaks the Zero Trust SPIFFE/SPIRE model. The mandate requires complete Chatwoot retirement in favor of a native, highly performant Rust architecture.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Audit:** Analyzed the Chatwoot Ruby-on-Rails repository (`https://github.com/chatwoot/chatwoot`). Key architectural components identified: multi-channel abstract models (Contacts, Conversations, Messages, Inboxes, Channels), real-time WebSockets, assignment rules, SLAs, macros, and webhook dispatching.
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed. By building this natively in Rust within `onehumancorp/mono`, we guarantee row-level tenant isolation, extremely low latency, and direct tight integration with our `OmnichannelService`.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Rust API Channel Adapter)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[(PostgreSQL: Unified Customer Graph DB)]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent / Operations Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Rust Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Data Model & Invariants
  The system will natively replicate Chatwoot's core data models using Rust and sqlx:
  - `Contact`: Represents the customer across platforms.
  - `Inbox`: The queue/bucket receiving messages (e.g., "Main Instagram Inbox").
  - `Channel`: The specific integration type (ChannelAdapter enum).
  - `Conversation`: A threaded context linked to a Contact and Inbox.
  - `Message`: Individual text/media payloads within a Conversation.
  **Invariants:** All tables must have a `tenant_id` and PostgreSQL RLS enabled to enforce strict multi-tenant Zero Trust boundaries.

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The top UI card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens the full threaded history spanning *all* channels with Sarah.
  - **AI Drafting:** Below the history, the system displays an AI-generated draft response proposed by the Ambassador agent.
  - **Action:** The owner simply taps "Approve" (large 44x44px target) to send, or taps the draft text field to natively edit the text using their mobile keyboard before sending.

  ### AI Agent Integration Points
  - **The Ambassador:** When a new `Message` is inserted via Webhook, the event mesh triggers The Ambassador agent. It pulls `Contact` history and context from `OmnichannelService`, queries the `LLMProvider`, and generates a draft `Message` marked as `status = Pending_Approval`.
  - **Operations Assistant:** If a message contains scheduling intent, the Operations agent proposes an availability calendar snippet.

  ### Key Design Decisions
  - **Native Rust over Chatwoot:** We drop the Chatwoot external dependency completely. The `OmnichannelService` in `src/server/services/omnichannel_service.rs` will be expanded to include these full CRUD operations.
  - **Zero Trust Multi-Tenancy:** We enforce SPIFFE/SPIRE-backed PostgreSQL RLS.
  - **Asynchronous Processing:** High-performance PostgreSQL `SKIP LOCKED` queues process incoming webhooks and AI drafting jobs to prevent blocking the HTTP adapters.

  ## Implementation Prompt
  **Goal:** Implement the native Rust omnichannel core data structures, services, and mobile-first UI components to fully replace Chatwoot for OHC SMB owners.
  **Task details:**
  1. Expand the PostgreSQL schema to include robust `Contact`, `Inbox`, `Conversation`, and `Message` tables, ensuring `tenant_id` RLS on all.
  2. Implement the Rust service traits and sqlx repositories for these entities inside `src/server/services/omnichannel_service.rs` (or a dedicated crate within the mono repo).
  3. Create standard REST or gRPC endpoints to ingest incoming channel webhooks and retrieve conversation threads.
  4. Develop the Flutter mobile UI (375px target viewport): create a unified feed card and a conversation detail screen that displays the threaded history alongside the AI-drafted response, including a prominent "Approve" button.
  **Acceptance Criteria:** A user can simulate receiving a webhook, view the message mapped to a Contact in the UI, review an AI-drafted reply, and approve it via a single tap. Ensure 100% unit test coverage and at least one E2E Playwright test proving the end-to-end CUJ.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
