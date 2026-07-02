issue_title: "🔍 Scout: Tool Integration Research - Twilio WhatsApp Business API"
issue_description: |
  ### Mission Queue Protocol: Tool Integration Brief

  **Title:** Integrate Twilio WhatsApp Business API for Unified Messaging

  **Problem Statement:**
  Small business owners and operators (Maya, Carlos, Fatima) conduct a significant portion of their customer communication through WhatsApp. Currently, managing WhatsApp separately from their main work assistant leads to fragmented context, missed leads, and slow response times. Owners need WhatsApp inquiries to flow directly into their OHC Work Triage feed so the assistant can draft replies, capture context, and turn conversations into tasks or orders seamlessly.

  **Research Report:**
  - **Tool Evaluated:** Twilio API for WhatsApp
  - **Capabilities:** Supports rich media, templated outbound messages (for reminders), and conversational inbound messaging. Reliable webhook delivery for real-time messaging.
  - **SaaS Viability & Pricing:** Twilio uses conversation-based pricing. Service conversations (user-initiated) are very cost-effective, and the first 1,000 per month are free in many regions. This fits perfectly with OHC's model of providing a high-value tool without huge per-tenant overhead. It supports both Cloud (multi-tenant webhook routing) and Standalone (local config) environments.
  - **Owner/Operator Usability:** Non-technical owners never see Twilio API keys. OHC will use the Embedded Signup flow so users simply log in with their Facebook account to connect their WhatsApp Business number to OHC.

  **Design Doc:**
  - **Trigger:** An inbound WhatsApp message triggers a Twilio webhook sent to OHC.
  - **Processing:** OHC identifies the tenant by the receiving phone number, maps the customer by their sender phone number, and creates or updates a conversation thread.
  - **Action (Inbound):** The new message appears in the owner's Work Triage feed. The Customer Assistant agent analyzes the message and drafts a contextual reply.
  - **Action (Outbound):** When the owner taps "Send" on a drafted reply, or when an automated operational task (e.g., Carlos's appointment reminder) runs, OHC calls the Twilio API to send the message.
  - **User Experience:** The owner sees a unified inbox in OHC. They don't switch apps to reply to WhatsApp leads.

  **Implementation Prompt:**
  - Provide a simplified settings interface for the owner to link their WhatsApp Business account.
  - Implement the inbound webhook handler to receive WhatsApp messages and place them into the Work Triage unified feed.
  - Provide context to the Customer Assistant agent so it can generate draft replies for WhatsApp threads.
  - Implement the outbound message flow where an owner's approval of a draft sends the message back to the customer via WhatsApp.
  - Acceptance Criteria: A test user can send a WhatsApp message to the connected number, see it in OHC's triage feed, generate an AI-drafted reply, and send it back to their phone successfully.

  **Priority:** P0

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
