issue_title: "Unified Multi-Channel Inbox & Identity Resolution Architecture"
issue_description: |
  ## Title: Unified Multi-Channel Inbox & Identity Resolution Architecture

  ## Problem Statement
  Business owners like Maya (baker, relying on Instagram DMs) and Carlos (handyman, using SMS/WhatsApp) are currently forced to context-switch across fragmented messaging apps. They lose leads because conversations are isolated from their business state (inventory, bookings, CRM). There is no single "Work Triage" feed that successfully unifies customer communications, automatically resolves a customer's identity across platforms (e.g., matching an Instagram DM to an existing client profile in the DB), and allows the AI Customer Assistant to draft and send context-aware replies securely.

  ## Research Report
  - **Market Context:** Traditional helpdesk tools (Zendesk, Intercom) are designed for support teams and are too complex for a solo operator on a phone. Tools like Shopify Inbox offer unified messaging but lock users into their e-commerce ecosystem. We need a solution like Meta Business Suite's inbox, but natively integrated with the OHC operations, finance, and agent layers.
  - **Identified Gap:** The platform lacks a generalized ingestion pipeline for asynchronous messages (Webhooks for Instagram, WhatsApp, SMS, Email) and an identity resolution mechanism to link incoming messages (e.g., an Instagram handle or phone number) to a unified `Customer` record within a `Tenant`.
  - **Solution Strategy:** Build a high-throughput webhook ingestion layer, an Identity Resolution service, and a prioritized feed API (the "Agent Feed") where the AI Customer Assistant can proactively draft replies.

  ## Design Doc

  ### 1. Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
    A[Customer on IG/WhatsApp] -->|Webhook| B(Webhook Gateway)
    B --> C{Identity Resolution Service}
    C -->|New/Existing Identity| D[Unified Inbox DB (PostgreSQL)]
    D --> E((Event Bus / Redis PubSub))
    E --> F[AI Customer Assistant (Gemini)]
    F -->|Drafts Reply| G[Agent Feed API]
    G --> H[Flutter Mobile App (375px)]
    H -->|Owner Approves| I[Outbound Messaging Service]
    I --> A
  ```

  ### 2. Data Model & Invariants
  - **`CustomerIdentity`:** Maps an external identifier (e.g., `provider: instagram, external_id: @user123`) to an internal `customer_id`. Must enforce strict `tenant_id` isolation to prevent cross-tenant data leakage.
  - **`Message`:** Represents an individual message in a `Conversation`. Includes `direction` (inbound/outbound), `status` (pending_draft, approved, sent, failed), and `content`.
  - **`AgentDraft`:** Stores the AI-generated proposed response linked to a `Message` or `Conversation`, awaiting owner approval.
  - **Security:** All webhook payloads must be verified via provider-specific signature mechanisms (e.g., X-Hub-Signature for Meta) before processing.

  ### 3. Mobile UX Flow (375px First)
  - **Home Screen / Triage Feed:** The default view. Shows a unified list of actionable items (messages, tasks).
  - **Conversation Card:** A clean, UniFi-style card showing the customer's message, their identified profile context (e.g., "Past Customer, 2 Orders"), and the AI-drafted reply.
  - **Action Bar:** Translucent glass bottom bar with clear "Approve & Send", "Edit Draft", or "Ignore" buttons (touch targets ≥ 44x44px).
  - **No horizontal scrolling.** Forms and text inputs must use native keyboards efficiently without breaking the layout.

  ### 4. AI Agent Integration Points
  - **Customer Assistant:** Listens to the Event Bus for new inbound messages. Uses RAG (Retrieval-Augmented Generation) against the tenant's policies, inventory, and the customer's past history to generate an `AgentDraft`.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend unified messaging ingestion and the frontend Triage Feed UI.

  **Acceptance Criteria:**
  1. Build a webhook ingestion endpoint that securely accepts incoming messages (mocking Instagram/WhatsApp payloads for now) and persists them to the PostgreSQL database with row-level tenant isolation.
  2. Implement an Identity Resolution step that matches the incoming external ID to an existing customer profile or creates a new one.
  3. Create the Flutter UI (targeting a 375px viewport) for the "Triage Feed" that displays these unified messages.
  4. Ensure the UI includes premium "Translucent Glass" styling and clear actionable buttons ("Approve", "Edit") for AI drafts.
  5. The Critical User Journey (CUJ) must work end-to-end: An inbound webhook creates a message -> The UI displays the message in the feed -> The owner taps "Approve" -> The state updates to "sent".
  6. **Mandatory:** The UI must contain zero mock data. All displayed messages must come from the backend. E2E Playwright tests must cover the full approval flow.

  ## Priority: P0
  ## Estimated Scope: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
