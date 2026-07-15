issue_title: "Integrate Twilio WhatsApp Business API for Customer & Relationship Assistant"
issue_description: |
  **Title**: Integrate Twilio WhatsApp Business API for Customer Communications

  **Problem Statement**:
  Small business owners like Maya (the baker) and Carlos (the field service owner) rely heavily on WhatsApp to communicate with customers, take orders, and answer service requests. Currently, managing these conversations requires constantly checking a personal or separate business phone, leading to missed messages, forgotten follow-ups, and fragmented customer context. They need OHC's Customer & Relationship Assistant to seamlessly read from and reply to WhatsApp messages directly in their unified Work Triage feed.

  **Research Report**:
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Market Context**: WhatsApp is the dominant messaging platform in many regions (like LATAM, Europe, and India) and increasingly popular in the US for B2C communication. Competitors like WeCom and DingTalk heavily integrate with local messaging networks.
  - **Pricing & Economics**: Twilio offers a pay-as-you-go model starting at $0.005 per message, plus Meta's WhatsApp Connectivity fee (which varies by conversation type: marketing, utility, service). This is extremely viable for multi-tenant SaaS as well as standalone operators, as costs directly map to usage.
  - **Usability for Non-Technical Owners**: Owners will not need to understand "APIs". They simply authenticate their business phone number through an embedded Meta signup flow or Twilio console connection. Once connected, it just works—messages appear in OHC and replies are sent to the customer's WhatsApp.
  - **Reliability & SLAs**: Twilio provides robust enterprise-grade reliability, webhooks with signature verification (critical for security), and automatic retry mechanisms for failed message deliveries.

  **Design Doc**:
  - **Integration Trigger**: The integration is enabled in the owner's settings under "Communication Channels". When an owner connects their WhatsApp Business account, Twilio webhooks will start pushing inbound messages to OHC.
  - **User Experience**: Incoming WhatsApp messages appear in the OHC "Work Triage" feed just like web forms or emails. The AI Customer Assistant can draft suggested replies based on the customer's history. When the owner clicks "Send", the message goes back through Twilio to the customer's WhatsApp.
  - **Background Agent Actions**: The system will sync customer contact info, link conversations to existing customer profiles, and keep track of active conversation windows (Meta requires replies within 24 hours for service conversations).

  **Implementation Prompt**:
  - **User-Facing Outcome**: The owner can connect their WhatsApp Business number to OHC. Once connected, incoming WhatsApp messages from customers appear in their Work Triage feed. The AI drafts a relevant reply, and the owner can approve and send the message back to the customer's WhatsApp directly from OHC.
  - **Acceptance Criteria**:
    - A simple "Connect WhatsApp" setting is available for the owner.
    - Inbound messages from WhatsApp correctly appear in the Work Triage UI.
    - AI-drafted responses or manual owner responses are successfully sent back to the customer via WhatsApp.
    - If a response fails (e.g., outside the 24-hour window without a template), the UI truthfully shows a failed state and suggests an actionable workaround.
    - The integration works beautifully on the 375px mobile breakpoint, with clear sender identification (e.g., "WhatsApp").

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
