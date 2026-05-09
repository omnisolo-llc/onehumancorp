# Scout: Tool Integration Research Q2

## 6. SMS & Notifications
**Title**: Integrate Twilio for Global SMS Alerts & Customer Notifications
**Problem Statement**: Fatima the Food Cart Operator doesn't have a reliable internet connection at her cart and relies on SMS text messages to know when a pre-order arrives.
**Research Report**:
- Twilio is the industry standard for SMS and WhatsApp messaging globally.
- Reliable delivery, deep global coverage.
- Supports WhatsApp, which is critical for markets outside the US.
- Integrates well with the OHC platform to provide seamless messaging.
- Costs per message, can be passed to the tenant or subsidized in premium tiers.
**Design Doc**:
- Users can enable "SMS Notifications" in the "Operations" settings.
- When an order is placed, the OHC platform triggers a Twilio message to text the business owner.
- Additionally, "The Ambassador" can send order confirmation texts to customers who prefer SMS over email.
**Implementation Prompt**: Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers.
**Priority**: P0
**Estimated Scope**: Small
