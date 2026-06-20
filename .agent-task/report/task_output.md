issue_title: "🔍 Scout: Tool Integration Research - Meta WhatsApp Cloud API"
issue_description: |
  ### Title
  Integrate Meta WhatsApp Cloud API for Unified Messaging and Work Intake

  ### Problem Statement
  Owners and operators, like Maya (Home Baker) and Carlos (Field Service), conduct a massive portion of their business via WhatsApp. Currently, these interactions live entirely in the owner's personal or business WhatsApp app, siloed from their calendar, task management, and payment systems. This requires the owner to manually copy context back and forth, leading to missed messages, forgotten follow-ups, and fragmented customer records. The owner needs a unified way to triage WhatsApp inquiries, draft replies using their OHC assistant, and seamlessly convert chats into actionable bookings or quotes without switching apps.

  ### Research Report
  **Market Need & Competitor Landscape:**
  - In global markets (LATAM, EMEA, India) and increasingly in the US, WhatsApp is the primary channel for B2C communication.
  - Competitors like HubSpot, Zendesk, and Shopify all heavily feature WhatsApp Business integrations. WeCom and DingTalk have built massive adoption primarily by centralizing chat as the business operating system.
  - Owners frequently complain on Reddit (r/smallbusiness) about the difficulty of sharing WhatsApp Business access with a team or an assistant without paying for expensive, complex tools like Trengo or Intercom.

  **Tool Deep-Dive: Meta WhatsApp Cloud API**
  - **Usability for OHC:** Meta's Cloud API provides a direct, scalable way to send and receive messages without hosting local WhatsApp infrastructure.
  - **Pricing:** The first 1,000 service conversations per month are free, making it highly viable for small businesses to start. After that, pricing is per conversation (varies by region).
  - **Capabilities:** Webhook integration for incoming messages, rich media support (crucial for sharing quotes and images), and template messages for proactive notifications (e.g., appointment reminders).
  - **SaaS Viability:** Perfect for Cloud (multi-tenant) via OAuth/embedded signup, and can be configured via a dedicated app token for Standalone (local) usage.

  ### Design Doc
  - **User-Facing Behavior:**
    - The owner connects their WhatsApp Business account via a simple "Connect WhatsApp" button in OHC Settings.
    - Incoming WhatsApp messages flow directly into the OHC "Work Triage" feed.
    - The Customer & Relationship Assistant reads incoming context, links the sender to an existing customer record (by phone number), and drafts a suggested reply.
    - The Operations Assistant can parse the message to suggest creating a task or booking.
    - The owner can review, edit, and send the AI-drafted reply directly from the OHC interface.
  - **Technical Flow:**
    - OHC provides a multi-tenant webhook endpoint (`/webhooks/whatsapp`) to receive incoming message events from Meta.
    - Webhook events are queued, routed to the correct tenant via the linked WhatsApp Business Account ID, and processed by the AI Job Queue.
    - Outbound messages from the owner (or authorized agents) trigger the Meta API.

  ### Implementation Prompt
  Implement a "Connect WhatsApp" integration that allows an owner to link their WhatsApp Business account to OHC. Once linked, incoming WhatsApp messages should appear in the owner's unified Work Triage feed. The AI Assistant should be able to read these messages, identify the customer based on their phone number, and draft a response for the owner to review and send. Ensure the setup process is simple for a non-technical user (no complex token copying if possible, using embedded signup) and that the UI handles rich text and media cleanly on a 375px mobile screen.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
