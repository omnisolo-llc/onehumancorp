issue_title: "[Research] Build The Ambassador Agent: Automated DM Drafts & Feed Workflow"
issue_description: |
  # Research Report: The Ambassador Agent & Mobile-First Feed Notification Flow

  ## Target Persona: Maya (Home Baker)
  **Background:** Maya sells custom cakes and manages orders heavily via Instagram DMs.
  **The Problem:** Solopreneurs miss critical sales opportunities because they cannot constantly monitor and contextually respond to social media messages while performing physical operations. Traditional chatbots are often viewed as impersonal and complex to set up.

  ## Competitive Analysis
  Unlike Shopify or Wix, which often rely on complex third-party marketplace apps (like ManyChat or Gorgias) that require intricate flowchart setups and monthly fees, OHC's approach is native and autonomous. Shopify's Sidekick only acts as an internal advisor, whereas the OHC Ambassador Agent executes directly on behalf of the user. Squarespace lacks any robust native agentic workflow for social commerce.

  ## Proposed Architecture & Design Flow
  The goal is to seamlessly unify message ingestion with intelligent LLM classification and drafting, ultimately surfaced via a mobile-first "Agent Feed" card where the owner only needs to tap "Approve".

  ### Architecture Diagram
  ```mermaid
  graph TD
      IG[Instagram Graph API / Webhook] --> |Incoming DM| Ingestion[Event Ingestion Service]
      Ingestion --> PubSub[Redis Pub/Sub Bus]
      PubSub --> Worker[Agent Processing Worker]
      Worker --> |1. Intent Classification| LLM[Gemini/MiniMax/OpenAI]
      Worker --> |2. Retrieve Inventory/Policies| RAG[Database & Vector Store]
      RAG --> LLM
      LLM --> |Generate Draft| FeedDB[PostgreSQL `ohc:feed`]
      FeedDB --> UI[OHC Mobile App Feed]
      UI --> |Owner Taps 'Approve'| Sender[Chat Service Webhook]
      Sender --> |Final Message| IG
  ```

  ### Mobile-First UX Flow
  - **Feed Push:** The draft response is persisted as a pending "Action Task" or "Notification" in the `ohc:feed` for the specific tenant.
  - **User Interface:** The OHC Flutter/Tauri app displays an action card in the main Feed optimized for 375px viewports.
    - Card features the original message context.
    - Displays the LLM-generated draft using translucent glass styling.
    - Offers clean, 44x44px touch targets for **Approve & Send**, **Edit**, and **Discard**.
  - **Resolution:** Tapping "Approve" dispatches the final message via the platform's chat service.

  ### Dogfooding & UI Verification Plan
  Before declaring this feature complete, the implementer must:
  1. Boot the full `docker compose up --build` local stack.
  2. Log into the OHC web/mobile client as a seeded user representing the "Maya" persona.
  3. Simulate an incoming webhook event (e.g., via a CURL script representing an IG DM).
  4. Verify the new Action Card appears in the visible home Feed.
  5. Verify that clicking "Approve" correctly updates the UI state, transitions the card to "Completed", and correctly records the final action without any console or network errors.

  ## Implementation Prompt
  - Integrate a webhook pipeline capable of capturing message events.
  - Implement intent classification using the built-in LLM provider.
  - Implement RAG retrieval for context building from the store's inventory and policies.
  - Build the mobile-first (375px) feed notification card UX for draft approval.
  - Do NOT prescribe strict database schemas. Focus on the seamless connection between the webhook, the LLM draft generation, and the user's mobile feed approval flow.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
