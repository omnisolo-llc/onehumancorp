## [SMS & Notifications] Issue Brief: Global SMS Alerts & Customer Notifications

**Title**: Scout 🔍: Integrate Twilio for Global SMS Alerts
**Problem Statement**:
Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She doesn't have a reliable internet connection at her cart and relies on SMS text messages to know when a pre-order arrives so she can start cooking, directly integrated into OHC's Operations department.

**Research Report**:
- **Tool**: Twilio API.
- **Evaluation**: Direct integration with the Twilio SDK for native outbound SMS. Twilio is the industry standard for SMS and WhatsApp messaging globally. Reliable delivery, deep global coverage.
- **Ease of Use**: Invisible to the user. They just toggle "Send SMS reminders" in their settings natively.
- **Advantages**: Supports WhatsApp, which is critical for markets outside the US. Simple API, integrates well with Go backend.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-per-message. OHC will need to manage quotas or require merchants to buy "SMS Credits".
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

**Design Doc**:
- User goes to Operations Settings and toggles "Send me SMS for new orders".
- When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
- Additionally, "The Ambassador" can send order confirmation texts to customers who prefer SMS over email.

**Implementation Prompt**:
Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers. Include a settings panel for merchants to toggle these notifications on or off natively. Ensure phone number formatting is handled correctly globally (E.164).
- **Acceptance Criteria**: Merchant can toggle SMS notifications natively. Customer receives an SMS when their order is marked "Ready for Pickup". Merchant receives an SMS for new pre-orders.
**Priority**: P0
**Estimated Scope**: Small
