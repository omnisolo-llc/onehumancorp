issue_title: "Integrate WhatsApp Business API via Twilio for Unified Owner Inbox"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement:**
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) receive a massive volume of customer inquiries, order modifications, and scheduling questions through WhatsApp. Currently, this data lives in siloed personal or business WhatsApp apps on their phones. It doesn't sync with OHC's Work Triage, Customer Assistant, or Scheduling Assistant. This forces the owner to manually copy-paste context, leading to dropped leads, missed context during service delivery, and an inability for OHC agents to draft context-aware replies or automate deposit links.

  **Research Report (Twilio WhatsApp Business API):**
  - **Market Position:** WhatsApp is the dominant messaging platform in LATAM, EMEA, and APAC, and rapidly growing for B2C in North America. Twilio provides the most robust, developer-friendly WhatsApp Business API wrapper.
  - **Usability for Non-Technical Owners:** The owner does not need to know what Twilio or an API is. Through OHC, they will simply click "Connect WhatsApp" and go through the embedded signup flow (or Twilio's guided onboarding). Once connected, messages seamlessly appear in their OHC Work Triage feed.
  - **Pricing & Viability:** Twilio charges per conversation (user-initiated or business-initiated). It has a strong free tier/trial and is highly viable for a multi-tenant SaaS architecture. It uses reliable webhooks for incoming messages and REST APIs for sending.
  - **Capabilities:** Supports rich media (images for cake references, PDFs for invoices), template messages (for automated booking confirmations), and interactive buttons (for quick replies).

  **Design Doc:**
  - **Trigger:** An incoming WhatsApp message hits a Twilio Webhook, which OHC ingests.
  - **Action:** The system identifies the customer by phone number, pulls their context, and adds the message to the "Work Triage" feed. The Customer Assistant drafts a reply based on previous order history or booking availability.
  - **User Experience:** The owner sees the WhatsApp message exactly like an email or web form submission in OHC. They can tap "Approve Draft" to send a response back via WhatsApp instantly. All advanced setup (webhook config, API keys) is hidden.

  **Implementation Prompt:**
  Implement the Twilio WhatsApp API integration so that incoming WhatsApp messages create actionable items in the OHC Work Triage feed. Ensure the Customer Assistant can read these messages and generate draft replies. The integration must support capturing the customer's phone number and matching it to existing records. Provide a simple UI for the owner to link their WhatsApp account without seeing technical API details.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
