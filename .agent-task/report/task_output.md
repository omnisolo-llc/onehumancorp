issue_title: "Implement The Ambassador Agent - Mobile-First AI Social Inbox Auto-Responder"
issue_description: |
  # The Ambassador Agent: AI Social Inbox Auto-Responder

  ## Problem Statement
  Solopreneurs and micro-SMB owners (like Maya the baker) spend hours manually replying to the same customer inquiries (pricing, availability, booking links) across social media DMs (Instagram, WhatsApp). They miss critical sales because they cannot monitor DMs while executing their physical operations. Existing solutions like Shopify rely on reactive chatbots or require complex logic builders (ManyChat) which are too technical for the OHC target audience.

  ## Research Report
  - **Competitor Analysis**: Shopify provides "Sidekick" (a reactive chatbot for admin advice) and relies on third-party apps for automated messaging. Link-in-bio tools (Linktree, Beacons) lack conversational commerce capabilities entirely.
  - **The OHC Differentiator**: "Invisible AI Automation." The Ambassador Agent doesn't wait for the user to configure complex rules. It ingests social messages, classifies intent, checks inventory context, and drafts replies for a simple 1-tap approval in a mobile-first UI feed.

  ## Design Doc

  ### Architecture Diagram (Concept)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Webhook (IG/WA)
      participant Ambassador Agent (LLM + RAG)
      participant Postgres/Redis
      participant OHC Mobile App (Owner)

      Customer->>OHC Webhook: DM: "Do you have vegan cake?"
      OHC Webhook->>Ambassador Agent: Ingest Message Event
      Ambassador Agent->>Postgres/Redis: Check Inventory/Context
      Ambassador Agent-->>Ambassador Agent: Draft Response
      Ambassador Agent->>OHC Mobile App: Push Action Card to Agent Feed
      OHC Mobile App-->>Owner: Notification: "Review drafted reply"
      Owner->>OHC Mobile App: Tap "Approve & Send"
      OHC Mobile App->>Ambassador Agent: Approval Confirmed
      Ambassador Agent->>OHC Webhook: Send Response API Call
      OHC Webhook->>Customer: DM: "Yes, we have 3 left!"
  ```

  ### Mobile UX Flow (375px First)
  1. Owner receives a native mobile push notification or sees a notification badge in the OHC App.
  2. Opening the app lands on the "Agent Feed" (a unified inbox/task list).
  3. The feed displays an "Action Card" for the incoming message. The card includes:
     - Customer name/avatar and the original message.
     - The AI-drafted reply clearly visually distinct.
     - Large (44x44px minimum) touch targets for: "Approve & Send", "Edit", and "Discard".
  4. Tapping "Approve & Send" immediately clears the card and dispatches the message.

  ### AI Agent Integration Points
  - **Intent Classification**: Use Gemini Pro to analyze incoming webhook text to determine if it's an actionable inquiry.
  - **Context Retrieval**: Query the tenant's database for inventory, pricing, or FAQ policies.
  - **Drafting**: Use the context to formulate a friendly, brand-aligned response.

  ## Implementation Prompt
  **Outcome:** Implement the core event ingestion and agent drafting loop for "The Ambassador", presenting the drafted replies as actionable cards in the owner's mobile Agent Feed.

  **Critical User Journey (CUJ):**
  1. Connect a simulated social webhook source (for development/testing).
  2. Ingest an incoming message event (e.g., "Do you have vegan chocolate cake available for Saturday?").
  3. The Ambassador agent processes the message, queries inventory, and drafts a reply.
  4. The owner views the OHC mobile app (Agent Feed view).
  5. The owner sees the drafted reply card.
  6. The owner taps "Approve & Send".
  7. The system logs the sent message successfully.

  **Acceptance Criteria:**
  - Build the backend event ingestion and AI drafting service for the Ambassador Agent.
  - Implement the frontend "Action Card" UI in the Agent Feed, adhering strictly to 375px mobile width constraints and Premium Glassmorphism tokens.
  - Interactive elements must have >= 44x44px touch targets.
  - Add comprehensive E2E Playwright tests verifying the ingestion -> drafting -> approval flow.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
