# MessageBird SMS Integration

## Problem Statement
Many small business customers ignore emails. For urgent updates (e.g., appointment reminders, order deliveries), SMS is crucial. Small business owners need a reliable, global way to send automated SMS notifications without dealing with complex telecom APIs.

## Research Report
MessageBird (now part of Bird) is a global cloud communications platform.
- **Ease of Use**: Offers a robust API that is easy for developers to integrate. The end-user (business owner) simply toggles SMS notifications on/off.
- **Pricing**: Pay-per-message model, varies heavily by destination country but generally competitive.
- **Reputation**: Strong global presence, high deliverability rates, and reliable infrastructure.
- **Environment**: Cloud API. Works in Cloud and Standalone modes.

## Design Doc
**Trigger**: A specific event occurs in OHC (e.g., appointment booked, order shipped).
**Action**: OHC triggers the MessageBird API to dispatch an SMS to the customer's phone number.
**User Experience**: The business owner configures automated SMS triggers in settings. The customer receives a concise text message with the relevant update.

## Implementation Prompt
Integrate MessageBird to enable automated SMS notifications. Add a settings page where the business owner can toggle which events (e.g., new order, shipping update, appointment reminder) trigger an SMS. Ensure proper phone number validation and formatting before sending.

## Priority
P1

## Estimated Scope
Medium
