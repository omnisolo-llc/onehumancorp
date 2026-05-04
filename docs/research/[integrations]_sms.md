# SMS & Notifications

## Title
[SMS] Automated Order Notifications and Alerts

## Problem Statement
Users like Fatima (The Food Cart Operator) may not always be looking at the app or have reliable mobile data. SMS notifications are a critical, low-tech way to alert owners of new orders instantly and inform customers that their food is ready for pickup.

## Research Report
- **Evaluated Tools**: Twilio, Vonage, Plivo, MessageBird.
- **Ease of Use**: High via API. The owner simply provides their phone number.
- **Pricing**: Twilio is ~$0.0079 per SMS in the US, but international rates vary wildly (up to $0.10+ in some regions).
- **Delivery Reliability**: Very high.
- **Opt-Out Compliance**: Mandatory compliance (STOP messages) is required.
- **Cloud vs Standalone**: Fully supported.

## Design Doc
- **Triggers**: New order received; order status changed to "Ready".
- **Actions**: System dispatches a formatted SMS via the provider's API.
- **User View**: The owner receives a text: "New Order #102: 2x Chicken Over Rice. Paid." The customer receives a text: "Your order is ready for pickup!"

## Implementation Prompt
Integrate an SMS notification system (e.g., using Twilio) to send critical alerts. Allow business owners to opt-in to receive SMS alerts for new orders, which is especially important for fast-paced environments like food carts. Additionally, enable automatic SMS notifications to customers when their order status changes to "Ready for Pickup" or "Shipped".
- **Acceptance Criteria**: Business owner can opt-in and configure a phone number to receive SMS alerts for new orders. When an order is placed, the owner successfully receives an SMS notification with order details. Customers automatically receive an SMS notification when their order status is changed to "Ready for Pickup" or "Shipped". The system must include a reliable mechanism for customers to opt-out (e.g., replying "STOP").

## Priority
P1

## Estimated Scope
Small
