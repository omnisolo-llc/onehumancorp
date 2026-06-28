issue_title: "Integrate WhatsApp Business API for Unified Customer Triage"
issue_description: |
  **Title**: Integrate WhatsApp Business API for Unified Customer Triage and Automated Replies

  **Problem Statement**:
  Non-technical operators like Maya (Home Baker) and Carlos (Field Service) receive a significant portion of their customer inquiries, orders, and service requests via WhatsApp. Currently, these messages are siloed on their phones or the standard WhatsApp Business app. They must constantly context-switch between WhatsApp, scheduling tools, and payment apps to convert conversations into action. Because OHC cannot see these messages, the Work Triage feed is incomplete, and the AI Customer Assistant cannot draft replies or capture demand where the customers actually communicate.

  **Research Report**:
  - **Market Need**: WhatsApp is the dominant communication channel for small businesses globally, with over 200 million monthly active business users. Platforms like WeCom, DingTalk, and HubSpot heavily emphasize omnichannel messaging as a core value proposition.
  - **Tool Evaluation**: The Meta WhatsApp Business Cloud API provides robust webhooks for real-time messaging, media support (crucial for Carlos receiving photos of broken equipment or Maya getting cake design ideas), and structured message templates.
  - **SaaS Viability & Pricing**: Meta offers the first 1,000 user-initiated service conversations per month for free, making it highly accessible for small-business owners. Beyond that, the cost per conversation is manageable.
  - **Ease of Use for Owners**: Meta provides an "Embedded Signup" OAuth flow, allowing owners to connect their WhatsApp Business number directly inside the OHC app without ever seeing a developer portal or managing API keys.

  **Design Doc**:
  - **Trigger**: An inbound WhatsApp message hits the OHC webhook, which routes the message to the specific tenant's Work Triage feed.
  - **Action**: The Customer Assistant reads the new message, looks up the customer by phone number, and generates a drafted reply (e.g., a quote for Carlos or availability for Maya) based on the owner's operational data.
  - **User Visibility**: The owner sees the new WhatsApp message in their daily feed with a clear "WhatsApp" badge. Below the message, the drafted reply is shown with a "Send via WhatsApp" button.
  - **Setup**: A simple "Connect WhatsApp" card in the settings that pops up the Meta Embedded Signup modal.

  **Implementation Prompt**:
  - Add a "Connect WhatsApp" integration option in the user settings that initiates the Meta authentication flow.
  - Update the Work Triage UI to display inbound WhatsApp messages, supporting text and basic image media.
  - Enable the AI Customer Assistant to draft replies specifically for the WhatsApp channel, appearing directly below the received message in the owner's feed.
  - Provide a one-tap approval button for the owner to send the drafted reply back to the customer via the WhatsApp API.
  - Ensure the UI gracefully handles offline states (e.g., queueing outbound messages if the phone is offline and displaying truthful pending states).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
