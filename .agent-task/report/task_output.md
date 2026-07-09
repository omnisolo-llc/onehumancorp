issue_title: "Scout: WhatsApp Cloud API Integration for Conversational Commerce"
issue_description: |
  ## Problem Statement
  Small-business owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart) receive a significant portion of their customer inquiries and orders through WhatsApp. Currently, these messages remain siloed on personal or standalone business devices. This creates a fragmented workflow where owners must manually switch between WhatsApp and their operational tools to generate quotes, confirm bookings, or track deposits. This friction leads to slow response times, lost leads, and operational blind spots.

  ## Research Report
  ### Market Need
  WhatsApp is the primary communication channel in LATAM, EMEA, and parts of APAC, and is growing rapidly for B2C communication in North America. Competitors like WeCom and Shopify Inbox heavily leverage messaging platform integrations to centralize customer context.

  ### Tool Evaluation: WhatsApp Cloud API (Meta)
  - **Ease of Use for Non-Technical Users**: The owner simply links their Facebook Business account and WhatsApp number during onboarding. After setup, OHC transparently handles all message routing. The owner uses OHC as the unified inbox, without needing to understand the underlying API.
  - **Capabilities & Limits**:
    - Rich media support (images, documents, interactive buttons).
    - Webhooks for real-time message delivery and read receipts.
    - Template messages for proactive outreach (e.g., booking confirmations, deposit reminders) which require Meta approval but offer high engagement.
    - 24-hour customer service window for free-form replies.
  - **SaaS Viability & Pricing**: Meta offers a generous free tier (1,000 service conversations per month). Beyond that, pricing is conversation-based, which aligns with usage-based billing models. It operates perfectly in a Cloud (multi-tenant) environment, requiring tenant-specific access tokens.

  ## Design Doc
  - **Triggers**:
    - *Inbound*: Webhooks from Meta receive new WhatsApp messages, media, and delivery statuses.
    - *Outbound*: Agents or the owner trigger replies or proactive templates from the OHC interface.
  - **System Behavior**:
    - The integration maps WhatsApp phone numbers to OHC Customer profiles, maintaining conversation history.
    - The "Work Triage" agent evaluates inbound messages to generate tasks, draft replies, or extract order details.
    - The "Customer & Relationship Assistant" suggests responses and maintains the 24-hour reply window status.
  - **User Interface**:
    - A unified inbox view where WhatsApp messages appear alongside DMs and emails.
    - Clear visual indicators for the 24-hour free-form reply window.
    - Integrated action buttons (e.g., "Send Payment Link", "Book Appointment") within the chat interface that map to WhatsApp interactive messages.

  ## Implementation Prompt
  Implement the WhatsApp Cloud API integration to allow owners to manage WhatsApp conversations directly within OHC. The integration should support:
  1. **OAuth/Business Onboarding**: A straightforward flow for owners to link their WhatsApp Business number.
  2. **Inbound Messaging**: Reliable webhook processing to receive text, images, and audio messages, routing them to the correct owner's workspace and creating/updating customer profiles.
  3. **Outbound Messaging**: The ability for the owner (or authorized agents) to send text and media replies within the 24-hour window.
  4. **Agent Integration**: Expose WhatsApp messaging as a tool for the Work Triage and Customer Relationship agents to read context and draft replies.
  The final outcome should be a unified messaging experience where the owner cannot tell the technical difference between replying to a WhatsApp message and a native app notification.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
