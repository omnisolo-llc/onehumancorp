issue_title: "Native Rust Omnichannel Unified Inbox & Agent Chat Gateway"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  Furthermore, OHC requires a highly scalable, multi-tenant capable omnichannel customer support engine built natively in Rust. We must completely retire Chatwoot as an external dependency to enforce Zero-Trust SPIFFE/SPIRE identity and maintain Row Level Security (RLS) within our unified PostgreSQL architecture.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Chatwoot Source Audit:** We have evaluated `https://github.com/chatwoot/chatwoot` to understand its omnichannel data models, controllers, channels, WebSocket real-time messaging, and inbox architecture. We must replicate this feature set in a native Rust implementation.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway - Rust)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      E[Web Widget] -->|WebSocket| B
      B --> F{Customer Identity Resolution Engine}
      F -->|Lookup| G[Unified Customer Graph DB - Postgres RLS]
      F --> H[Event Mesh]
      H --> I[The Ambassador Agent]
      I -->|Query Context| G
      I -->|Draft Reply| J[Action Required Queue]
      J --> K[Mobile App Feed 375px]
      K -->|1-Tap Approve| L[Omnichannel Dispatcher - Rust]
      L --> A/C/D/E
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing. Following OHC Premium Token library with Apple/Ubiquiti-style hierarchy.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Complete Chatwoot Retirement:** Chatwoot is 100% retired. Native Rust microservices will handle channel adapters, web chat widget, WebSocket events, APIs, webhooks, and agent routing.
  - **Tenant Isolation:** Enforced via `tenant_id` Row Level Security (RLS) in PostgreSQL.
  - **Zero-Trust Identity:** Rely on SPIFFE/SPIRE for inter-agent and microservice communication.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes. All chat interactions (WhatsApp, Instagram, Web Widget) flow through a seamless, native Rust backend without any external Chatwoot dependency.

  **CUJ & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the Native Rust Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier (e.g., social handle) to an existing customer record in the database, applying RLS using `tenant_id`.
  3. The Ambassador Agent is triggered via Event Mesh and successfully queries the customer's past orders and the current product catalog.
  4. The Agent generates a draft reply and places it in the `ActionRequiredQueue` for the specific tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized (375px) feed, taps "Approve," and the system dispatches the message back to the mocked external channel through the Native Rust Dispatcher.
  6. **Unit Tests:** Achieve 100% unit test coverage for the new Rust omnichannel components (Gateway, Event Mesh Publisher, Identity Resolver).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
