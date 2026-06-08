issue_title: "Automated Social Inbox Auto-Responder (The Ambassador)"
issue_description: |
  # Research Report: The Ambassador - Native Social Inbox Auto-Responder

  ## Problem Statement
  Small business owners like Maya the Baker receive customer inquiries across multiple unlinked channels, primarily Instagram DMs. Managing these manually leads to missed messages, slow response times, and lost sales. The owner spends hours replying to repetitive questions ("Do you have vegan cake?"). Traditional platforms like Shopify require third-party apps for this, increasing complexity and cost.

  ## Research Report
  - **Competitor Gaps**: Legacy builders either ignore this entirely or offer simple rule-based chatbots. They don't have agents that understand intent and access the actual product catalog and business policies autonomously.
  - **OHC Opportunity**: Provide "The Ambassador," a Customer Success Agent that monitors connected social channels (e.g., Instagram Graph API). It uses LLMs to determine intent and RAG to query the owner's inventory and FAQs. It drafts an accurate, polite response and pushes it to the owner's mobile device for one-tap approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram Webhook] --> B[OHC Ingestion API]
      B --> C[Intent Classifier]
      C --> D[The Ambassador Agent]
      D -->|Query| E[Inventory/FAQ DB]
      D -->|Draft Reply| F[Action Required Feed]
      F --> G[Owner Mobile App]
      G -->|Tap Approve| H[Dispatch API]
      H -->|Send| I[Instagram API]
  ```

  ### Mobile UX Flow
  - **Action Card**: Appears in the unified feed. "New DM from @customer asking about Vegan Cake. Draft prepared."
  - **Interaction**: Tap card -> View original message and drafted reply -> Tap "Approve" (sends message) or "Edit".

  ### AI Agent Integration Points
  - Triggered by incoming webhook.
  - Extracts intent from the message.
  - Queries `ProductCatalog` and `BusinessSettings` (FAQs, policies).
  - Generates a draft response.

  ### Key Design Decisions
  - **Review and Approve**: Instead of a fully autonomous bot that might make mistakes, the agent acts as an assistant, drafting the response for human approval.
  - **Unified Inbox**: The architecture supports multiple channels (Instagram, WhatsApp, SMS) using a common ingestion gateway.

  ## Implementation Prompt
  **Outcome**: Maya connects her Instagram account. When a customer asks about a specific product, she gets a notification with a pre-written, accurate reply. She taps "Approve" and the message is sent.

  **CUJ & Acceptance Criteria**:
  1. Simulated incoming webhook for an Instagram DM asking about product availability.
  2. System accurately identifies the product and checks inventory.
  3. Ambassador Agent drafts a polite response confirming availability (or lack thereof) and optionally includes a purchase link.
  4. Draft appears in the mobile UI feed.
  5. User taps "Approve".
  6. System simulates sending the response back via the API.
  7. End-to-end flow verified via Playwright E2E tests simulating the user interaction.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
