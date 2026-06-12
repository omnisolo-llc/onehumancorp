issue_title: "Integrate Twilio for WhatsApp Business API for Unified Work Intake"
issue_description: |
  **Problem Statement**:
  Non-technical business owners—like Maya (Home Baker), Carlos (Field Service Owner), and Fatima (Food Cart Operator)—receive a massive volume of customer inquiries, orders, and service requests via WhatsApp. Currently, checking WhatsApp separately from their main operations dashboard causes missed leads, delayed responses, and a disjointed workflow. They need a way to triage messages, draft replies, and turn WhatsApp DMs directly into tasks or bookings without ever leaving their central work assistant.

  **Research Report**:
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Market Context**: WhatsApp is the dominant communication channel for small businesses globally, especially in LATAM, EMEA, and APAC. Competitors often lack native, deeply integrated WhatsApp support that ties directly into operational tasks (e.g., turning a chat into a custom-order deposit or service route note).
  - **Usability for Non-Technical Users**:
    - *Pros*: Once authenticated and connected, the owner never has to deal with Meta's complex Business Manager again. All inbound messages appear natively within the OHC unified feed. The OHC AI Assistant can read the messages and suggest contextual replies.
    - *Cons*: The initial onboarding (Twilio sign-up and Meta approval process) can be daunting. OHC must provide a guided, seamless onboarding flow to abstract this complexity.
  - **Pricing & SaaS Viability**:
    - Twilio charges a flat $0.005 per message fee (inbound or outbound).
    - Meta's per-template message fees (Utility, Authentication, Marketing) are passed through, but crucially, during a 24-hour "customer service window" initiated by a user message, free-form messages have no Meta fee (only the Twilio fee applies).
    - This pricing is highly viable for a multi-tenant cloud offering where OHC manages the API keys, or as a standalone integration where the user inputs their own Twilio credentials.

  **Design Doc**:
  - **Trigger**: User selects "Connect WhatsApp" in the OHC integrations menu.
  - **Actions**:
    - A guided setup wizard helps the user authenticate their Twilio account and connect their WhatsApp number.
    - Webhooks are established to route inbound WhatsApp messages directly into the OHC AI Job Queue.
    - The OHC "Work Triage" capability processes the message, pulling customer context and suggesting next steps (e.g., draft reply, create booking, generate quote).
  - **User Visibility**: Inbound messages appear in the owner's primary feed. The AI drafts a response, which the owner can approve with one tap.

  **Implementation Prompt**:
  - **User-Facing Outcome**: Enable owners to manage all their WhatsApp customer communications directly within OHC. When a customer messages their WhatsApp business number, the message must appear in the OHC feed, and the owner can reply directly or let the OHC assistant draft a response.
  - **Acceptance Criteria**:
    1. A new integration card for "WhatsApp via Twilio" is available in the UI.
    2. The integration successfully authenticates and registers the necessary webhooks for receiving messages.
    3. Inbound WhatsApp messages are routed to the OHC unified inbox/feed.
    4. Outbound replies sent from OHC are successfully delivered to the customer via WhatsApp.
    5. The integration supports parsing standard text messages and images.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
