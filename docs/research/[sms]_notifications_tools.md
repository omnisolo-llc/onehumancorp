# SMS & Notifications Tools

**Title**: Integrate SMS Notifications for Orders and Alerts (Twilio, Vonage)

**Problem Statement**:
Users like Fatima the Food Cart Operator may have limited English or poor data connections, meaning push notifications or emails might be missed. Reliable SMS notifications for new orders are critical for their operations.

**Research Report**:
Evaluated Twilio and Vonage.
- **Twilio**: Industry leader for SMS API.
  - *Ease of Use*: Extremely easy developer experience. High deliverability globally.
  - *Pricing*: Pay-per-message. Complex A2P 10DLC registration required for US numbers.
  - *Reputation*: Gold standard.
- **Vonage**: Strong alternative.
  - *Ease of Use*: Good API.
  - *Pricing*: Often slightly cheaper internationally.
- **Recommendation**: Use Twilio as the primary SMS gateway, but abstract it so we can swap to Vonage if needed. We must build a streamlined UI to handle the A2P 10DLC compliance registration for US users.

**Design Doc**:
- **Trigger**: A customer places an order (e.g., pre-ordering from Fatima's food cart).
- **Action**: OHC backend triggers an SMS via Twilio to the business owner's phone: "New order! 2x Chicken Over Rice. Pickup in 15 mins."
- **User Experience**: The user receives a standard text message. They don't need the app open or a strong internet connection. They can optionally reply to the text to change order status (e.g., reply "READY").

**Implementation Prompt**:
Integrate the Twilio SMS API for sending outbound notifications. Create a user preference toggle in the app to enable "SMS Alerts for New Orders". Implement an onboarding flow to collect required compliance info (A2P 10DLC) if necessary. Consider a webhook receiver to handle simple SMS replies from the business owner to update order status.

**Priority**: P0
**Estimated Scope**: Small
