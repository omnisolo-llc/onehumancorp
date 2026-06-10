issue_title: "Implement 'The Ambassador' Agent - Native Social Inbox Auto-Responder"
issue_description: |
  # Research Report: "The Ambassador" Native Social Inbox Auto-Responder

  ## 1. Problem Statement
  Non-technical business owners, such as "Maya the Home Baker", are overwhelmed by the volume of inquiries across different channels like Instagram DMs or WhatsApp. They miss critical sales opportunities because they are engaged in physical operations and cannot constantly monitor messages. Traditional aggregation tools only combine inboxes without providing context-aware, proactive assistance. They leave the burden of crafting personalized responses on the owner.

  ## 2. Research Report
  - **Competitive Gap**: Existing platforms (e.g., Shopify Inbox, Wix Inbox) aggregate messages but act merely as rigid auto-replies or require the user to write out the context. Tools like Zendesk or Intercom are far too complex and costly for solopreneurs. Link-in-bio tools lack the depth for an e-commerce or booking setup.
  - **The Solution**: An "Invisible AI Agent" that not only aggregates messages but resolves customer identity across channels, queries business data (inventory, policies, FAQs), and proactively drafts highly context-aware replies. The owner shifts from "writing messages" to simply "approving drafts" via a 375px mobile-first interface.
  - **Persona Fit**: Maya (Baker) who relies on Instagram DMs. When asked "Do you have vegan chocolate cake available for Saturday?", the system automatically checks inventory and drafts "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?". Maya taps "Approve".

  ## 3. Architecture & Design Doc

  ### Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Instagram / WhatsApp Webhook] --> B(Omnichannel Gateway)
      B --> C{Identity Resolution Engine}
      C --> D[Event Mesh]
      D --> E[The Ambassador Agent]
      E -->|RAG / Lookup| F[(Tenant Database / Inventory / FAQs)]
      E -->|Intent & Context| G[LLM Draft Generator]
      G --> H[Action Required Queue]
      H --> I[Mobile Agent Feed 375px]
      I -->|User Approves| J[Omnichannel Dispatcher]
      J --> A
  ```

  ### Core Components
  1.  **Event Ingestion & Routing**: A unified webhook receiver that standardizes incoming messages into a common event format.
  2.  **Identity Resolution**: Link social handles or incoming phone numbers to a unified customer record within the tenant scope.
  3.  **The Ambassador Agent (LLM Integration)**: RAG against the user's specific business context to understand intent (e.g., pricing, availability, support) and draft a personalized response.
  4.  **Mobile UX (375px) Flow**:
      -   Push notification alerts the owner.
      -   The OHC app displays an "Action Card" in the Unified Agent Feed.
      -   The card shows the customer's message, the relevant context (e.g., previous purchase history), and the AI-drafted reply.
      -   Primary action: Large, touch-friendly "Approve & Send" button. Secondary action: "Edit".
  5.  **Multi-Tenant Isolation**: Strict enforcement of tenant boundaries when looking up inventory and customer history.

  ## 4. Implementation Prompt
  **Feature Name**: The Ambassador - Native Social Inbox Auto-Responder
  **Target Persona**: Maya the Home Baker
  **User-Facing Outcome**: When a customer DMs Maya on Instagram asking about cake availability or past orders, she opens the OHC app to find a pre-written, perfectly accurate response already drafted in her Agent Feed. She taps one button to send it, reducing response time to seconds.

  **Acceptance Criteria & Next Steps**:
  1.  **Backend Services**: Create an Omnichannel Gateway service to ingest messages (simulated via webhook for initial implementation).
  2.  **Identity & RAG**: Implement the RAG pipeline that queries the tenant's inventory and customer history based on the incoming message to generate the draft.
  3.  **Agent Logic**: The Ambassador Agent should produce an "Action Required" item in the tenant's feed.
  4.  **Frontend Mobile UX**: Develop the "Action Card" for the 375px mobile view containing the draft and "Approve/Edit" buttons.
  5.  **Testing**: Build a full Playwright E2E test simulating a customer inquiry, the agent drafting the response, the owner approving it on mobile, and the simulated dispatch of the reply.

  ## 5. Priority & Scope
  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
