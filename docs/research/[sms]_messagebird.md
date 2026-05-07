# Reliable Global SMS Notifications via MessageBird

## Problem Statement
For users with low English proficiency or those lacking reliable internet access, email is ineffective. Critical notifications (order updates, appointment reminders) are missed. Small business owners need a way to reach their customers instantly on their phones, no matter where they are in the world.

## Research Report
**Tool Evaluated:** MessageBird (Bird) SMS API
- **Ease of Use:** Business owner sets it up once with API keys. After that, it runs transparently in the background, converting critical OHC events into SMS texts.
- **Pricing:** Pay-as-you-go based on destination country. Generally competitive with Twilio but often better pricing/routing in Europe and Asia.
- **Reputation:** Top-tier global communications platform. Known for high deliverability rates and direct carrier connections.
- **Deployment:** Cloud mode is fully supported. Standalone mode works perfectly via outbound REST API calls.

## Design Doc
- **Trigger:** System events (e.g., "Order Ready for Pickup", "Appointment Tomorrow") trigger the notification engine.
- **Action:** OHC formats a short SMS message and sends an API request to MessageBird. Opt-outs (e.g., replying STOP) are handled via incoming webhooks from MessageBird to OHC.
- **User View:** A "Notifications" settings tab where the owner can toggle "Send SMS" on or off for specific event types.

## Implementation Prompt
Integrate the MessageBird SMS API to allow OHC to send transactional text messages to customers. Create a service module that handles sending SMS and processing delivery receipts. Implement webhook handlers to receive SMS replies, specifically handling opt-out keywords (STOP, CANCEL) to automatically update the customer's communication preferences in the CRM.

## Priority
P1

## Estimated Scope
Medium