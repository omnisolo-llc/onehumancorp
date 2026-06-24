issue_title: "Implement Twilio WhatsApp Integration for Work Intake & Triage"
issue_description: |
  **Title**: Implement Twilio WhatsApp Integration for Work Intake & Triage

  **Problem Statement**:
  Many owners (like Maya the Home Baker and Carlos the Field Service Owner) run their primary customer intake through WhatsApp DMs. Constantly monitoring a phone, copying order details manually, and responding to inquiries breaks focus from actual work. There is a critical gap between where customers demand service (WhatsApp) and where the owner manages operations (OHC).

  **Research Report**:
  The Twilio WhatsApp Business API is an industry-standard, robust integration that allows platforms to programmatically send and receive WhatsApp messages. It supports rich media (images for cake orders, or photos of broken pipes for service estimates). It uses standard HTTP webhooks for incoming messages, which works well in both our multi-tenant Cloud setup and local Standalone environments (via tunneling/public webhooks if needed). The pricing is affordable for small businesses, and it does not require non-technical owners to manage complex infrastructure—they simply link their WhatsApp Business account.

  **Design Doc**:
  The integration will establish an inbound webhook listener in OHC for Twilio. Incoming WhatsApp messages will trigger the Work Triage capability, placing the message directly into the owner's unified feed. The OHC Customer Assistant will automatically maintain context and draft replies. The Work Triage capability will identify whether the message is a new service request, booking, or general inquiry. Owners can tap to approve drafted replies or type their own, which OHC will dispatch back out through Twilio.

  **Implementation Prompt**:
  Implement the Twilio WhatsApp API integration. Expose a webhook endpoint to receive incoming WhatsApp messages from Twilio. Route these inbound messages into the owner's Work Triage feed as actionable items. Integrate with the existing AI capabilities so the Customer Assistant can draft replies to these WhatsApp messages. Build the outbound path allowing the owner to send messages back to the customer's WhatsApp via Twilio directly from the OHC UI. The setup should require minimal technical configuration from the owner.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []