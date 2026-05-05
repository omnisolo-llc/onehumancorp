# MessageBird Integration for OHC

## Problem Statement
For users like Fatima (Food Cart Operator), SMS is the most reliable way to receive order notifications. Furthermore, many customers prefer SMS updates for order tracking or appointment reminders. While email is common, SMS provides immediate, high-visibility communication essential for time-sensitive businesses.

## Research Report
- **Features & API Suitability**: MessageBird (now Bird) provides a global SMS API, WhatsApp Business API, and Voice API. It's designed for omnichannel messaging.
- **Pricing**: Pay-per-message pricing, varying significantly by destination country.
- **Ease of Use for Non-Technical Users**: High. The business owner toggles "Send SMS Notifications" in settings.
- **Cloud vs. Standalone**: API-based, works in both.
- **Advantages**: Excellent global carrier coverage, omnichannel support (can expand to WhatsApp later).
- **Risks**: High costs for high-volume SMS. Strict regulations on marketing SMS (A2P 10DLC in the US).

## Design Doc
- **Integration Point**: "The Ambassador" (Customer Success) and "The Manager" (Operations).
- **Trigger**: Critical events like "New Order Received" (for the owner) or "Order Ready for Pickup" (for the customer).
- **Action**: OHC triggers a request to MessageBird API to dispatch the SMS.
- **User View**: A toggle in notification settings: "Notify me via SMS for new orders". Customers see an option during checkout: "Send me order updates via text message".

## Implementation Prompt
Implement MessageBird to provide SMS notification capabilities. Create settings allowing the business owner to opt-in to SMS alerts for new orders or bookings. Allow customers to optionally provide a phone number during checkout for order status updates (e.g., "Shipped", "Ready for Pickup"). Ensure the integration handles international phone number formatting (E.164) and graceful failures if the SMS cannot be delivered.

## Priority
P2

## Estimated Scope
Small
