# Title
SMS & Notifications: Twilio Integration for High-Urgency Alerts

# Problem Statement
For users like Fatima (The Food Cart Operator), a web push notification might be missed in a noisy environment. She needs a loud, immediate SMS text message when a new food order arrives. Similarly, customers often prefer SMS updates for order readiness over checking their email.

# Research Report
**Tool Analyzed:** Twilio
Twilio is the global leader in programmable SMS.
- **Ease of Use (for non-technical users):** The end-user does nothing but provide their phone number. All complexity is handled by OHC.
- **Pricing:** Pay-per-message. Can get expensive at scale, so OHC must meter usage or charge for an SMS add-on tier.
- **Reputation:** The gold standard for SMS delivery.
- **Integration Risk:** High regulatory risk (A2P 10DLC compliance in the US, opt-out management). The technical API integration is trivial, but compliance requires robust logic.
- **Cloud/Standalone:** Cloud API.

# Design Doc
- **Trigger:** A new order is placed, or a booking is confirmed.
- **Actions:**
  1. The event is pushed to the OHC background job queue.
  2. The job worker checks if the merchant has SMS notifications enabled and has sufficient "credits."
  3. The worker calls the Twilio API to dispatch the message.
  4. Twilio handles delivery.
- **User Experience:** Fatima receives a text: "New Order #104: 2x Halal Cart Chicken. Paid. Pickup in 15m." Customers receive: "Your order from Fatima's Cart is ready for pickup!"

# Implementation Prompt
Integrate Twilio's SMS API to send automated alerts for critical business events (e.g., new order received, order ready for pickup). Implement a robust background worker that handles the SMS dispatch and tracks usage for billing purposes. Acceptance criteria include successful delivery of a test SMS, proper handling of A2P compliance constraints (e.g., handling STOP replies), and a configuration UI for merchants to toggle SMS alerts.

# Priority
P1

# Estimated Scope
Medium
