issue_title: "Integrate Twilio WhatsApp Business API for Centralized Customer Communication"
issue_description: |
  ## **Problem Statement**
  Small business owners like **Maya (Home Baker)** and **Carlos (Field Service Owner)** are struggling to manage customer inquiries across multiple platforms (Instagram DMs, WhatsApp, SMS, Emails). They are losing potential leads and wasting time switching contexts.
  - **Maya** receives custom cake orders via Instagram DMs and WhatsApp but frequently misses follow-ups or forgets to request deposits because she has no centralized view.
  - **Carlos** operates primarily from his Android phone while on jobs. He needs service requests and customer follow-ups on a widely used platform (WhatsApp) but routed through an assistant so he doesn't have to text back manually while working.

  Owners need a unified inbox where the AI assistant can see WhatsApp inquiries, draft replies, send booking links, and request payments directly within the conversation, without the owner needing technical setup.

  ## **Research Report: Twilio WhatsApp Business API**
  **Findings:**
  - **Usability for Non-Technical Users:** Twilio provides a robust Programmable Messaging API that allows ISVs (Independent Software Vendors) like OHC to register WhatsApp senders on behalf of customers through the "Tech Provider Program". This means we can abstract the complex Meta business verification away from Maya and Carlos, giving them a simple "Connect WhatsApp" button.
  - **Capabilities:** Supports rich media (images, PDFs for invoices), template messages (for marketing and utility, e.g., deposit reminders), and conversational messaging (free-form replies within a 24-hour customer service window).
  - **Pricing Model:** Highly viable for SaaS. Twilio charges a flat $0.005 per message fee, plus Meta's passthrough fee based on conversation category (Utility, Authentication, Marketing, Service). Utility messages (e.g., appointment reminders) and Service messages (responding to customer inquiries within 24hrs) are very affordable or free of Meta fees in many regions.
  - **Integration Reliability:** Excellent developer docs, reliable webhooks, and seamless fallback to SMS if a WhatsApp message fails. It operates well in Cloud (multi-tenant) environments.

  ## **Design Doc**
  **Trigger:**
  - Owner navigates to "Channels" in OHC settings and clicks "Connect WhatsApp Number".
  - Customer messages the connected WhatsApp number.

  **Action:**
  - OHC provisions or links a WhatsApp Business number via the Twilio Tech Provider API in the background.
  - Incoming WhatsApp messages hit an OHC webhook and are routed to the **Work Triage** feed.
  - The **Customer & Relationship Assistant** reads the message context, pairs it with the customer's profile, and drafts a reply (e.g., a quote or scheduling link).
  - The owner sees the drafted reply in their unified OHC feed on their phone and clicks "Approve & Send".
  - OHC sends the message back out via Twilio API.

  **User Interface:**
  - A simple OAuth-style connection flow to link a WhatsApp number.
  - WhatsApp messages appear inline in the daily triage feed with a WhatsApp icon.
  - AI-drafted replies show up just like email/SMS drafts, abstracting the underlying protocol.

  ## **Implementation Prompt**
  **Outcome:** Provide the owner with a single button to connect their WhatsApp business number. Once connected, all incoming WhatsApp messages must appear in the OHC triage feed. The AI assistant must automatically draft context-aware replies to these WhatsApp messages for the owner to approve and send with one tap.

  **Acceptance Criteria:**
  1. A "Connect WhatsApp" UI flow exists that allows an owner to link their number without dealing with Meta API keys directly.
  2. Incoming WhatsApp messages create actionable items in the owner's triage feed.
  3. The OHC assistant successfully drafts replies to WhatsApp messages.
  4. The owner can tap "Approve" to send the reply back to the customer's WhatsApp seamlessly.
  5. The integration must handle media (e.g., customer sending a photo of a cake design or a broken pipe).

  ## **Priority**
  P1

  ## **Estimated Scope**
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []