issue_title: "Integrate Twilio for WhatsApp Business API"
issue_description: |
  ### Title
  Integrate Twilio for WhatsApp Business API

  ### Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) run their businesses heavily on messaging platforms, particularly WhatsApp, which is the primary communication channel in many markets globally. Currently, they have to constantly switch between their WhatsApp app and OHC, leading to missed messages, scattered customer context, and delayed responses. They need OHC's Assistant to centralize WhatsApp inquiries, draft replies, and link conversations to bookings and orders without requiring them to leave the OHC command center.

  ### Research Report
  - **Ecosystem & Market Need**: WhatsApp has over 2 billion active users. For small businesses, WhatsApp Business is often their main storefront and customer support channel. Competitors like WeCom and Shopify (via Inbox) offer deep integrations with messaging platforms to unify communications.
  - **Tool Evaluation (Twilio API for WhatsApp)**:
    - **Usability for Non-Technical Owners**: Owners will not interact with Twilio directly. They will authenticate or link their existing WhatsApp Business number via an OAuth or guided setup within OHC. From then on, messages appear in the OHC feed like any other work task.
    - **Developer Docs & API Quality**: Twilio provides robust, stable APIs, excellent documentation, and reliable webhooks for incoming messages and delivery receipts.
    - **Pricing & Viability**: Twilio charges per conversation (user-initiated vs. business-initiated). It has a clear pay-as-you-go model suitable for multi-tenant cloud environments. The free tier for testing is generous.
    - **Cloud vs. Standalone**: Primarily built for Cloud (multi-tenant), where OHC handles webhooks. For standalone (local), ngrok or similar tunneling would be required for webhooks, or local polling if supported (though webhooks are standard).

  ### Design Doc
  - **Trigger / Entry Point**:
    - Incoming WhatsApp messages trigger a webhook from Twilio to OHC's API layer.
    - OHC creates a new conversation or appends to an existing customer record.
  - **Actions & Work Triage**:
    - The message is placed in the "Work Intake" feed.
    - The AI Assistant reads the message, fetches customer context, and drafts a proposed reply or action (e.g., "Prepare quote for 2-tier cake").
  - **User Visible Output**:
    - The owner sees the message in their central feed on mobile (375px) or desktop.
    - They can tap "Approve & Send" on the AI's drafted reply, or edit the reply directly.
    - Outgoing messages are sent back via the Twilio API to the customer's WhatsApp.
  - **Storage & Locking**:
    - Conversation states stored in PostgreSQL.
    - Redis locks (`ohc:lock:{tenant_id}:whatsapp:{phone_number}`) prevent concurrent agent actions on the same message thread.

  ### Implementation Prompt
  - Create a seamless setup flow in OHC Settings where an owner can connect their WhatsApp Business number.
  - Establish a webhook endpoint to receive incoming WhatsApp messages and securely route them to the correct tenant.
  - Integrate these messages into the main Work Triage feed.
  - Enable the Customer Assistant to automatically draft replies for WhatsApp messages, keeping character limits and WhatsApp policy rules in mind.
  - Ensure the chat interface is fully responsive, looking native on a 375px mobile screen.
  - **Acceptance Criteria**:
    1. Owner can receive a test WhatsApp message in their OHC feed.
    2. Assistant generates a draft reply.
    3. Owner can review, edit, and send the reply via OHC, and it is successfully delivered to the customer's WhatsApp app.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
