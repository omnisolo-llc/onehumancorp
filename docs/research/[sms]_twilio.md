## [SMS & Notifications] Issue Brief: High-Reliability SMS Alerts

**Title**: Scout 🔍: Integrate Twilio for Critical Order SMS Alerts
**Problem Statement**:
Users like Fatima (Food Cart Operator) work in fast-paced, noisy environments where they might not hear a standard push notification, or they may be in areas with poor internet connectivity. They need highly reliable SMS notifications when a new order arrives.
**Research Report**:
- **Tool**: Twilio Programmable SMS.
- **Evaluation**: Twilio is the industry standard for SMS. It guarantees delivery and provides global carrier coverage.
- **Ease of Use**: The user simply toggles "Send me SMS alerts for new orders" and verifies their phone number.
- **Pricing**: ~$0.01 per message. Because of this cost, it should be restricted to Premium users or metered.
- **Cloud vs. Standalone**: Cloud uses OHC's Twilio account. Standalone would require the user to configure their own Twilio SID and Auth Token.
**Design Doc**:
- "Notifications" setting panel.
- User enters their phone number and verifies it via a one-time code.
- When the backend processes a new paid order, an event is emitted.
- The Notification worker picks up the event and dispatches an SMS via Twilio.
**Implementation Prompt**:
Integrate Twilio to send SMS notifications to the business owner when critical events occur (e.g., new pre-order received). Add a settings UI for the user to verify their phone number and opt-in. Ensure the backend handles Twilio rate limits and securely stores the business owner's verified phone number.
**Priority**: P2
**Estimated Scope**: Small
