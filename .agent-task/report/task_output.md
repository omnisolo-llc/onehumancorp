issue_title: "Implement 'The Ambassador' Agent - Native Social Inbox Auto-Responder"
issue_description: |
  **Problem Statement:**
  Solopreneurs like Maya (Home Baker) miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience.

  **Research Report:**
  - Traditional tools like Shopify and Wix require third-party apps for social media auto-responses.
  - SMBs hate the "App Tax" and piecing together disparate tools.
  - Non-technical users need an AI that executes actions, not just advises.
  - OHC can differentiate by offering a native, proactive autonomous agent that drafts replies and awaits owner approval via a simple mobile feed.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    sequenceDiagram
        participant Customer
        participant Webhook
        participant OHC_Agent
        participant RAG_Pipeline
        participant Owner_Mobile_App

        Customer->>Webhook: Sends DM (e.g., "Do you have vegan cakes?")
        Webhook->>OHC_Agent: Forwards message
        OHC_Agent->>RAG_Pipeline: Classify intent & retrieve inventory/policies
        RAG_Pipeline-->>OHC_Agent: Returns context (Vegan cakes in stock)
        OHC_Agent->>Owner_Mobile_App: Pushes Action Card to Agent Feed (Draft reply)
        Owner_Mobile_App->>OHC_Agent: Owner taps "Approve"
        OHC_Agent->>Customer: Sends response
    ```
  - **Mobile UX Flow (375px first):**
    - The OHC mobile app displays a 375px optimized card showing the drafted message.
    - The card has clear "Approve & Send", "Edit", and "Discard" actions with minimum 44x44px touch targets.
  - **AI Agent Integration Points:**
    - The Ambassador agent listens for new messages, analyzes intent, queries inventory/knowledge base, and generates a draft.
    - It interfaces with the Agent Feed to present the draft to the owner.
  - **Key Design Decisions:**
    - Do not automatically send messages without owner approval initially; use the Agent Feed to build trust.
    - Keep setup zero-click: no rule builders.

  **Implementation Prompt:**
  - Build a webhook receiver for incoming DMs (or a unified inbox mock).
  - Implement intent classification using the built-in LLM provider.
  - Implement RAG retrieval for context building based on the tenant's inventory and knowledge base.
  - Draft a response and push an "Action Card" to the Agent Feed (`agent_feed_items` table).
  - Build the mobile-first (375px) notification card UX in the unified agent feed to allow the owner to review and approve the drafted message.
  - Acceptance Criteria: A customer sends a DM, the agent drafts a reply visible in the Agent Feed on a 375px screen, and the owner can approve it.

  **Priority:** P0
  **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
