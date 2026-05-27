issue_title: "Integrate Twilio WhatsApp Business API for Customer Notifications"
issue_description: |
  # Integrate Twilio WhatsApp Business API for Customer Notifications

  ## Problem Statement
  Small business owners—like Carlos, who runs a busy local delivery service, or Priya, managing a fast-paced salon—struggle to keep customers informed using traditional email. Emails often get lost in spam folders, leading to missed appointments, confused customers, and increased support calls. They need a way to reach their customers directly where they already are: on their phones, via WhatsApp.

  ## Research Report
  *   **Market Need:** WhatsApp has over 2 billion active users globally. In many regions (LATAM, India, Europe) and increasingly in the US, it is the primary mode of communication. Small business owners frequently cite direct messaging integrations as a top priority for reducing no-shows and improving customer satisfaction.
  *   **Tool Evaluated:** Twilio API for WhatsApp.
  *   **Ease of Use (Non-technical):** Twilio itself is a developer platform, but by building an integration into OHC, we can abstract away the complexity. The business owner will simply connect their Meta/WhatsApp Business account via an OAuth-style flow or by providing a few keys, and then toggle which notifications (e.g., "Order Confirmed", "Appointment Reminder") to enable.
  *   **Pricing:** Twilio offers pay-as-you-go pricing per conversation, which is highly scalable for small businesses. There is a free tier for testing and a reasonable entry point that aligns well with SMB budgets.
  *   **SaaS Viability:** Excellent. The API is robust, well-documented, and supports webhooks for delivery status updates. It can operate in Cloud environments effectively.

  ## Design Doc
  *   **Trigger:** Internal events within OHC (e.g., a new order is placed, an appointment is scheduled, an order ships).
  *   **Action:** OHC formats a predefined, localized template message and dispatches it via the Twilio WhatsApp API to the customer's phone number.
  *   **User Interface:** A new "Notifications" or "Messaging" settings page in the OHC dashboard. The user will see a "Connect WhatsApp" button. Once connected, they will see a list of toggles for different notification types (e.g., Order Confirmation, Shipping Update). They can preview the message templates.

  ## Implementation Prompt
  Create a new integration module that connects OHC to the Twilio WhatsApp Business API. The outcome should be a user-facing settings page where a small business owner can easily link their WhatsApp account and enable automated messages for key business events (like order confirmations). The integration must gracefully handle missing customer phone numbers and provide simple status feedback (e.g., "Message Sent") on the relevant order/appointment details page.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []