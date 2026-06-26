issue_title: "Implement 'The Ambassador' Agent - Native Social Inbox Auto-Responder"
issue_description: |
  # Research Report: The Ambassador Agent - Native Social Inbox Auto-Responder

  ## Problem Statement
  Solopreneurs like Maya (Home Baker) miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience. They need an automated DM response system where the AI agent drafts replies based on inventory and business rules, which they can review and approve directly from their phone.

  ## Research Report
  - **Market Context:** Traditional platforms (Shopify, Wix) require third-party apps for social media auto-responses. These apps are complex and often lack deep integration with the core business data (inventory, orders).
  - **OHC Differentiation:** OHC's "Ambassador" agent acts proactively. It reads incoming DMs, uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and policies, and drafts a contextual reply. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Customer Identity Resolution Engine}
      C -->|Lookup| D[Unified Customer Graph DB]
      C --> E[Event Mesh]
      E --> F[The Ambassador Agent]
      F -->|Query Context| D
      F -->|Draft Reply| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> A
  ```

  ### Mobile UX Flow (375px First)
  1. Maya logs into the OHC mobile web app (375px view).
  2. Maya connects her Instagram Business account via the Integrations tab.
  3. A customer DMs Maya: "Do you have vegan chocolate cake available for Saturday?"
  4. The Ambassador Agent queries Maya's OHC inventory, confirms availability, and drafts a reply.
  5. Maya receives a push notification and sees a card on her Home Feed: "Agent drafted a reply to @customer. Tap to review."
  6. Maya taps the card, sees the draft ("Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?"), and taps "Approve & Send".
  7. The message is sent via the Omnichannel Dispatcher.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies. Needs to integrate with the AI Job Queue and LLM Provider (Gemini Pro).

  ### Key Design Decisions
  - **Read-Approve over Read-Reply:** Move from read-reply to read-approve. The AI drafts the response before the user opens the app.
  - **Mobile-First Approval:** The UI must be optimized for 375px, making approval a 1-tap action.

  ## Implementation Prompt
  Implement the core backend flow for "The Ambassador" agent. This includes:
  1. An endpoint/webhook handler for incoming social messages (Omnichannel Gateway).
  2. A worker/job to process these messages, classify intent, and use the LLM (Gemini Pro) to draft a response based on tenant context.
  3. Placing the drafted response into an "Action Required Queue" for the owner to review.
  4. An endpoint for the owner to approve/edit/discard the draft, which then dispatches the message.

  Do NOT prescribe specific database schemas or API signatures. Ensure the flow supports multi-tenancy and is resilient (use the AI Job Queue pattern). Include Playwright E2E tests simulating the webhook ingestion and the user's mobile UI approval flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
