# Title: Order and Appointment SMS Notifications
## Problem Statement
Many customers (and business owners) ignore emails. SMS is critical for urgent updates like "Your order is ready" or "Your appointment is tomorrow," especially in regions where email usage is low.

## Research Report
Twilio is the standard, but alternatives like MessageBird or local providers might be cheaper internationally. Lets focus on Twilio for broad coverage.
- **Ease of Use**: The business owner doesnt need a Twilio account. OHC manages it and bills the owner for usage, or includes a quota in their subscription.
- **Pricing**: Variable per country, but generally affordable for high-value transactional messages.
- **Reputation**: Twilio is the gold standard for reliability.

## Design Doc
- **Trigger**: System events (e.g., order status changes to "Ready for Pickup", or an appointment is 24 hours away).
- **Action**: OHC triggers an SMS via the Twilio API to the customers phone number.
- **User View**: A settings page where the owner can toggle "Send SMS for new orders" or "Send SMS reminders for appointments." They can customize a simple template (e.g., "Hi {{name}}, your order is ready!").

## Implementation Prompt
Integrate the Twilio SMS API. Create a centralized notification service within OHC that can send SMS messages based on specific triggers (e.g., order updates, appointment reminders). Build a UI in settings allowing the business owner to enable/disable SMS notifications and customize short message templates using variables like `{{customer_name}}` and `{{order_number}}`.

## Priority
P0

## Estimated Scope
Medium

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported. Twilio API requests are managed securely by the cloud backend.
- **Standalone Mode**: Supported if the standalone app is provisioned with a secure Twilio API key or routes through an OHC proxy to protect credentials.
- **Risks**: High costs from SMS abuse, strict local telecom regulations, and undelivered messages.
