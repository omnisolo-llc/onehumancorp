# Critical SMS Notifications Integration

## Title
Critical SMS Notifications Integration

## Problem Statement
Many small business customers (especially older demographics, or in regions where email is less prevalent) do not check their emails regularly. Appointment reminders, order pickups, or urgent updates sent via email often go unseen, leading to no-shows and frustration. Business owners need a reliable way to send critical, time-sensitive updates via text message.

## Research Report
*   **Tool:** Twilio SMS API, MessageBird.
*   **Market Analysis:** SMS open rates are exceptionally high compared to email. For transactional updates (like appointment reminders), it is the gold standard.
*   **Competitor Analysis:** Dedicated scheduling tools (like Vagaro or Booksy) include this out-of-the-box. General CRMs often require complex Zapier integrations to achieve it.
*   **Ease of Use:** Must be a simple toggle setting for the business owner: "Enable SMS reminders for appointments."
*   **Pricing:** SMS costs money per segment. OHC will need to either bundle a small amount of credits, sell credit packs, or require the user to input their own Twilio credentials.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Straightforward API integration. Requires strict compliance handling (opt-out management, A2P 10DLC registration in the US).
    *   *Standalone:* Can work if the user provides their own API key, but requires robust local scheduling to ensure reminders fire at the correct time even if the app isn't actively being used.

## Design Doc
*   **User Journey:** The business owner navigates to "Notification Settings" in OHC. They toggle on "SMS Reminders". When a client books an appointment, there is a checkbox asking "Send me a text reminder?". 24 hours before the appointment, OHC automatically sends a brief SMS reminder to the client.
*   **Triggers:** Time-based triggers (e.g., 24h before an event), specific status changes (e.g., Order Status changed to "Ready for Pickup").
*   **Actions:**
    *   Format concise text messages.
    *   Send SMS via API provider.
    *   Process opt-out requests (STOP).
*   **Visuals:** A settings panel to customize the template text of the SMS reminder. A log showing delivery status.

## Implementation Prompt
Implement automated SMS notifications for critical events, primarily focusing on appointment reminders and order pickups. Use a provider like Twilio to send the messages. The system must include a way for customers to opt-in during booking and automatically handle standard opt-out replies (e.g., "STOP"). Keep the configuration simple for the business owner by providing sensible default message templates. Ensure the scheduling mechanism for sending reminders is reliable in both Cloud and Standalone architectures.

## Priority
P1

## Estimated Scope
Medium
