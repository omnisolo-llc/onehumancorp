# SMS & Notifications: Twilio

**Title**: Integrate Twilio for Global SMS Alerts & Customer Notifications

**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives so she can start cooking, directly integrated into OHC's Operations department without a third-party notification service.

**Research Report**:
- Direct integration with the Twilio SDK for native outbound SMS.
- **Pricing**: Pay-per-message. OHC will need to manage quotas or require merchants to buy "SMS Credits".
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

**Design Doc**:
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
- The Operations Agent decides the optimal time to send the reminder.

**Implementation Prompt**: Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers.
- **Priority**: P2
- **Estimated Scope**: Medium
- **Acceptance Criteria**:
  - Settings allow toggling SMS notifications.
  - Paid orders trigger Twilio SMS notifications to the business owner and customers.

**Strategy**: Integrate Twilio SDK to natively dispatch SMS notifications for critical alerts.
