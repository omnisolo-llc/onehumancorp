issue_title: "Integration: Meta WhatsApp Cloud API for Omnichannel Inbox"
issue_description: |
  Title
  Integration: Meta WhatsApp Cloud API for Omnichannel Inbox

  Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service Owner) receive a massive volume of customer inquiries, orders, and service requests directly through WhatsApp. Currently, they have to constantly switch between their personal/business WhatsApp app and the OHC platform to manage bookings, send quotes, or check order status. This scattered workflow leads to missed leads, delayed responses, and a lack of centralized context for the owner. They need WhatsApp conversations to flow directly into their OHC Work Triage feed so they can rely on the AI assistant to draft replies and manage operations without context switching.

  Research Report
  - Tool Evaluated: Meta WhatsApp Cloud API (directly hosted by Meta).
  - Why not Twilio? While Twilio offers a unified API, going direct to Meta's Cloud API removes the middleman markup on conversation-based pricing, which is critical for the margins of small business owners. Meta provides a robust Graph API for sending and receiving messages.
  - Competitor Landscape: Tools like WeCom, Feishu, and HubSpot all offer native integration with popular chat channels. WhatsApp is the de facto standard for business communication in LATAM, EMEA, and parts of APAC.
  - Capabilities: Supports text, media, interactive messages (lists, reply buttons), and template messages (for marketing/utility out of the 24-hour window). Webhooks provide real-time updates for incoming messages and read receipts.
  - Ease of Use for Owners: The integration process requires a Facebook Business account. The complexity can be hidden behind an embedded signup flow (Facebook Embedded Signup) allowing owners to connect their WhatsApp number in a few clicks directly from OHC.
  - Pricing: First 1,000 service conversations per month are free, which comfortably covers small owner/operators like Maya and Fatima. Paid conversations vary by region but are generally affordable.

  Design Doc
  - Trigger: A customer sends a WhatsApp message to the owner's connected business number.
  - Action: Meta sends a webhook payload to the OHC backend (/api/webhooks/whatsapp). The backend verifies the signature, identifies the OHC tenant_id associated with the WhatsApp Phone Number ID, and routes the message to the Rust WebSocket / SSE real-time messaging engine.
  - User Experience (Owner): The message appears in the OHC Work Triage feed with a WhatsApp badge. The Customer Assistant auto-drafts a reply based on business context (e.g., inventory, availability). The owner can review, edit, and click "Send".
  - Outbound: When the owner replies in OHC, the backend calls the WhatsApp Cloud API (POST /v19.0/{PHONE_NUMBER_ID}/messages) to deliver the message.
  - Data Model: Extending the omnichannel inbox to support provider: "whatsapp" and mapping WhatsApp contact phone numbers to unified customer profiles.

  Implementation Prompt
  - Build a native Rust channel adapter for the Meta WhatsApp Cloud API within the omnisolo-llc/onehumancorp backend.
  - Implement a secure webhook endpoint to receive incoming WhatsApp messages and status updates (delivered, read).
  - Integrate with the OHC real-time engine to push incoming WhatsApp messages to the connected tenant's Work Triage feed via SSE/WebSocket.
  - Implement the outbound message sending using the WhatsApp Graph API, supporting plain text and basic image attachments.
  - Ensure all database interactions respect the multi-tenant SaaS architecture (RLS with tenant_id).
  - Create a simple settings UI where the owner can input their Meta App credentials (or use embedded signup if preferred) to connect their WhatsApp number.
  - Acceptance Criteria: A user can connect their WhatsApp Business account, receive a message from a customer in the OHC Work Triage UI, and reply back to the customer successfully.

  Priority
  P0

  Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
