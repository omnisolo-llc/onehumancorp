# Integration Issue Brief: SMS & Notifications (Twilio)

## Title
Global SMS Notifications & Marketing: Twilio

## Problem Statement
Many small business owners have clients who do not check email frequently or have lower English/technical proficiency. SMS is a universal, high-open-rate channel. Owners need a reliable way to send appointment reminders, order updates, and marketing blasts via SMS globally.

## Research Report
*   **Tool Evaluated**: Twilio
*   **Ease of Use**: Developer-focused API, but highly reliable. The end-user (business owner) will not interact with Twilio directly; they will use OHC's interface, which wraps Twilio's complexity.
*   **Market Position & Reputation**: The undisputed leader in CPaaS (Communications Platform as a Service). Unmatched global reach and reliability.
*   **Pricing**: Pay-as-you-go usage-based pricing.
    *   **SMS**: Starts at ~$0.0083 per message to send/receive (US pricing), varies heavily by destination country.
    *   **Phone Numbers**: ~$1.15/month for a local US number.
*   **Cloud vs. Standalone Compatibility**: API-based. Fully compatible with both modes.

## Design Doc
*   **Integration Trigger**: OHC administrator configures master Twilio API credentials. (Alternatively, users can plug in their own API keys).
*   **Action Flow**:
    1.  User creates an SMS campaign or a system event triggers an SMS (e.g., appointment reminder).
    2.  OHC formats the payload and calls Twilio's Programmable SMS API.
    3.  Twilio delivers the message and sends delivery status webhooks back to OHC.
*   **User Experience**: The business owner simply types a text message into the OHC marketing/notification dashboard, hits "Send", and the message is reliably delivered to their customers' phones.

## Implementation Prompt
Implement an SMS notification engine using Twilio. Create a backend service that accepts a phone number and message body, and dispatches it via the Twilio API. Expose this service to internal OHC events (like calendar bookings for reminders) and to a user-facing "SMS Campaign" UI. Ensure the system handles opt-outs (STOP messages) appropriately to maintain compliance.

## Priority
P0

## Estimated Scope
Medium
