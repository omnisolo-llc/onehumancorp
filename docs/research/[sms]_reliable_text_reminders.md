# Reliable Text Reminders for Appointments

**Problem Statement:** Fatima's customers don't check their emails, but they always read their WhatsApp or SMS text messages. She needs OHC to automatically text her customers a reminder 24 hours before their cake pickup.

**Research Report:** Twilio is the industry standard for SMS, offering global reach. However, A2P 10DLC compliance in the US makes it hard for small businesses to set up. For global/local use, Twilio is still the best raw API. Plivo is a strong alternative. Cost is per message.

**Design Doc:** User provides their Twilio credentials (or OHC provides a pooled number in Cloud mode). When an order has a pickup time, a background job schedules an SMS. The customer receives a simple text: "Reminder: Your order from [Business] is ready tomorrow."

**Implementation Prompt:** Add a background worker that checks for upcoming appointments or pickups. Use an SMS provider to dispatch a text message reminder to the customer's phone number 24 hours prior.

**Priority:** P0

**Estimated Scope:** Small
