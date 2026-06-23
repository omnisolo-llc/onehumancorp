issue_title: "Integrate Twilio for WhatsApp Business to Unify Customer Messaging"
issue_description: |
  # Mission Queue Protocol: Twilio WhatsApp Integration

  ## Problem Statement
  For many owners like **Maya (Home Baker)**, **Carlos (Field Service)**, and **Fatima (Food Cart)**, WhatsApp is the primary channel for customer inquiries, orders, and service coordination. Currently, managing these conversations happens entirely on the owner's personal or business phone, disconnected from their scheduling, quoting, and operational workflows. This leads to missed messages, forgotten follow-ups, and an inability for OHC's AI assistant to help triage demand, draft replies, or automatically generate tasks. The owner is stuck manually context-switching between WhatsApp and their operational tools.

  ## Research Report
  **Discovered Tool:** Twilio Programmable Messaging (WhatsApp Business API)

  **Market Need Discovery:**
  A review of competitor ecosystems (e.g., Shopify Inbox, HubSpot Marketplace, Zendesk) reveals that WhatsApp integration is consistently among the top-installed and most-requested features, especially for businesses outside the US or those relying on conversational commerce. Small business owners on platforms like Reddit (r/smallbusiness) frequently complain about the inability to track WhatsApp conversations in their CRM or operational software.

  **Deep-Dive Evaluation:**
  - **Capabilities:** Twilio offers a robust API for sending and receiving WhatsApp messages. It supports rich media (images, PDFs - crucial for quotes and design concepts like Maya's cakes), read receipts, and message templates (required for business-initiated conversations outside the 24-hour customer service window).
  - **SaaS Viability:** Twilio's pricing is pay-as-you-go per conversation, making it cost-effective for a multi-tenant SaaS model. OHC can manage a master Twilio account and map subaccounts or specific sender numbers to individual OHC tenants, handling billing internally or passing costs along.
  - **Usability for Owner:** The technical setup (Meta Business Manager verification, Twilio API keys) must be abstracted. The owner should simply go through a guided OAuth/setup flow in OHC to link their WhatsApp Business number. Once connected, messages flow into the OHC Work Triage feed alongside emails and DMs, where the AI assistant can summarize them and draft responses.

  ## Design Doc
  - **Integration Trigger:** Owner connects their WhatsApp Business account via an OHC settings panel.
  - **Incoming Flow:** Twilio webhooks deliver incoming WhatsApp messages to OHC's API layer. OHC routes the message to the correct tenant's `Work Triage` queue.
  - **AI Assistant Action:** The Customer Relationship Assistant reads the incoming message, matches it to existing customer records (using the phone number), and prepares a drafted reply or identifies intent (e.g., "This looks like a cake inquiry. Draft a quote?").
  - **Outgoing Flow:** Owner approves or edits the AI-drafted reply in the OHC mobile or web UI. OHC sends the message back through the Twilio API.
  - **Data Storage:** Chat history is stored in the tenant's localized PostgreSQL database, ensuring context is available for future AI actions and order history.

  ## Implementation Prompt
  Implement a Twilio WhatsApp integration that allows an owner to receive and reply to customer WhatsApp messages directly from the OHC Work Triage feed.

  **Acceptance Criteria:**
  1. Provide a UI for the owner to connect a WhatsApp Business number.
  2. Implement webhook endpoints to receive incoming messages from Twilio, parse them, and display them in the owner's unified task/message feed.
  3. Ensure the Customer Assistant AI can read these messages, retrieve customer history by phone number, and draft contextual replies.
  4. Allow the owner to click "Approve & Send" or edit the drafted reply, which sends the response back to the customer via Twilio WhatsApp API.
  5. The entire experience must be fully functional on a 375px mobile screen, feeling like a unified chat interface rather than a complex API configuration.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
