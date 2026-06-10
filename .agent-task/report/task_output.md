issue_title: "Implement WhatsApp Business API for Customer Operations"
issue_description: |
  **Problem Statement:** Maya, Carlos, and Fatima rely heavily on WhatsApp to coordinate with customers (custom cake orders, service route updates, and pre-order pickups). Constantly switching between a personal WhatsApp app and a business management tool creates lost leads and disorganized schedules. They need a way for their assistant to handle WhatsApp messages directly.

  **Research Report:**
  - WhatsApp is the primary communication channel for small businesses globally, especially in LATAM and Southeast Asia, but also growing rapidly for local services in the US.
  - The WhatsApp Cloud API allows sending and receiving messages.
  - Alternatives like Twilio are powerful but often require more developer-centric setup and are more expensive for basic use-cases compared to direct Meta Cloud API integration, though Twilio provides easier onboarding for SMS fallback.
  - We should target Twilio initially for ease of integration into an app ecosystem like OHC. Twilio’s API abstracts away much of the complexity of the WhatsApp Business API and allows easy expansion into SMS if needed. Pricing is accessible for small businesses (pay-as-you-go).

  **Design Doc:**
  - OHC Assistant will expose a "Connect WhatsApp" integration option.
  - Once connected via Twilio API keys, incoming WhatsApp messages will be routed to the OHC Work Triage feed.
  - The Customer Assistant agent will draft replies to these messages, appearing in the OHC UI as pending drafts for the owner to approve, edit, or send.
  - Operations/Sales Assistant can trigger outbound WhatsApp notifications (e.g., "Your cake is ready for pickup!", "Carlos is on his way to your address").

  **Implementation Prompt:**
  - Implement a Twilio WhatsApp integration module.
  - The integration must allow an owner to input their Twilio credentials (Account SID, Auth Token, WhatsApp Phone Number).
  - Create a webhook endpoint in OHC to receive incoming messages from Twilio.
  - Route incoming messages into the unified `Work Triage` feed.
  - Enable the OHC AI assistant to draft replies and allow the owner to send them via the Twilio API.
  - Ensure all interactions are visible in a simplified, mobile-friendly 375px view.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
