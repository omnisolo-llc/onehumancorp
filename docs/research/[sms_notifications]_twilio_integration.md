# [SMS & Notifications] Integrate Twilio for Reliable SMS

## Problem Statement
Many small business owners and their customers rely heavily on mobile phones and often miss emails. For urgent actions—like a food cart order being ready for pickup, or a last-minute appointment reminder—SMS is the most reliable channel. They need automated text notifications to reduce no-shows and keep operations smooth.

## Research Report
**Tool Analyzed:** Twilio (Programmable SMS API)

*   **Capabilities:** Global SMS delivery, WhatsApp integration, phone number provisioning, and voice calls.
*   **Ease of Use (for Non-Technical Users):** Invisible to the business owner. They just toggle "Send SMS Reminders" in the OHC settings.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Standard API integration.
    *   *Standalone:* Cannot be self-hosted, requires cloud API access.
*   **Pricing:** Pay-as-you-go, roughly $0.0079 per SMS message in the US. Requires managing A2P 10DLC compliance for US numbers, which can add overhead.
*   **Reputation:** The undisputed market leader in programmable communications. Highly reliable but can be complex regarding compliance.

## Design Doc
**Integration with OHC:**
*   **Trigger:** A scheduled event (e.g., 2 hours before an appointment) or a state change (e.g., Order marked "Ready for Pickup").
*   **Action:** OHC back-end queues an SMS payload and dispatches it via the Twilio API using an OHC-managed sender number.
*   **User Interface:** The business owner configures Notification Preferences ("Notify me by SMS for new orders", "Send customers SMS for pickups").
*   **AI Agent Synergy:** "The Manager" handles the scheduling and dispatch of these alerts. "The Ambassador" can parse simple SMS replies (e.g., user replies "CANCEL") to update booking statuses automatically.

## Implementation Prompt
Integrate Twilio to enable automated SMS notifications.
1.  Implement a backend service to send SMS messages using Twilio credentials.
2.  Add a feature for appointment reminders (e.g., 24h before a booking).
3.  Add a feature for order status updates (e.g., "Your food is ready for pickup!").
4.  Ensure users can opt-in/opt-out during the checkout/booking flow to maintain compliance.

## Priority
P1 (High) - Vital for local services and food & beverage operations.

## Estimated Scope
Medium
