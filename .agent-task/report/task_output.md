issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System - Architecture & Design Brief

  ## Mission Queue Protocol
  This report details the architectural design for OneHumanCorp's native Rust Omnichannel Chat system, completely replacing external dependencies like Chatwoot.

  ## Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) receive customer inquiries across unlinked channels: WhatsApp, Instagram DMs, SMS, and email. Managing these manually leads to missed messages, slow responses, and lost sales. We need to implement a native, highly performant Rust omnichannel chat system within OHC that aggregates these messages and leverages our "The Ambassador" AI to proactively draft contextual responses, removing the friction of manual multi-channel management. Chatwoot has been retired, and we must build the core engine natively.

  ## Research Report
  - **Chatwoot Source Audit:** We reviewed Chatwoot's core architecture (conversations, messages, inboxes, channel adapters). Chatwoot's data models (`app/models/conversation.rb`, `message.rb`, `inbox.rb`) provide a solid reference for the entities required, but we will implement them natively in Rust, optimized for our multi-tenant SaaS architecture.
  - **WhatsApp Integration:** Meta's WhatsApp Cloud API will be a primary channel adapter. It uses webhooks for incoming messages and requires tracking user-initiated vs. business-initiated conversations.
  - **AI Integration:** The system must seamlessly integrate with "The Ambassador" agent to draft replies based on customer history and product catalogs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp / Insta DM / Web Widget] -->|Webhook/WS| B(Channel Adapters / Gateway)
      B --> C[Omnichannel Chat Engine - Rust]
      C -->|Tenant Isolated DML| D[(PostgreSQL Unified Graph DB)]
      C --> E[Event Mesh]
      E --> F[The Ambassador AI Agent]
      F -->|Context Lookup| D
      F -->|Drafts Reply| C
      C --> G[Action Required Queue]
      G --> H[Flutter Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> A
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed:** The owner sees a unified "Action Required" card for a new incoming message (e.g., "1 New Message from Sarah via WhatsApp").
  - **Detail View:** Tapping the card reveals:
    - Top half: Customer context (e.g., "Sarah bought a custom cake 2 months ago").
    - Bottom half: The Ambassador AI-drafted reply (e.g., "Hi Sarah! Yes, we can do vegan again. Would you like to reorder?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visuals:** MacOS Translucent Glass styling, clean typography, 44x44px touch targets.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered via the Event Mesh upon new message ingestion. It queries the tenant's product catalog and customer history to generate a context-aware draft, marking the message state as `draft_ready` for owner review.

  ### Key Design Decisions
  - **Native Rust Implementation:** Eliminates the operational overhead of managing external Chatwoot instances. Allows deep integration with OHC's multi-tenant (RLS) PostgreSQL database and Event Mesh.
  - **Data Models:**
    - `Inbox`: Configuration for a specific channel (e.g., a WhatsApp number).
    - `Conversation`: Represents a thread with a contact.
    - `Message`: Individual messages within a conversation.
    - `Contact`: The resolved omnichannel identity of the customer.
  - **Row Level Security (RLS):** All entities must strictly enforce `tenant_id` isolation.
  - **Proactive Drafting over Read-Reply:** The AI drafts the response *before* the owner even opens the app.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer messages the business via WhatsApp, the owner opens the OHC app to find a pre-written, perfectly accurate response drafted by the AI. They tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1.  **Data Layer:** Implement the core Rust data models and PostgreSQL migrations for `Inbox`, `Conversation`, `Message`, and `Contact`, strictly enforcing `tenant_id` RLS.
  2.  **API/Service Layer:** Implement the Rust API endpoints/services in `src/server/integrations/chat/` (or appropriate module) to handle message ingestion (simulated webhook), conversation management, and drafting.
  3.  **Agent Integration:** Connect the message ingestion flow to The Ambassador agent (or a placeholder integration) to generate a draft reply.
  4.  **UI Verification:** Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
  5.  **No Mocks in UI:** The frontend must render real data from the backend, not hardcoded mock data.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
