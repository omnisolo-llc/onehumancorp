# Native Integration of Vonage for SMS Notifications

## Title
Native Integration of Vonage for SMS Notifications

## Problem Statement
Many small business owners, especially in the food and local services sectors, rely entirely on their mobile phones and often miss push notifications or emails. They need reliable, instantaneous SMS alerts for new orders or appointment reminders to ensure they fulfill customer requests on time.

## Research Report
- **Strategy**: Direct API integration with Vonage (formerly Nexmo) for global SMS delivery.
- **Target Persona**: Food cart operators, local service providers, and appointment-based businesses.
- **Advantages**: Vonage provides excellent global reach, which is critical for an international platform. Their API is straightforward and highly reliable. They often have better global pricing than Twilio.
- **Risks**: Global SMS regulations (like 10DLC in the US) make onboarding complex. Senders must be verified, which adds friction for small businesses.
- **Pricing**: Pay-per-message model depending on the destination country.
- **Compatibility**: Works best in Cloud mode (using a centralized OHC Vonage account and managing compliance centrally). In Standalone mode, users would provide their own API keys and manage their own compliance.

## Design Doc
- In OHC Settings, the merchant navigates to "Notifications" and enables "SMS Alerts".
- The merchant enters their phone number and verifies it via a one-time code.
- OHC automatically configures routing rules: when an order is placed or a booking is made, an SMS is queued.
- The AI Agent determines the optimal time to send the SMS (e.g., immediate for a hot food order, 24 hours in advance for a consultation).
- **AI Integration**: The Operations Agent can receive replies via SMS (e.g., replying "READY" updates the order status in the OHC backend).

## Implementation Prompt
Integrate Vonage to provide SMS notifications for merchants. Implement a settings panel for merchants to opt-in and verify their phone numbers. Build a backend service that listens for order/booking events and dispatches SMS messages via the Vonage API.
- **Acceptance Criteria**: Merchant can verify their phone number. System sends an SMS alert via Vonage when a new order is received. System sends a reminder SMS before a scheduled appointment.
- **Priority**: P2
- **Estimated Scope**: Medium
