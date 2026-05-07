# Global SMS & WhatsApp Notifications

**Problem Statement:**
Many customers ignore emails. For time-sensitive updates (like appointment reminders or order deliveries), SMS or WhatsApp messages are critical. This is especially true in regions where email penetration is low or for non-technical users who rely heavily on their phones.

**Research Report:**
- **Evaluated Tools:** Twilio API, MessageBird.
- **Ease of Use:** Completely transparent to the user. They just check a box that says "Send text reminders to customers."
- **Pricing:** Variable by country, but generally affordable (e.g., $0.01 - $0.05 per message).
- **Reputation:** Twilio is the gold standard for global SMS delivery. MessageBird is excellent for WhatsApp and European/Asian markets.
- **Cloud vs Standalone:** Requires Cloud infrastructure for API key security, though Standalone could prompt users for their own API keys (less ideal for non-technical users).

**Design Doc:**
- **Trigger:** An event occurs (e.g., appointment in 24 hours, order shipped).
- **Action:** OHC triggers an API call to Twilio/MessageBird to send a templated SMS to the customer's phone number.
- **User Interface:** A simple toggle in the settings: "Enable SMS Notifications." A field to collect customer phone numbers during checkout/booking.

**Implementation Prompt:**
Implement SMS notification capabilities via Twilio or MessageBird. Allow business owners to toggle automatic SMS reminders for upcoming appointments or order shipments. The system must validate phone numbers, handle international dialing codes cleanly, and provide basic delivery status logs for the business owner.

**Priority:** P1
**Estimated Scope:** Small
