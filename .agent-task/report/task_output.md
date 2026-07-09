issue_title: "[Architecture] Autonomous Dynamic Service Quoting & Intake Engine"
issue_description: |
  ## Title
  Autonomous Dynamic Service Quoting & Intake Engine

  ## Problem Statement
  Service-based owners like Carlos (Handyman) and Nora (Agency Principal) spend hours manually communicating with leads, gathering requirements, and drafting quotes or proposals. They often lose potential clients because they are busy in the field and cannot respond fast enough. Existing tools like Jobber, Housecall Pro, or HoneyBook require manual data entry, rigid forms, and human intervention to generate a quote. These owners need an invisible assistant that autonomously conducts conversational intake via SMS/WhatsApp, calculates a quote based on historical data and pricing rules, and sends an actionable proposal—while they are out on a job.

  ## Research Report
  **Findings, competitive analysis, data, references:**
  - **Jobber / Housecall Pro:** Industry standards for field services, but they rely heavily on static forms and require the owner to manually review requests and build quotes. They lack conversational AI intake.
  - **HoneyBook / Dubsado:** Great for creatives (like Nora), but still require manual effort to read a client inquiry, select a brochure/proposal template, and adjust pricing.
  - **Shopify / Wix:** Primarily built for standard products, making custom service quoting extremely clunky or requiring expensive third-party apps.
  - **OHC Opportunity:** The "Sales & Operations Assistant" agents can seamlessly interact with leads across channels (WhatsApp, Web Chat, Instagram DMs). They use Large Language Models connected to the owner's service catalog, pricing rules, and availability calendar to autonomously negotiate scope, generate a dynamic quote, and secure a deposit. The owner simply reviews an "Action Required: Approve $850 Quote for Sink Repair" card on their 375px mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer SMS / WhatsApp / Web Chat] -->|Webhook| B(Omnichannel Inbox Gateway)
      B --> C{Intake & Triage Agent}
      C -->|Extracts intent & requirements| D[Sales & Quoting Agent]
      D -->|Reads pricing rules| E[(Service Catalog & Pricing DB)]
      D -->|Checks availability| F[(Scheduling Calendar)]
      D -->|Drafts Quote| G[Quote Engine]
      G --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px - Owner UI]
      I -->|Owner 1-Tap Approve| J[Omnichannel Dispatcher]
      J -->|Sends secure Stripe payment link| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px first)
  1. **Notification:** Owner (Carlos) receives a push notification: "New Lead: John wants a sink replaced."
  2. **Feed Card:** Opening the app shows a highly visible, translucent glass-styled card at the top of the feed: "Draft Quote Ready: $250 for John's Sink Replacement. Includes 2 hours labor and parts."
  3. **Interaction:** Tapping the card expands it to show the AI-conducted conversation summary (no need to read 20 messages).
  4. **Action:** A prominent primary button "Approve & Send Quote" allows instant dispatch. A secondary button "Edit Quote" opens a simple numerical pad to adjust the price.
  5. **Post-Action:** Once approved, the card updates to "Quote Sent - Awaiting Deposit".

  ### AI Agent Integration Points
  - **Intake & Triage Agent:** A lightweight LLM prompt optimized for entity extraction (Service Type, Urgency, Location, Photos).
  - **Sales & Quoting Agent:** An agent armed with RAG (Retrieval-Augmented Generation) over the owner's past quotes to estimate pricing accurately, hooked into the `Service Catalog` tool.

  ### Key Design Decisions & Why
  - **Human-in-the-Loop by Default:** Autonomously sending quotes could result in underpricing. The system drafts the quote but requires 1-tap owner approval unless "Auto-Approve" rules are explicitly set.
  - **Conversational Form Replacement:** Instead of sending leads to a rigid web form, the AI asks sequential, natural questions in the chat thread.
  - **Zero Trust & Security:** Multi-tenant isolation applies to the Service Catalog and Pricing DB. Agents only retrieve data scoping to the authenticated owner's `tenant_id`.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend logic and the mobile-first UI for the Autonomous Dynamic Service Quoting Engine.
  1. Create a `QuotingAgent` service that listens to the Omnichannel Gateway for new service inquiries.
  2. The agent must parse the inquiry, consult the service catalog, and generate a draft quote object (price, breakdown, description).
  3. Insert this draft into the `Action Required Queue`.
  4. Build the Flutter/Web mobile UI (375px width optimized, translucent glass style) that displays the "Action Required" card on the home feed.
  5. Implement the "Approve & Send" interaction, which marks the quote as sent and generates a payment link.
  Acceptance Criteria: A non-technical owner can log in, see a pending quote generated from a chat inquiry on their dashboard, and approve it in one click without manually typing line items.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
