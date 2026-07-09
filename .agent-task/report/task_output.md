issue_title: "AI Unified Inbox & Omnichannel Customer Memory"
issue_description: |
  # AI Unified Inbox & Omnichannel Customer Memory

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require the owner to manually type responses.

  ## Research Report
  Our competitive analysis indicates that existing solutions (e.g. Shopify Inbox, Wix Inbox) aggregate messages but lack proactive AI capabilities. Enterprise-grade tools (Zendesk, Intercom) are too complex for SMBs. The opportunity for OHC is to leverage our "Teammate" AI philosophy, deploying a Customer Success Agent ("The Ambassador") that actively reads messages, queries a unified customer identity graph for context (purchase history, past interactions), and drafts actionable responses for owner approval.

  ## Design Doc
  - **Architecture Diagram (Mermaid)**:
    ```mermaid
    graph TD
        A[External Channels: IG, WhatsApp, Email] -->|Webhook| B(Omnichannel Gateway)
        B --> E{Customer Identity Resolution Engine}
        E -->|Lookup| F[Unified Customer Graph DB]
        E --> G[Event Mesh]
        G --> H[The Ambassador Agent]
        H -->|Query Context| F
        H -->|Draft Reply| I[Action Required Queue]
        I --> J[Mobile App Feed 375px]
        J -->|1-Tap Approve| K[Omnichannel Dispatcher]
    ```
  - **Mobile UX Flow (375px)**:
    - User receives a push notification on their phone.
    - Opening the app reveals an "Action Required: Approve Reply" card in the Feed.
    - The card displays the incoming message and the agent's drafted reply (with retrieved context).
    - Buttons: "Approve & Send", "Edit", "Discard".
  - **AI Agent Integration**:
    - The Ambassador agent listens to the Event Mesh for new incoming messages.
    - It uses RAG (Retrieval-Augmented Generation) against the Unified Customer Graph DB.
    - It generates a reply and posts it to the Action Required Queue.

  ## Implementation Prompt
  **Target Persona**: Maya (Home Baker) / Carlos (Handyman)
  **Feature Name**: AI Unified Inbox & Omnichannel Customer Memory
  **Outcome**:
  Develop the core backend models for an omnichannel gateway and unified customer identity resolution. Build "The Ambassador Agent" that ingests these messages, queries context, and drafts a reply. Create a 375px mobile-first UX feed card that allows the business owner to review, edit, or approve the agent's drafted message with 1-tap. Make sure this handles multi-tenant data securely.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
