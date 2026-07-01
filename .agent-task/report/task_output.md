issue_title: "Implement 'The Ambassador Agent' - Mobile-First Social Inbox & Auto-Responder"
issue_description: |
  # The Ambassador Agent: Automated Customer Relationship Assistant

  ## Target Persona: Maya (Home Baker)
  Maya runs a custom cake business through Instagram DMs and referrals. She receives multiple inquiries daily (e.g., "Do you make vegan cakes?"). She needs an assistant that triages messages, cross-references her business data, and drafts context-aware replies so she doesn't lose leads while she is away from her phone.

  ## Problem Statement
  Solopreneurs like Maya miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience.

  ## Research Report
  - **Market Landscape & Competitor Analysis**:
    - **Shopify (Sidekick)**: Provides agentic help for the merchant's backend, but lacks a native, cross-platform social inbox interceptor for end-customers.
    - **Wix & Squarespace**: Offer basic automated email replies and contact form autoresponders, but do not provide a context-aware conversational agent for Instagram/WhatsApp DMs.
    - **GoDaddy (Airo)**: Focuses on initial brand setup and ads rather than daily customer service interactions.
    - **Lindy & 11x.ai**: Offer executive assistant and sales agents, but are often standalone platforms requiring complex integrations.
  - **The OHC Opportunity**: By integrating directly into the user's OHC app feed, we can provide a "Human-in-the-Loop" mobile-first experience. OHC's Ambassador agent will natively use the tenant's data (inventory, policies) to draft highly accurate replies, reducing response time from hours to seconds with just a single tap from the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Customer[Customer DM on Instagram/WhatsApp] -->|Webhook| Ingestion[Webhook API Layer];
      Ingestion --> JobQueue[(AI Job Queue: PG SKIP LOCKED)];
      JobQueue --> Worker[Async Worker];
      Worker --> Classifier[Intent Classification];
      Classifier --> RAG[RAG: Retrieve Inventory/Policies];
      RAG --> LLM[Gemini Pro: Generate Draft];
      LLM --> Feed[Owner Mobile Feed];
      Feed --> Owner((Maya));
      Owner -- Taps 'Approve' --> SendMessage[Dispatch Message to Customer];
  ```

  ### Architectural Flow & Integration Points
  1. **Data Ingestion (Webhook API Layer):** A unified webhook ingestion endpoint that securely accepts incoming messages from platforms (e.g., Instagram Graph API, WhatsApp).
  2. **Intent Classification & Context (AI/RAG Layer):**
     - LLM intent classification (Is this a pricing inquiry, availability check, or general support?).
     - The RAG pipeline queries the tenant's specific knowledge base, product inventory, and policies to build a precise context window.
  3. **Draft Generation (AI Draft Service):** The LLM generates a contextually accurate reply.
  4. **Mobile UX:** Pushes a notification to Maya. The OHC mobile app displays a 375px card showing the drafted message, with "Approve & Send", "Edit", and "Discard" actions.

  ### Key Design Decisions
  - **Human-in-the-Loop:** For critical communications (like custom pricing), the agent drafts the reply but requires the owner's explicit "Approve & Send" tap.
  - **Tenant Data Isolation:** Ensure the RAG query strictly scopes to the current `tenant_id` to prevent cross-contamination of business context.
  - **Mobile First**: All notifications are designed for a 375px viewport.

  ## Implementation Prompt
  - Integrate Instagram Graph API for message receiving/sending.
  - Implement intent classification using Gemini Pro.
  - Implement RAG retrieval for context building.
  - Build the mobile-first (375px) notification card UX for approval.
  - Do NOT prescribe database schemas here. Focus on the seamless connection between the webhook, the LLM, and the user's mobile feed.

  **Critical User Journey (CUJ):**
  1. Customer DMs Maya on Instagram asking about vegan cake availability.
  2. Instagram Graph API webhook triggers an event in the OHC backend.
  3. Event is processed: Intent classified as "availability inquiry".
  4. System queries Maya's inventory: Vegan cakes are in stock.
  5. LLM drafts response: "Yes, we have vegan cakes available! Would you like to order?"
  6. Action Card is pushed to Maya's OHC app feed.
  7. Maya taps "Approve" -> Response sent to customer via Instagram Graph API.

  **Acceptance Criteria:**
  - Build the backend webhook ingestion and draft-generation pipeline.
  - Build the frontend Action Card UI that handles the 3-state action (Approve, Edit, Discard).
  - Ensure the UI looks premium (macOS Translucent Glass style) and is fully responsive starting at 375px.
  - Ensure 100% unit test coverage for the new backend services and Playwright E2E tests verifying the entire mobile UI flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
