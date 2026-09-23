issue_title: "Scout: Tool Integration Research - Twilio SMS for Operational Alerts"
issue_description: |
  **Title**: Twilio SMS Integration for Operational Alerts

  **Problem Statement**:
  Business owners (like Fatima, the food cart operator) and service professionals often operate in environments where checking an app or email is not feasible. They need immediate, reliable notifications on their phones when a new order is placed, a booking is made, or an urgent inquiry arrives. Without this, they miss sales, delay fulfillment, and degrade customer satisfaction. For non-technical owners, setting up third-party SMS alerts is complex and error-prone.

  **Research Report**:
  - **Market Need & Context**: Fast-paced physical operations and field services require immediate notification mechanisms. SMS remains the most universal and immediate channel, boasting open rates near 98%.
  - **Tool Evaluation**: Twilio is the industry standard for programmable SMS. It offers global reach, high reliability, and a developer-friendly API.
  - **Ease of Use**: By abstracting Twilio behind OmniSolo, the setup becomes zero-friction for the owner. They simply verify their phone number and toggle "SMS Alerts" on.
  - **Pricing & Viability**: Twilio's pay-as-you-go pricing (roughly $0.0079 per SMS in the US) is cost-effective. OmniSolo can absorb basic alert costs in premium tiers or pass them through transparently.
  - **Risks**: A2P 10DLC compliance in the US requires business registration. OmniSolo will need to guide users through compliance or utilize a shared, pre-approved shortcode/toll-free number for operational alerts to minimize friction.
  - **Cloud vs Standalone**: Cloud mode will utilize a centralized OmniSolo Twilio account. Standalone mode requires the user to input their own Twilio API credentials or use the OmniSolo Cloud Relay.
  - **Matrix**:
    | Feature | Twilio | MessageBird | Alternative SMS |
    | --- | --- | --- | --- |
    | Global Reach | Industry Leader | Strong EU focus | Varies |
    | Reliability | 99.99% SLA | High | Varies |
    | API Developer Experience | Excellent | Good | Varies |
    | Cost | ~$0.0079/msg (US) | ~$0.008/msg (US) | Varies |

  ```mermaid
  graph TD
      Event[Operational Event (e.g., Order Paid)] --> OmniSolo[OmniSolo Backend]
      OmniSolo --> CheckPrefs{Check User Prefs}
      CheckPrefs -->|SMS Enabled| TwilioAPI[Twilio API]
      CheckPrefs -->|SMS Disabled| End[Do Nothing]
      TwilioAPI --> SMS[Send SMS to Owner]
  ```
  - **Source URLs**: https://www.twilio.com/docs/sms

  **Design Doc**:
  - **Trigger**: A critical operational event occurs (e.g., 'Order Paid', 'Booking Confirmed').
  - **Actions**:
    1. The OmniSolo backend detects the event and checks the owner's notification preferences.
    2. If SMS is enabled, OmniSolo formats a concise message (e.g., "OmniSolo Alert: New order #123 for $15.00 - Chicken Over Rice.").
    3. The OmniSolo platform dispatches the message via the Twilio API.
  - **User Experience (The Business Owner)**:
    - In the OmniSolo dashboard (Settings -> Notifications), the owner verifies their mobile number.
    - They toggle switches for specific events (e.g., "Text me when I get a new order").
    - They receive immediate SMS alerts on their phone.

  **Implementation Prompt**:
  Integrate the Twilio SDK to dispatch outbound SMS notifications. Add a settings panel for the business owner to verify their phone number and opt-in to SMS alerts for critical events (like new orders). Ensure the integration handles API errors gracefully and respects the user's notification preferences. Do not prescribe specific database schemas or API endpoints.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
