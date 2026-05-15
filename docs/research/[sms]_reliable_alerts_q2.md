# [sms] Issue Brief: Reliable Order Alerts via SMS

**Title**: Twilio SMS Notifications for Low-Data/Low-English Users
**Problem Statement**: As a food cart operator like Fatima, I don't always have a strong 4G/5G connection to receive push notifications reliably from an app. When someone pre-orders food, I need a simple, immediate SMS text message with the order details so I can start cooking right away.
**Research Report**:
- Evaluated Tools: Twilio, MessageBird, Vonage.
- Ease of Use: Twilio is the global standard for SMS APIs. Very simple to integrate.
- Pricing: Twilio costs ~$0.0079 per SMS in the US, slightly more internationally. This is cheap but must be metered or limited per tenant to avoid abuse.
- Reputation: Highly reliable, excellent global carrier coverage.
- Environment: Cloud mostly.
- Recommendation: Use Twilio to send immediate order alerts to the business owner, and optional shipping updates to the customer.
**Design Doc**:
- **Integration Flow**: In the Operations settings, the user enables "SMS Alerts for New Orders" and verifies their phone number.
- **Actions**: When a checkout webhook fires indicating a paid order, OHC enqueues an SMS job. Twilio sends a short text: "New Order! $15.40 - 2x Falafel Platter. Pickup in 15 mins."
- **User Interface**: Simple toggle in the app settings.
**Implementation Prompt**: Integrate Twilio to send SMS notifications to the business owner when a new order is placed. Add a setting for the owner to opt-in and verify their mobile number. The message must be concise and contain the order total, items, and customer name. Acceptance criteria: A text message is successfully delivered to a verified number upon a completed checkout, and failures are logged but do not break the checkout flow.
**Priority**: P1
**Estimated Scope**: Small
