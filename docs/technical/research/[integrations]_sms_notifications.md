# [SMS & Notifications] Integrate MessageBird for Global SMS

## Problem Statement
For users with limited English or low internet connectivity (like Fatima the Food Cart Operator), push notifications and emails are unreliable. They need immediate SMS alerts when a new order arrives, and their customers need SMS order confirmations.

## Research Report
**Evaluated Tool:** MessageBird API (now Bird)
**Alternatives Considered:** Twilio, Vonage
**Pros:** Excellent global coverage, competitive pricing outside the US, and a unified API that also handles WhatsApp. Strong omnichannel capabilities.
**Cons:** Less market dominance in the US compared to Twilio; recent rebranding may cause minor API documentation confusion.
**Ease of Use for Non-technical Users:** The user simply provides their phone number and toggles "SMS Alerts" on. No technical setup required.
**Pricing:** Pay-per-message, varies by destination country.
**Deployment:** Cloud-native.

## Design Doc
**Integration with OHC:**
- **Trigger:** A critical event occurs (e.g., new paid order, pickup ready).
- **Action:** OHC sends a templated SMS via the MessageBird API to the business owner or the customer.
- **AI Agent Interaction:** "The Operations Manager" decides when an SMS is necessary (vs. email) based on user preferences and urgency.
- **User View:** A simple toggle in settings: "Send me an SMS for new orders", and a field in checkout for customers to opt-in to SMS updates.

## Implementation Prompt
Integrate the MessageBird API for sending transactional SMS messages. Add preference toggles in the tenant dashboard for receiving SMS alerts. Ensure checkout flows capture customer phone numbers and opt-in consent, and trigger SMS confirmations for pickups/deliveries.

## Priority
P1

## Estimated Scope
Small
