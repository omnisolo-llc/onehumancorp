# Title: Integrate Twilio for Automated SMS Notifications

## Problem Statement
Many small business customers (especially in service industries or non-English speaking demographics) do not reliably check email. Businesses need a way to send critical updates—like appointment reminders, pickup notifications, or payment links—directly to customers' phones via SMS to reduce no-shows and delays.

## Research Report
Twilio is the industry standard for programmatic SMS and voice communications.
- **Ease of Use:** For the business owner, the experience is completely invisible. They just see a "Send SMS" button in OHC. Setup requires adding a Twilio Account SID and Auth Token, which is slightly technical but manageable with good documentation.
- **Pricing:** Pay-as-you-go, roughly $0.0079 per message in the US. No monthly minimums. Extremely cost-effective.
- **Reputation:** The backbone of modern SMS communications. Highly reliable.
- **Competitors:** MessageBird, Plivo, Vonage. Twilio has the best documentation, widest global reach, and the most robust compliance tools (e.g., A2P 10DLC handling for US numbers).
- **Cloud vs Standalone:** Perfect for both. Standalone users can simply provide their own API keys.

## Design Doc
OHC will allow business owners to trigger SMS messages manually or automatically based on specific events.
- **Trigger:** An automated event occurs (e.g., appointment is 24 hours away) or the owner clicks "Send SMS" on a customer profile.
- **Action:** OHC uses the Twilio API to dispatch an SMS to the customer's verified phone number.
- **User Interface:** A new "SMS Notifications" toggle in the booking/invoicing settings. A chat-like view in the customer's CRM profile showing a history of sent SMS messages and any replies.

## Implementation Prompt
Integrate Twilio to enable outgoing SMS messages. Add a settings page for users to input their Twilio API credentials and associated phone number. Add functionality to send automated appointment reminders via SMS (24 hours prior) and a manual "Send SMS" feature on the Customer Details page. Implement incoming webhooks so customer replies can be viewed within the OHC interface alongside the outbound messages.

## Priority
P1

## Estimated Scope
Medium