issue_title: "Implement The Ambassador Agent (Omnichannel Customer Success)"
issue_description: |
  ## The Ambassador Agent: Omnichannel Customer Success Memory

  ### Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require manual typing. They don't proactively draft contextual responses based on full customer history. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ### Research Report
  - **Shopify Inbox:** Aggregates chat and email but relies on manual responses or rigid auto-replies. Does not proactively draft contextual responses.
  - **Wix Inbox:** Good aggregation, but AI is limited to "improving tone".
  - **Zendesk/Intercom:** Enterprise-grade, too complex and expensive for single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Ambassador doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ### Design Doc
  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  #### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  #### AI Agent Integration Points
  - **Trigger:** Webhook from Omnichannel Gateway.
  - **Context Gathering:** Agent queries the `Unified Customer Graph DB` for recent orders, tags (e.g., "vegan"), and past conversations.
  - **Drafting:** Agent uses a prompt incorporating the owner's tone and business rules to draft a reply.
  - **Approval:** Draft is placed in the `Action Required Queue` for owner review.

  ### Implementation Prompt
  Implement the core architectural foundations for the "Omnichannel Customer Success Memory" (The Ambassador Agent).
  - Define the data model for the `Unified Customer Graph DB`, ensuring strict multi-tenant isolation (`tenant_id`).
  - Create the API endpoints or gRPC services for the `Omnichannel Gateway` to receive incoming messages.
  - Implement the core logic for the `Customer Identity Resolution Engine` to match incoming messages to existing customer profiles based on phone numbers, emails, or social handles.
  - Design the `Action Required Queue` using PostgreSQL `SKIP LOCKED`.
  - Create a mock or stub for `The Ambassador Agent` that listens to the `Event Mesh`, queries the database, and creates a draft reply in the queue.
  - Ensure all database schemas have strict row-level security enabled for `tenant_id`.
  - Write unit and E2E tests for the implemented data models and services.
  - Note: Actual integration with Instagram, WhatsApp, or LLM providers is out of scope for this initial foundation task. Focus on the internal architecture, data models, and queue mechanisms.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
