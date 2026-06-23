issue_title: "[Research] AI Unified Inbox & Agentic Triage"
issue_description: |
  # Research Report: AI Unified Inbox & Agentic Triage

  ## 1. Problem Statement
  Small business owners and solopreneurs (like Maya the Baker or Carlos the Handyman) are drowning in fragmented communication channels. Inquiries, booking requests, and support questions are scattered across Instagram DMs, WhatsApp, SMS, email, and website forms. This fragmentation leads to missed opportunities, delayed responses, and significant cognitive overload. Existing tools (like Zendesk or Intercom) are too complex and expensive, while native platform tools (like Shopify Inbox) lack deep, cross-channel AI integration that actually *does the work* of triage and response drafting.

  ## 2. Research Report
  - **Market Context**: The SMB market is highly dependent on mobile communication. WhatsApp and Instagram are primary sales channels in many regions (LATAM, Asia) and growing in the US.
  - **Competitor Gaps**:
    - *Shopify Inbox*: Consolidates some channels but requires manual response or simple keyword-based auto-replies. Sidekick is reactive.
    - *Zendesk/Intercom*: Enterprise-focused, high cost, steep learning curve.
    - *Meta Business Suite*: Consolidates FB/IG but doesn't integrate with business operations (inventory, bookings, payments).
  - **The OHC Opportunity**: OHC can provide a truly unified, AI-first inbox. Instead of just showing the messages, the OHC "Work Triage" engine uses the LLM layer to classify the intent, query the business context (RAG on policies, inventory, calendar), and present the owner with a drafted response or action card (e.g., "Approve Quote," "Send Payment Link"). This shifts the paradigm from "reading messages" to "approving actions."

  ## 3. Design Doc

  ### Architecture Overview (Mermaid)
  ```mermaid
  graph TD
      IG[Instagram DMs] -->|Webhook| Ingest[Omnichannel Ingestion API]
      WA[WhatsApp] -->|Webhook| Ingest
      Email[Email] -->|Webhook| Ingest
      Web[Web Form] -->|API| Ingest

      Ingest --> Queue[PostgreSQL AI Job Queue]

      Queue --> Triage[Work Triage Worker]
      Triage -->|Extract Intent & Context| LLM[Gemini Pro/GPT-4o]
      Triage -->|Query State| DB[(Tenant DB: Inventory, Bookings)]

      LLM --> Draft[Generate Draft Response/Action]
      Draft --> ActionCard[Action Card Generator]

      ActionCard --> Feed[Owner Feed / Unified Inbox UI]
      Feed -->|1-Tap Approve| Dispatch[Omnichannel Dispatch API]
      Dispatch --> IG
      Dispatch --> WA
  ```

  ### Mobile UX Flow (375px First)
  1. **The Feed**: The app opens to a prioritized feed, not a standard inbox list. Urgent actionable items (e.g., "New Booking Request", "Refund Inquiry") are at the top.
  2. **The Action Card**: A user taps an item. Instead of an empty text box, they see:
     - The customer's message context.
     - An AI-generated summary of intent.
     - A drafted response ready to send.
     - Action buttons (e.g., "Send Draft", "Edit", "Generate Payment Link").
  3. **Approval**: Tapping "Send Draft" immediately dispatches the message back to the originating channel (e.g., Instagram DM) via the dispatch API.

  ### AI Agent Integration
  - **The Triage Agent**: Acts as the first line of defense. It classifies incoming messages (Sales, Support, Spam) and routes them to specialized agents if needed (e.g., passing a pricing question to the Sales Agent).
  - **The Customer Assistant Agent**: Drafts the actual replies based on tenant-scoped memory (previous interactions, business tone, policies).

  ## 4. Implementation Prompt
  **Feature Name**: OHC AI Unified Inbox & Agentic Triage
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya receives a DM on Instagram asking "Do you have vegan cupcakes for this Saturday?". The OHC app pushes a notification. When she opens it, she doesn't just see the message; she sees an action card that checked her calendar and inventory, and drafted the reply: "Yes, we do! I have 2 slots left for Saturday. Shall I send a deposit link?" She taps "Send."

  **Acceptance Criteria for Implementer**:
  1. Define the Omnichannel Ingestion data models (`Message`, `Conversation`, `Channel`, `Intent`). Ensure strict `tenant_id` isolation.
  2. Implement the `Work Triage Worker` that pulls from the AI Job Queue.
  3. Integrate the LLM call within the worker to classify intent and draft a response based on a simple mock context (e.g., mock inventory availability).
  4. Build the Mobile-First Feed UI: Display actionable cards with the drafted response and an "Approve" button.
  5. The UI must be fully functional at 375px width and utilize the translucent glass design tokens.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
