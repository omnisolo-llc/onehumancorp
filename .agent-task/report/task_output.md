issue_title: "Implement Intelligent Customer Auto-Responder Agent"
issue_description: |
  # Research Report: Intelligent Customer Auto-Responder Agent

  ## Problem Statement
  Small business owners (like Maya the baker or Carlos the handyman) lose potential sales and waste time because they cannot monitor their communication channels (Instagram DMs, email, website chat) while performing their actual work. They need an invisible assistant that automatically replies to common inquiries (e.g., "Do you do vegan cakes?", "What are your hours?", "Where is my order?") without manual intervention, allowing them to focus on operations while maintaining high customer satisfaction and capturing leads.

  ## Research Report
  Our competitive analysis and market research highlight that current solutions are either too complex (Zendesk, Intercom), require manual rule-building (ManyChat), or are merely chatbots that advise rather than execute (Shopify Sidekick). OHC has the opportunity to leapfrog these by providing an **autonomous auto-responder** that directly queries the tenant's business data (inventory, FAQs, order history) via a RAG pipeline to generate and propose responses.

  ### Key Findings
  - **Instagram DM Overload** is the second most cited pain point (38%) among the target demographic.
  - Business owners want to shift from "read-and-reply" to "read-and-approve" workflows.
  - The feature must be entirely mobile-first, presenting actionable cards on a 375px viewport.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Incoming Message Webhook/Event] --> B(Intent Classifier - LLM)
      B --> C{Context Retrieval - RAG}
      C -->|Query| D[Tenant DB: Inventory, FAQs, Orders]
      D --> E(Draft Generator - LLM)
      E --> F[Action Card Queue]
      F --> G[Mobile App Feed]
      G -->|User Approves| H[Message Dispatcher]
  ```

  ### AI Agent Integration
  The core of this feature relies on the `Ambassador` agent.
  - **Trigger**: An incoming message event is placed onto the job queue.
  - **Processing**: The agent uses the configured LLM (e.g., Gemini Pro) to classify intent.
  - **Context**: The agent queries the tenant's data (using vector search if available, or direct DB queries) to find relevant information (e.g., checking if "vegan cake" is in the inventory).
  - **Output**: The agent drafts a contextual reply and creates a pending action item in the database, targeted for the mobile feed.

  ### Mobile UX Flow (375px First)
  1. The user opens the OHC app and sees a feed of "Action Cards".
  2. A new card appears: "Drafted Reply to @customer123".
  3. The card displays the original message ("Do you do vegan cakes?") and the AI-drafted reply ("Yes, we do! Our vegan chocolate cake is currently available. Would you like to place an order?").
  4. The card has three prominent touch targets (>= 44x44px): "Approve & Send", "Edit", and "Discard".
  5. The UI uses the defined Glassmorphism / clean dashboard style.

  ## Implementation Prompt
  As an Implementer Agent, you are tasked with building the end-to-end flow for the Intelligent Customer Auto-Responder.

  **Outcome:** An incoming message event automatically triggers an LLM-based agent that retrieves relevant context from the tenant's data, drafts a reply, and surfaces this drafted reply as an actionable card in the user's mobile feed for approval.

  **Acceptance Criteria (CUJ):**
  1. Create the necessary backend event handlers and agent logic to process an incoming message.
  2. Implement the context retrieval (RAG or direct query) to inform the LLM's response based on the tenant's catalog/FAQs.
  3. Update the frontend feed to display these drafted responses as actionable cards.
  4. Provide a full Playwright E2E test where a mock message is injected, the user logs in, sees the drafted response card on a mobile viewport (375px), clicks "Approve", and the system records the approval.
  5. Ensure 100% unit test coverage for new backend and frontend logic. No mock data in the UI; data must flow from the backend.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
