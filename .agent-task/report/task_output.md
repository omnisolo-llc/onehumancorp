issue_title: "Integrate Twilio WhatsApp Business API for Unified Customer Messaging"
issue_description: |
  **Title**: Integrate Twilio WhatsApp Business API for Unified Customer Messaging

  **Problem Statement**:
  For owners like Maya (Home Baker) and Carlos (Field Service), a significant portion of customer communication, from initial inquiries to appointment reminders and custom order details, happens over WhatsApp. Currently, these owners have to switch back and forth between the OHC app and their personal or business WhatsApp accounts. This leads to dropped leads, forgotten follow-ups, and fragmented customer histories. They need a way for their OHC assistant to see WhatsApp messages, draft replies, and send automated notifications (like order ready or technician en route) directly to customers' WhatsApp numbers without leaving the OHC command center.

  **Research Report**:
  - **Market Need**: WhatsApp is the dominant messaging platform in many global markets (LATAM, Europe, India) and is increasingly used by small businesses in the US for customer support and sales. Competitors like HubSpot, Zoho, and specialized local tools deeply integrate WhatsApp.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Ease of Use for Non-Technical Users**: As a backend integration, Twilio is invisible to the user. The owner simply clicks a "Connect WhatsApp" button in OHC, follows a Meta OAuth flow, and their WhatsApp Business number is linked. From then on, it acts as a seamless channel in OHC's unified inbox.
  - **Pricing**: Twilio charges per conversation (business-initiated vs. user-initiated), which is standard. There are no heavy fixed monthly fees, making it highly accessible for small-scale operators. OHC could either pass these costs through or bundle them into a premium tier.
  - **Reputation and Reliability**: Twilio is the industry standard for cloud communications. Their API, webhooks, and documentation are best-in-class. They support both multi-tenant SaaS environments (using subaccounts) and standalone deployments.

  **Design Doc**:
  - **Trigger/Setup**: The owner navigates to a "Channels" or "Integrations" screen in OHC and selects "Connect WhatsApp". They are guided through an embedded setup to link their WhatsApp Business Profile.
  - **Action**: Inbound WhatsApp messages trigger webhooks to OHC, which route the messages to the Work Triage feed. The Customer & Relationship Assistant reads the message, retrieves the customer's history, and drafts a reply.
  - **User Experience**: The owner sees the message in their unified feed alongside Instagram DMs and emails. They can approve or edit the AI-drafted reply. Operations Assistant can also trigger outbound template messages (e.g., "Your cake is ready for pickup, Maya!").

  **Implementation Prompt**:
  Build a WhatsApp messaging integration via Twilio that seamlessly routes inbound customer messages to the OHC Work Triage feed and allows the owner to reply directly from the OHC interface.
  - **Acceptance Criteria**:
    - An owner can link a WhatsApp Business number to their OHC workspace.
    - Inbound WhatsApp messages appear in the OHC feed within seconds.
    - The AI Customer Assistant can draft replies to these WhatsApp messages.
    - The owner can send replies from OHC, which are delivered to the customer's WhatsApp.
    - Automated operational alerts (e.g., booking confirmations) can be routed via WhatsApp if the customer prefers.
    - The integration handles media attachments (images of products/issues) seamlessly.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
