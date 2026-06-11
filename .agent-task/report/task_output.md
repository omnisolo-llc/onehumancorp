issue_title: "Implement Intelligent Customer Auto-Responder"
issue_description: |
  # Intelligent Customer Auto-Responder (P0)

  ## Problem Statement
  Small business owners like Maya the Baker or Carlos the Handyman are overwhelmed by common customer inquiries via Instagram DMs, WhatsApp, and website chat. They lose sales and time answering repetitive questions like "Do you do vegan cakes?" or "What are your hours?" They need an intelligent system that intercepts these queries, determines the intent, and replies automatically using their business context, allowing them to focus on their actual work.

  ## Research Report
  - **Competitor Analysis:** Shopify requires 3rd party apps (like Gorgias) for this, which are complex and expensive. Wix and Squarespace have limited chatbot features that require manual rule building.
  - **Market Gap:** SMBs need a zero-configuration, "it just works" AI responder. It must learn from their existing documentation, FAQs, and product catalogs without the user explicitly creating branching dialogue trees.
  - **Core Value:** Immediate time savings and captured leads that would otherwise bounce due to slow response times.

  ## Design Doc
  ### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
      IncomingMessage[Incoming Customer Message (DM, Chat)] --> IntentAnalyzer[Agent: Intent Analyzer]
      IntentAnalyzer --> ContextEngine[Agent: Context Retrieval Engine]
      ContextEngine --> BusinessData[(Tenant Knowledge Base / Products / Orders)]
      ContextEngine --> AutoResponder[Agent: Auto Responder Drafts Reply]
      AutoResponder --> ActionGateway{Confidence Check}
      ActionGateway -- High Confidence --> SendMessage[Send Automatic Reply]
      ActionGateway -- Low Confidence / Complex --> Escalate[Draft for Owner Approval via UI Feed]
  ```

  ### Mobile UX Flow (375px)
  - **Setup:** A simple "Auto-Responder: ON/OFF" toggle in the app settings. A section to optionally review "Learned Knowledge" (extracted from website/catalogs).
  - **Daily Operations:** The owner opens the app.
    - **High-confidence replies:** Shown in a collapsed summary view ("AI handled 15 messages today").
    - **Low-confidence or escalated messages:** Appear prominently in the **Work Triage Feed** with a pre-drafted suggested reply. The owner can tap to edit or tap a single "Approve & Send" button (min 44x44px touch target).
  - **Visuals:** Use the OHC Premium Token library (Translucent Glass styling, clear Ubiquiti-style hierarchy).

  ### AI Agent Integration
  - Utilize `Gemini Pro` for intent analysis and response generation.
  - Implement a tenant-scoped memory store (vector DB) to hold FAQs, product details, and past successful interactions to ground the AI responses.
  - Implement strict guardrails: the AI must never make up pricing or promise services not explicitly listed in the tenant's data.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the core backend infrastructure and mobile-first UI for the "Intelligent Customer Auto-Responder".
  - Create the necessary API endpoints and database schema extensions to store AI agent settings and tenant knowledge contexts.
  - Build the agent logic to parse incoming messages, query the knowledge base, and generate responses.
  - Develop the mobile-first UI (Tauri/Flutter/React depending on current stack) that allows the owner to toggle the responder and view/approve drafted messages in their feed.
  - The UI must perfectly fit a 375px screen, use translucent design tokens, and have zero mock data. Ensure E2E Playwright tests cover the flow of receiving a message and approving a draft.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
