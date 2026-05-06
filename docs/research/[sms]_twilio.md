### Title
Native SMS Order Notifications (Twilio)

### Problem Statement
Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives so she can start cooking, directly integrated into OHC's Operations department without a third-party notification service.

### Research Report
- **Strategy**: Direct integration with the Twilio SDK for native outbound SMS.
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Invisible to the user. They just toggle "Send SMS reminders" in their settings.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-per-message. OHC will need to manage quotas or require merchants to buy "SMS Credits".
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

### Design Doc
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
- The Operations Agent decides the optimal time to send the reminder.

### Implementation Prompt
Integrate Twilio SMS to allow the platform to send order confirmations, pickup notifications, and appointment reminders via text message. Include a settings panel for merchants to toggle these notifications on or off. Ensure phone number formatting is handled correctly globally (E.164).
- **Acceptance Criteria**: Customer receives an SMS when their order is marked "Ready for Pickup". Customer receives a reminder SMS before a booked appointment.
- **Priority**: P2
- **Estimated Scope**: Medium
