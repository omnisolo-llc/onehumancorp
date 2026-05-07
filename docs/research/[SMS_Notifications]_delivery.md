# Title: Reliable SMS Notifications for Order and Booking Updates

## Problem Statement
Many customers, particularly in certain demographics or regions, do not reliably check their email. For small business owners catering to these groups (like Fatima, who has low-English-proficiency customers), critical updates like order confirmations, delivery tracking, or appointment reminders get missed if sent only via email. This leads to no-shows, missed deliveries, and frustrated customers. The business needs a seamless way to send automated SMS updates without managing a separate texting service.

## Research Report
We evaluated SMS API providers to determine the best fit for OHC:
- **Twilio:** The industry standard. Massive global reach, high reliability, and extensive documentation. However, setting up A2P 10DLC compliance (required for sending SMS to US numbers) is notoriously complex and time-consuming, which could be a major hurdle for our non-technical users if we expose it directly.
- **MessageBird:** Excellent international coverage and slightly easier onboarding in some regions. Pricing is competitive.
- **Plivo:** Another strong alternative to Twilio, often slightly cheaper for international routing.
- **Cloud vs. Standalone Compatibility:** Sending outbound SMS is a simple API call and works flawlessly in both **Cloud** and **Standalone** modes. Handling inbound SMS (replies) or delivery receipts relies on webhooks. As with other webhook-dependent features, Standalone mode would require an OHC relay to receive these asynchronous events reliably.

**Recommendation:** Integrate Twilio for its unparalleled global reach, but strictly manage the complexity on the OHC side. We must act as the primary sender or build an extremely streamlined abstraction over the A2P 10DLC registration process to shield the business owner from technical bureaucracy.

## Design Doc
In the "Notifications" settings, the business owner can toggle "Enable SMS Notifications." They can select which events trigger a text (e.g., "Appointment Reminder 24h before," "Order Shipped"). During checkout or booking, customers are prompted to provide a phone number and opt-in to SMS updates. When a triggered event occurs, OHC automatically dispatches a concise SMS via the API provider. The system handles "STOP" replies automatically to ensure compliance without merchant intervention.

## Implementation Prompt
Implement automated SMS notifications for critical customer events (bookings and order fulfillment). Create a simple settings UI where the merchant can enable SMS for specific triggers. Ensure the customer-facing flows collect phone numbers and explicitly request opt-in consent. Use a reliable API (like Twilio) to send the messages. The system MUST automatically handle carrier compliance requirements, specifically processing "STOP" messages to opt out the customer, ensuring the merchant does not have to manage compliance manually.

## Priority
P0

## Estimated Scope
Medium
