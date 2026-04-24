# Twilio SMS Integration

**Title**: Implement SMS Notifications via Twilio Programmable Messaging
**Problem Statement**: Not all customers have smartphones or reliable internet, and some business operators (like Fatima the food cart owner) prefer simple text messages over app notifications for order alerts.
**Research Report**:
- **Tool**: Twilio Programmable Messaging API.
- **Ease of Use (End User)**: Completely invisible. They simply receive text messages on their phone.
- **Pricing**: Pay-as-you-go (approx. $0.0079 per outbound SMS in the US, varies globally). Requires A2P 10DLC registration in the US (compliance overhead).
- **Cloud vs. Standalone**: Cloud API. Works in both, but Standalone users might need to provide their own Twilio credentials if OHC doesn't want to act as a proxy/reseller.
**Design Doc**:
- **Trigger**: System events (New Order received, Order Ready for Pickup, Appointment Reminder).
- **Action**: OHC sends a formatted SMS string via the Twilio API to the specified phone number.
- **UI**: Notification preferences toggles ("Send me an SMS when I get a new order", "Send customer an SMS when order is ready").
**Implementation Prompt**: Integrate the Twilio API to send SMS notifications. Implement triggers for critical business events (e.g., new order alerts for the owner, pickup readiness for the customer). Provide UI settings for users to opt-in and configure SMS alerts.
**Priority**: P2
**Estimated Scope**: Medium
