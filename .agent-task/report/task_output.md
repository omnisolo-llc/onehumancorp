issue_title: "Integrate Twilio for WhatsApp Business to unify customer messaging"
issue_description: |
  ## Title
  Integrate Twilio for WhatsApp Business to unify customer messaging

  ## Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service Owner) receive a significant portion of their customer inquiries, orders, and service requests via WhatsApp. Currently, managing these conversations requires juggling a personal or separate business phone, leading to missed leads, scattered context, and an inability for OHC's AI assistant to draft replies or triage work automatically. Owners need these WhatsApp interactions centralized in their OHC work feed to maintain momentum without constantly switching apps.

  ## Research Report
  - **Tool Evaluated:** Twilio API for WhatsApp
  - **Relevance:** WhatsApp is the dominant messaging platform in many global markets (e.g., LATAM, Europe, India) and increasingly for local SMBs in the US. Twilio provides a robust, scalable API to send and receive WhatsApp messages, manage templates, and handle opt-ins.
  - **Owner/Operator Benefit:** Non-technical users won't know it's Twilio. They will simply connect their WhatsApp Business number to OHC. All incoming messages will appear in the OHC Work Triage feed. The Customer & Relationship Assistant can then automatically draft replies, associate messages with existing customer profiles, and track order/service context.
  - **SaaS Viability:** Twilio offers pay-as-you-go pricing which is highly viable for a multi-tenant SaaS. It supports webhooks for real-time incoming messages, making it easy to sync state to OHC. It can be configured per tenant (using distinct senders or subaccounts).
  - **Ease of Use:** The complexity of the WhatsApp Business API (approvals, templates, 24-hour session windows) can be abstracted away from the owner. OHC handles the routing; the owner just reads and taps "Send Draft".

  ## Design Doc
  - **Trigger:** A customer sends a WhatsApp message to the owner's connected business number.
  - **Action:** Twilio triggers a webhook to OHC's backend. OHC associates the message with the correct tenant and customer profile. The Work Triage system creates a prioritized feed item. The Customer Assistant agent generates a suggested reply based on context (e.g., pending cake order for Maya, or a repair quote for Carlos).
  - **User Visibility:** The owner opens the OHC app, sees the unread WhatsApp message in their feed with an AI-drafted reply. They can edit or approve the reply. Once approved, OHC sends the reply back through the Twilio API.
  - **Handling Constraints:** The design must account for WhatsApp's 24-hour customer service window, nudging the owner to reply promptly or using pre-approved templates if the window expires.

  ## Implementation Prompt
  - **Outcome:** The owner can read and reply to WhatsApp messages directly within the OHC assistant interface. Replies drafted by the AI can be sent to the customer's WhatsApp seamlessly.
  - **Acceptance Criteria:**
    - An owner can connect a WhatsApp Business number in their OHC settings.
    - Incoming WhatsApp messages appear in the OHC Work Triage feed in near real-time.
    - The AI assistant successfully drafts replies to these messages based on customer context.
    - The owner can send a reply from OHC, which is delivered to the customer's WhatsApp.
    - The system gracefully handles the 24-hour session window, providing clear UI feedback if a standard reply cannot be sent.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
