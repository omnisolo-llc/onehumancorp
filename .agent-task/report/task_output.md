issue_title: "🔍 Scout: Tool Integration Research - Twilio WhatsApp Business API"
issue_description: |
  **Title**: Integrate Twilio WhatsApp Business API for Unified Customer Inbox

  **Problem Statement**:
  Small business owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart) receive a significant portion of their customer inquiries, orders, and service requests via WhatsApp. Currently, these messages remain siloed on the owner's personal or business phone. This forces the owner to manually copy context between WhatsApp and OHC, disrupting the unified "Work Triage" experience. The AI Assistant cannot draft replies or capture demand automatically if it cannot see the incoming WhatsApp messages.

  **Research Report**:
  - **Ecosystem & Market Need**: WhatsApp is the dominant communication channel for small businesses in LATAM, EMEA, and APAC, and is rapidly growing in North America for customer-to-business chats. Competitors like WeCom and DingTalk heavily integrate with their regional messaging giants.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Ease of Use for Owners**: The owner does not need to understand APIs. In OHC, they will simply go to "Settings > Channels", click "Connect WhatsApp", and follow a Meta/Twilio embedded signup flow to link their number. Once connected, WhatsApp messages appear directly in OHC's feed.
  - **Pricing**: Twilio charges per conversation (business-initiated vs. user-initiated). For low-volume small businesses, user-initiated conversations are relatively inexpensive. It operates well in a multi-tenant Cloud environment where OHC manages the Twilio account and bills the tenant, or Standalone where the user provides their own Twilio credentials.
  - **Capabilities & Limits**: Twilio provides robust webhooks for incoming messages and an API for outgoing replies. Rich media (images, PDFs) is supported, which is critical for Carlos receiving photos of broken equipment or Maya sending cake sketches.

  **Design Doc**:
  - **Integration Trigger**: A tenant goes to the OHC Channels settings and authenticates their WhatsApp Business number.
  - **Webhook Flow**: Twilio sends incoming WhatsApp message webhooks to an OHC webhook endpoint. OHC maps the incoming phone number to an existing or new Customer record for the tenant.
  - **User-Facing Outcome**: The message appears in the OHC Assistant's unified "Work Triage" feed. The AI Customer Assistant reads the message, retrieves past customer context, and prepares a drafted reply or action (e.g., "Drafted a quote for 2 custom cakes"). The owner reviews the draft in the OHC UI and clicks "Send", which triggers the Twilio API to reply on WhatsApp.
  - **Media Handling**: Incoming images are downloaded from Twilio and stored in OHC's file storage (GCS/MinIO), displaying directly in the chat feed.

  **Implementation Prompt**:
  Implement the Twilio WhatsApp channel integration. Create a secure webhook receiver that processes incoming Twilio messages and routes them into the tenant's Work Triage feed. Update the AI Assistant prompt context to include these messages so it can draft replies. Provide a UI component in the unified inbox where the owner can review drafted WhatsApp replies and send them back through the Twilio API. Ensure that failures (e.g., message undeliverable due to 24-hour window expiration) are clearly surfaced to the owner as actionable alerts.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
