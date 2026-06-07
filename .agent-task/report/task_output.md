issue_title: "Twilio WhatsApp Business API Integration for OHC"
issue_description: |
  ## Title
  Twilio WhatsApp Business API Integration for OHC Omnichannel Communications

  ## Problem Statement
  Small business owners and operators (like Carlos the handyman or Maya the baker) rely heavily on WhatsApp to communicate with customers, send quotes, and manage bookings. However, managing these conversations on a personal phone or a disconnected WhatsApp Business app creates silos. The owner loses track of who requested what, has to manually copy-paste appointment details, and cannot easily delegate conversations to staff or an AI assistant. They need their WhatsApp communications fully integrated into the OHC unified feed, allowing OHC's Customer Success Agent to draft replies and trigger operational workflows directly from WhatsApp messages.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Market Context:** WhatsApp is the dominant messaging platform in many regions (LATAM, Europe, Asia) and increasingly used for business in North America. Customers expect quick, conversational interactions.
  - **Competitors:** Tools like Zendesk, Intercom, and specialized WhatsApp CRMs (like Wati or Sirena) offer WhatsApp integrations but are either too expensive, too complex, or focus purely on support rather than tying into core operations (like quoting and booking).
  - **Twilio WhatsApp Business API:** Provides robust, scalable programmatic access to WhatsApp. It supports sending and receiving messages, rich media, and templates. It is reliable and widely used, making it an ideal infrastructure provider for OHC.
  - **Ease of Use for Owners:** Integrating via Twilio removes the technical burden from the owner. OHC can handle the API complexities, presenting a simple "Connect WhatsApp" button to the user.
  - **Pricing:** Twilio charges per conversation (session), which is generally affordable, and OHC could wrap these costs into its subscription tiers or pass them through transparently.
  - **Capabilities:** Webhook events for incoming messages, read receipts, and status updates. Support for sending text, images, location, and pre-approved template messages (essential for initiating conversations or sending notifications outside the 24-hour window).

  ## Design Doc
  ### Architecture & Integration Strategy
  - **Webhook Listener:** Implement a dedicated webhook endpoint in the OHC Backend to receive incoming messages from Twilio.
  - **Message Normalization:** Incoming WhatsApp messages via Twilio are normalized into an internal `Message` format and routed to the Omnichannel Gateway.
  - **Customer Identity Resolution:** Map the incoming WhatsApp phone number to existing customer records in the OHC database.
  - **Unified Feed Integration:** Display WhatsApp messages in the owner's 375px mobile feed, styled to indicate the channel but functionally identical to Instagram or Email inquiries.
  - **Agent Handoff:** Trigger the Customer Success Agent (The Ambassador) to read the message context, lookup customer history, and draft a proposed reply.
  - **Outbound Dispatcher:** Send approved replies back through the Twilio API to the customer's WhatsApp. Support sending automated notifications (e.g., appointment reminders) using pre-approved Twilio WhatsApp templates.

  ### User Experience (Non-Technical Owner Lens)
  - The owner connects their WhatsApp Business account in OHC settings.
  - A customer messages the business on WhatsApp asking for a quote.
  - The owner sees a new item in their OHC feed: "WhatsApp Message from [Customer Name] requesting quote."
  - OHC presents a pre-drafted response (created by the AI agent based on the request and product catalog).
  - The owner taps "Approve and Send", and the message is instantly delivered to the customer's WhatsApp.

  ## Implementation Prompt
  **User-Facing Outcome:**
  As a business owner, I can connect my WhatsApp Business account to OHC. When a customer sends me a WhatsApp message, it appears in my OHC mobile feed. My AI assistant drafts a contextual reply based on my business data, which I can review and send with one tap, without ever needing to open a separate WhatsApp app or manually copy customer information.

  **Acceptance Criteria:**
  1. Create a webhook endpoint to receive Twilio WhatsApp messages and store them in the database, linked to a specific tenant.
  2. Implement an outbound service to send text messages back to the customer via the Twilio API.
  3. Integrate with the existing AI agent framework to automatically draft replies for incoming WhatsApp messages.
  4. Display the incoming message and drafted reply in the mobile-first UI feed.
  5. The owner can edit or approve the draft, triggering the outbound send.
  6. E2E tests using Playwright must demonstrate the full flow: receiving a mocked Twilio webhook, displaying the message in the UI, and simulating the owner approving a draft response.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
