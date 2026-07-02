issue_title: "[Architecture] Universal Agentic Omnichannel Inbox & Contextual Triage"
issue_description: |
  # Research Report: Universal Agentic Omnichannel Inbox & Contextual Triage

  **Problem Statement:**
  Small business owners (like Maya the Baker or Carlos the Handyman) receive customer inquiries and bookings across multiple channels—Instagram DMs, WhatsApp, SMS, email, and web forms. Currently, these channels are disjointed, requiring the owner to switch contexts, manually connect customer identities, and decipher which messages require immediate action versus which can wait. They need a unified, intelligent inbox that triages intent, synthesizes context, and proactively drafts resolutions.

  **Research Findings:**
  - **Market Context:** Traditional helpdesk tools (Zendesk, Intercom) are too complex for SMB operators. Modern platforms (like Meta Business Suite or Shopify Inbox) unify *some* channels but lack deep autonomous triage—they still require the human to read everything. WeCom and DingTalk provide strong unification but are heavy.
  - **Customer Pain:** Maya wakes up to 3 Instagram DMs about cakes, 2 emails asking for custom quotes, and 1 WhatsApp message changing a delivery date. The cognitive load to prioritize these is high.
  - **Architectural Gap:** OHC currently processes webhooks or API requests for external messages but doesn't have a centralized, multi-tenant normalized "Event/Message" bus connected to an AI Intent routing layer.

  **Design Doc:**
  - **Architecture Diagram (Mental Model):**
    ```mermaid
    graph TD
        subgraph Ingestion
            IG[Instagram DMs] --> W[Webhook Gateway]
            WA[WhatsApp] --> W
            EM[Email/Forms] --> W
        end

        subgraph Processing Pipeline
            W --> NB[Normalized Event Bus (Kafka/Redis Stream)]
            NB --> IR[AI Intent Router (Gemini)]
        end

        subgraph Departments
            IR -->|Sales Intent| SA[Sales Agent]
            IR -->|Support Intent| CS[CS Agent]
            IR -->|Ops Intent| OA[Operations Agent]
        end

        subgraph Owner Presentation
            SA --> TQ[(Triage Queue DB)]
            CS --> TQ
            OA --> TQ
            TQ --> U[Mobile UI: Unified Priority Inbox]
        end
    ```
  - **Mobile UX Flow (375px):**
    1.  **Home Screen:** Shows a unified "Attention Needed" card. E.g., "3 New Inquiries, 1 Urgent Delivery Change."
    2.  **Triage Feed:** A clean, vertical list. High-priority items (like a delivery change for *today*) are at the top, styled with a distinct status token.
    3.  **Detail View:** Tapping a message shows the customer's history (e.g., "Maya's custom cake from last month"), the new message, and a pre-drafted response by the relevant Agent (e.g., "Sales Agent drafted a $50 quote").
    4.  **Action:** The owner simply taps "Approve & Send" or edits the text native-mobile style.
  - **AI Agent Integration Points:**
    -   `Intent Router`: Fast, low-latency LLM call to classify the raw message (Sales, Support, Ops, Spam).
    -   `Context Synthesizer`: Pulls the customer profile and relevant business state (inventory, calendar).
    -   `Response Drafter`: Specific department agent drafts the reply and queues it for owner approval.
  - **Key Design Decisions:**
    -   **Normalize Early:** All incoming messages, regardless of source, must be converted into a standard `OmniMessage` protobuf/struct before AI processing.
    -   **Asynchronous Processing:** Triage is backgrounded via the PostgreSQL job queue. The mobile app receives WebSocket or Push notifications when an item is ready for review.
    -   **Glassmorphism UI:** The Triage Feed will utilize the OHC Premium Token library, presenting information in readable, translucent cards that feel like a native iOS/Android assistant, not a spreadsheet.

  **Implementation Prompt:**
  Implement the backend core for the Universal Omnichannel Inbox.
  1.  Define the `OmniMessage` entity and database schema with tenant isolation.
  2.  Create the `IntentRouter` service that takes an `OmniMessage` and uses the LLM integration to assign a category and urgency.
  3.  Expose a REST/gRPC endpoint for the frontend to fetch the prioritized `TriageQueue`.
  4.  Ensure 100% unit test coverage for the routing logic. Focus on the data pipeline and routing logic; frontend UI will follow in a separate task.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
