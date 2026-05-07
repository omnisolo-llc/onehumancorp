# MessageBird Global SMS Integration

## Problem Statement
Small business owners have customers who prefer or only rely on text messages rather than email (e.g., appointment reminders, delivery updates). Email open rates can be low, leading to missed appointments and lost revenue. Merchants need a reliable way to send automated text messages to customers globally without dealing with complex telecom regulations.

## Research Report
MessageBird (now Bird) provides global cloud communications including SMS, WhatsApp, and voice.
- **Ease of Use**: Offers a user-friendly API and a visual flow builder, though the OHC integration will abstract this completely for the merchant.
- **Capabilities**: High deliverability globally, supports two-way SMS, automated opt-out handling, and local number provisioning.
- **Competitors**: Twilio, Vonage. MessageBird is highly competitive internationally and often has better pricing in European and Asian markets compared to Twilio.
- **Reputation**: Very strong reputation for international messaging and omnichannel support.
- **Pricing**: Pay-as-you-go pricing (e.g., $0.008 per SMS in the US, varies globally). Often requires a small monthly fee for a dedicated local number.
- **Deployment**: Standard REST API. Completely compatible with Cloud and Standalone (outgoing requests only).

## Design Doc
OHC will integrate with the MessageBird API to send transactional SMS messages. Merchants will toggle on "SMS Notifications" in their settings. When critical events occur (e.g., "Order Ready for Pickup" or "Appointment Tomorrow"), OHC's backend will format a short, friendly text message and send it via MessageBird to the customer's phone number. Opt-outs (e.g., replying "STOP") will be handled via MessageBird webhooks updating the customer's contact preferences in OHC.

## Implementation Prompt
Create an "SMS Alerts" section in the OHC settings. Allow the merchant to turn on specific text notifications (e.g., Appointment Reminders, Order Updates). When an event triggers, send a plain-text SMS to the customer via the integration. Display a small indicator on the customer's profile in the OHC dashboard showing whether they are opted-in to receive texts. Ensure the sent messages are logged in the customer's activity feed.

## Priority
P1

## Estimated Scope
Small
