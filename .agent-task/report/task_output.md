issue_title: "Implement The Ambassador Agent & Unified Mobile Intake Flow"
issue_description: |
  ## Title
  Implement The Ambassador Agent & Unified Mobile Intake Flow

  ## Problem Statement
  Solopreneurs like Maya (the baker) and Carlos (the handyman) are missing critical sales opportunities because they cannot simultaneously execute physical operations and monitor social media DMs (Instagram/WhatsApp). Existing solutions rely on complex logic builders (e.g., ManyChat) or "advice-only" chatbots (e.g., Shopify Sidekick) which do not execute work autonomously and are too technical to configure.

  ## Research Report
  - **Market Dynamics**: Research confirms a massive gap between enterprise solutions (Shopify, BigCommerce) which require deep technical configurations, and low-end tools (GoDaddy, Wix) which offer basic websites without operations integration.
  - **The "App Tax"**: SMBs hate piecing together fragmented systems (e.g., Shopify + Klaviyo + Calendly). OHC needs a single unified ecosystem.
  - **Zero-Setup Vision**: OHC must provide "invisible automation". Instead of a manual configuration panel, the LLM should act as a functional department (Customer Success Agent).
  - **Current Pain Points**: A user (like Maya) gets 10 DMs a day asking "Do you have vegan cake today?" and has to stop baking to reply. The Ambassador agent needs to answer this based on the existing inventory state.

  ## Design Doc
  ### Mobile UX Flow (375px Target)
  1. Maya connects her Instagram via the Integrations card on her mobile dashboard.
  2. The unified Feed component generates an "Action Card" when an incoming DM is resolved.
  3. The card displays: "Drafted reply to @user based on availability: 'Yes we have 3 vegan cakes left!'"
  4. Buttons: [ Approve & Send ] (primary, #0066FF), [ Edit ] (secondary), [ Discard ] (ghost).

  ### System Architecture
  ```mermaid
  sequenceDiagram
      participant User (IG)
      participant OHC Webhook
      participant Intent Classifier
      participant RAG Store
      participant Action Feed
      participant Maya (Mobile)

      User (IG)->>OHC Webhook: "Do you have vegan cake?"
      OHC Webhook->>Intent Classifier: Classify incoming message
      Intent Classifier-->>RAG Store: Retrieve Inventory & FAQs
      RAG Store-->>Intent Classifier: Vegan Cake: 3 in stock
      Intent Classifier->>Action Feed: Generate Action Card
      Action Feed->>Maya (Mobile): Push Notification
      Maya (Mobile)->>Action Feed: Tap "Approve"
      Action Feed->>User (IG): Send "Yes we have 3!"
  ```
  - **Multi-Tenant State**: Implement state updates safely using tenant-scoped context and locks to ensure inventory counts do not oversell during generation.

  ### AI Agent Integration Notes
  - The agent should use `minimax.reason()` (or fallback to OpenAI) for initial intent parsing and drafting.
  - The RAG context should be populated from the `inventory` and `knowledge` modules of the user's workspace.

  ## Implementation Prompt
  - Create the backend webhook handler to process incoming social messages (Instagram Graph API format) and route them to the new `Ambassador Agent`.
  - Implement the `Ambassador Agent` logic to classify intent, fetch RAG context (inventory availability), and produce a drafted `Action Card`.
  - Build the corresponding Mobile-First (375px viewport) UI feed component in Flutter/Tauri to display the drafted replies with "Approve", "Edit", and "Discard" actions.
  - Do NOT hardcode the database schema; rely on the existing multi-tenant entity models. Ensure E2E Playwright tests cover the "Approve" happy path.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
