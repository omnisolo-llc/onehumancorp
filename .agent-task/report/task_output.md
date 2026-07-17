issue_title: "Architectural Design: Multi-Channel Omni-Inbox & AI Auto-Drafting"
issue_description: |
  ## Title: Implement Unified Multi-Channel Omni-Inbox with AI Auto-Drafting

  ## Problem Statement
  Small business owners like Maya (the baker who receives cake inquiries via Instagram DMs) and Carlos (the handyman who gets service requests via SMS) struggle to juggle multiple communication channels. They lose track of inquiries, miss out on potential revenue, and spend hours drafting repetitive responses. They need a single, unified inbox where an AI assistant automatically triages messages, identifies the context, and drafts personalized replies (like a quote or deposit link) ready for their approval.

  ## Research Report
  - **Market Context**: Modern conversational commerce is highly fragmented across WhatsApp, Instagram, SMS, and Email.
  - **Competitive Analysis**:
    - **Shopify Inbox**: Centralizes chat but relies heavily on predefined quick replies and lacks deep AI contextualization for service/booking-based businesses.
    - **Wix Inbox / GoDaddy**: Provide unified feeds but require manual triage and drafting.
    - **HubSpot / Zendesk**: Powerful but too complex (admin-heavy) for a single operator on a mobile device.
  - **The OHC Opportunity**: By integrating directly with the "Customer & Relationship Assistant" and "Sales & Revenue Assistant", OHC can not only centralize messages but proactively draft quotes, bookings, and deposit requests based on the business's existing catalog and calendar.

  ## Design Doc
  ### High-Level Architectural Design
  - **Message Ingestion Layer**: Webhook endpoints and polling workers to ingest messages from external channels (Instagram Graph API, Twilio SMS, WhatsApp Business API).
  - **Unified Activity Feed**: A central data model that standardizes incoming messages into "Conversations" and "Events".
  - **AI Triage & Drafting Engine**: Upon ingestion, the Customer Assistant analyzes the intent, fetches context (past interactions, inventory, calendar), and generates a drafted response.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Channels[Instagram / SMS / WhatsApp] -->|Webhooks| IngestionAPI[Ingestion API]
      IngestionAPI --> JobQueue[(PostgreSQL Job Queue)]
      JobQueue --> Worker[Message Processor Worker]
      Worker --> CustomerAgent[Customer & Relationship Assistant]
      CustomerAgent --> Context[Memory & Context Store]
      CustomerAgent --> UnifiedFeed[(Unified Feed Database)]
      UnifiedFeed --> MobileUI[OHC Mobile Client - 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen (Triage Feed)**: A single feed showing urgent items. A new inquiry appears as an "Unread Message" card.
  - **Conversation View**:
    - Top: Customer details and source icon (e.g., Instagram).
    - Middle: Chat history.
    - Bottom: The AI-generated draft highlighted with a "Sparkle" icon, with an "Approve & Send" button and an "Edit" button.
  - **Mobile UX Flow**: Maya opens OHC -> sees a notification for a new cake inquiry -> taps to open the conversation -> sees the AI has already drafted "Hi! I can definitely do a vegan cake for Saturday. Here is the deposit link to confirm." -> Maya taps "Approve & Send" -> message is dispatched via Instagram DM.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Evaluates inbound message intent and context to draft the reply.
  - **Sales & Revenue Assistant**: Provides the deposit/payment link to include in the draft if intent is purchasing.
  - **Operations Assistant**: Provides calendar availability if the intent is booking/delivery.

  ### Key Design Decisions
  - **Draft-First Approach**: The AI should never send automatically on behalf of the user unless explicitly configured in advanced settings. The default is drafting for owner approval to maintain trust.
  - **Asynchronous Processing**: Webhooks must return 200 OK immediately and push the payload to the PostgreSQL Job Queue (`SKIP LOCKED`) to ensure reliability and handle rate limits without dropping messages.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the "Multi-Channel Omni-Inbox & AI Auto-Drafting" capability.
  - **Outcome**: The owner must see a unified feed of messages from various channels (simulated or real APIs) on their 375px mobile view. When a new message arrives, the system must use the Customer Assistant to analyze it and attach a drafted reply.
  - **CUJ**:
    1. A new message webhook is received (e.g., customer asking about a service).
    2. The message appears in the owner's UI feed.
    3. The owner opens the message and sees an AI-generated draft response.
    4. The owner clicks "Approve & Send", which marks the draft as sent and dispatches it back to the channel.
  - **Acceptance Criteria**:
    - Create the necessary webhook ingestion endpoints and worker queue processing.
    - Integrate the LLM (Customer Assistant) to draft replies based on message content.
    - Update the UI to display the unified feed and the AI draft approval interaction, adhering strictly to the mobile-first (375px) translucent glass design language.
    - Ensure all state mutations are verified with Playwright E2E tests simulating the full user journey.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
