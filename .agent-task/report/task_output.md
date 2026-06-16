issue_title: "Research: Native OHC Unified Omni-Channel Inbox Architecture"
issue_description: |
  **Title**: Native OHC Unified Omni-Channel Inbox Architecture

  **Problem Statement**:
  Small business owners like Carlos (Handyman) and Maya (Baker) receive customer inquiries across multiple disconnected channels (Instagram DMs, WhatsApp, Email, Web Chat). Managing these manually is reactive, labor-intensive, and leads to lost sales. Current platform implementations (like Shopify Inbox) simply aggregate messages without deep context or agent-driven resolution, acting as glorified chat apps rather than intelligent work assistants.

  **Research Report**:
  - Shopify Inbox aggregates basic messages but lacks proactive, AI-drafted replies deeply tied to the merchant's business context (inventory, past orders, policies).
  - Wix and Squarespace offer simple chat widgets that wait for the owner to type.
  - Zendesk/Intercom are enterprise-grade, too complex, and too expensive for solopreneurs.
  - **OHC Opportunity**: OHC's differentiation lies in "The Ambassador" (Customer Success Agent). Instead of just showing the message, the system should instantly resolve the customer's identity, pull their omnichannel history, query active inventory or calendar availability, and draft a complete, accurate response for the owner to approve with one tap on their mobile feed.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
        C[WhatsApp] -->|Webhook| B
        D[Email / Web] -->|Webhook| B
        B --> E{Identity Resolution Engine}
        E -->|Lookup| F[Unified Customer Graph DB]
        E --> G[Event Mesh]
        G --> H[The Ambassador Agent]
        H -->|Query Context| I[Inventory / Booking DB]
        H -->|Draft Reply| J[Action Required Queue]
        J --> K[Mobile App Feed 375px]
        K -->|1-Tap Approve| L[Omnichannel Dispatcher]
        L --> A/C/D
    ```
  - **Mobile UX Flow (375px First)**:
    - User opens the OHC mobile app. The top of the work feed shows an "Action Required" card: "1 New Message from Sarah (Insta DM)".
    - Tapping the card opens a unified view. Top half: Customer context (e.g., "Sarah bought a vegan cake 2 months ago"). Bottom half: The Ambassador Agent's drafted reply (e.g., "Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
    - Actions: Prominent primary button "Approve & Send", secondary "Edit". Native mobile keyboard integration if editing.
  - **AI Agent Integration Points**:
    - **The Ambassador (Customer Success)**: Triggered via the event mesh upon new message arrival. Uses RAG against tenant data (inventory, past orders, policies) to draft the reply.
    - **The Manager (Operations)**: Consulted by The Ambassador to verify stock or calendar slots before promising availability.
  - **Key Design Decisions**:
    - Shift from "read-reply" to "read-approve": The AI drafts the response *before* the owner sees it.
    - Strict Multi-Tenancy: Customer graphs and LLM context are isolated via `tenant_id` at the row level.

  **Implementation Prompt**:
  **User-Facing Outcome**: When a customer DMs the owner on Instagram asking about an order or product, the owner opens the OHC app to find a pre-written, highly accurate response already drafted. They tap one button to send it, taking 2 seconds instead of 2 minutes.
  **CUJ & Acceptance Criteria**:
  1. Create the `Omnichannel Gateway` service to ingest mocked incoming webhook payloads (representing IG/WhatsApp messages).
  2. Implement the `Identity Resolution Engine` to match incoming handles/numbers to existing customer records in PostgreSQL.
  3. Wire the incoming event to `The Ambassador` agent, enabling it to query the seed inventory/customer history data (no internal mocking allowed) and draft a response.
  4. Expose the drafted response via an API endpoint serving the `ActionRequiredQueue` for the mobile feed.
  5. Provide an endpoint to accept the "Approve & Send" action, simulating dispatch back to the channel.
  6. **Automated Verification**: Include unit tests for the identity matching and Playwright E2E tests verifying the 1-tap approval flow in a simulated 375px mobile UI environment.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
