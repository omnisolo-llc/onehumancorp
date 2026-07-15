issue_title: "Integrate WhatsApp Cloud API for Conversational Work Intake & Customer Triage"
issue_description: |
  ### Title
  Integrate WhatsApp Cloud API for Conversational Work Intake & Customer Triage

  ### Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive a significant portion of their business inquiries, orders, and customer support requests via WhatsApp. Managing these messages manually on a personal or basic business app leads to missed leads, forgotten follow-ups, and overwhelming cognitive load. They need a way to connect WhatsApp directly to OHC so the assistant can automatically triage incoming inquiries, draft replies, summarize context, and convert chats into actionable tasks, bookings, or orders without leaving the OHC interface.

  ### Research Report
  - **Tool Evaluated**: WhatsApp Cloud API (Meta)
  - **Relevance**: WhatsApp is the dominant communication channel for small businesses in LATAM, EMEA, and APAC. Competing tools (e.g., WeCom, DingTalk, HubSpot) offer deep integration with chat platforms, which is a major selling point.
  - **Capabilities**:
    - Receive webhooks for incoming text, media, and location messages.
    - Send text, media, and interactive messages (e.g., buttons, lists) to users.
    - Support for template messages for proactive outreach (e.g., appointment reminders, delivery updates).
  - **Pricing**: Free tier includes 1,000 service conversations per month, which is highly viable for small operators. Beyond that, pricing is per-conversation (varies by region and category). Meta hosts the API (Cloud API), removing the need for local infrastructure.
  - **Ease of Use for Owners**: Meta provides embedded signup flows (Embedded Signup) allowing OHC to let owners connect their WhatsApp Business account with a few clicks without dealing with Facebook Developer Consoles directly.
  - **Cloud & Standalone Viability**: Webhook-based architecture fits perfectly with OHC's multi-tenant Cloud setup. For Standalone, users can configure a custom webhook URL via tools like ngrok or direct ingress.

  ### Design Doc
  - **Trigger/Setup**: The owner navigates to "Channels" in OHC and clicks "Connect WhatsApp". OHC initiates a Meta Embedded Signup OAuth flow. Upon success, OHC registers a webhook for the tenant.
  - **Inbound Flow**: When a customer sends a WhatsApp message, Meta posts a webhook to OHC's API. The OHC API routes it to the tenant's AI Job Queue. The **Work Triage** agent analyzes the message, matches it to an existing customer profile (or creates one), and pushes a prioritized item to the owner's feed.
  - **Outbound Flow**: The **Customer & Relationship Assistant** drafts a reply in OHC. The owner taps "Approve & Send". OHC translates this into a WhatsApp Cloud API `POST /messages` call and logs the interaction in the customer's timeline.
  - **User Experience**: The owner sees a unified chat interface in OHC where WhatsApp messages appear alongside emails and Instagram DMs. Technical details like access tokens and webhook verification are entirely hidden.

  ### Implementation Prompt
  - Build an embedded setup flow that allows a user to link their WhatsApp Business account to OHC securely.
  - Create a webhook endpoint that receives WhatsApp messages, verifies the Meta signature, and drops the payload into the AI Job Queue.
  - Update the owner's feed and Customer Profile view to display inbound WhatsApp messages seamlessly.
  - Provide a UI for the owner to approve AI-drafted replies and send them back to the customer via the WhatsApp API.
  - Acceptance Criteria: A non-technical owner can connect WhatsApp in under 3 minutes. Inbound messages instantly appear in the OHC feed. The owner can send a reply from OHC that arrives on the customer's phone via WhatsApp.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
