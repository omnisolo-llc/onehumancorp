issue_title: "Scout Tool Integration Research - Twilio SMS"
issue_description: |
  # Problem Statement
  Food cart operators and others need immediate, reliable SMS notifications for new pre-orders without relying on app connectivity. Relying entirely on push notifications can fail in noisy environments, so native SMS alerts integrated into OmniSolo's Operations department are needed.

  # Research Report
  - **Tool Name**: Twilio
  - **Relevance**: Industry leader for programmable SMS, global reach, and high deliverability.
  - **Capabilities**: API-driven SMS sending, webhook support for replies.
  - **Limits**: A2P 10DLC compliance adds complexity for businesses in the US, requiring business registration.
  - **Pricing**: Pay-per-message. Will require managing quotas or requiring merchants to buy "SMS Credits" if not absorbed in standard fees.
  - **Viability**: Cloud (multi-tenant) mode can use a centralized Twilio account. Standalone mode allows the user to provide their own API key. Open-source tools like Listmonk or Novu could be alternatives but Twilio handles the raw telecom connections best.
  - **User-First Value Mapping**: The owner just toggles "Send SMS reminders" in their settings, making it invisible to the user.

  # Design Doc
  - User goes to Settings and toggles "Send me SMS for new orders".
  - When an order is paid, OmniSolo dispatches async jobs to send SMS messages via Twilio API.
  - The Operations Agent decides the optimal time to send the reminder based on the order and delivery time.
  - The system will need an onboarding flow to manage 10DLC compliance automatically or notify the user of requirements.

  # Implementation Prompt
  Integrate Twilio SMS to allow the platform to send order confirmations, pickup notifications, and appointment reminders via text message. Include a settings panel for merchants to toggle these notifications on or off. Ensure phone number formatting is handled correctly globally (E.164).
  - **Acceptance Criteria**: Customer receives an SMS when their order is marked "Ready for Pickup". Customer receives a reminder SMS before a booked appointment.
  - **Implementation Constraints**: Do NOT prescribe specific implementation approaches, SQL DDL, API endpoint lists, or function signatures.

  # Priority
  P2

  # Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
