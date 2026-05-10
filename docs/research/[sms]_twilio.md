# Title: Reliable Native SMS Order Notifications

## Problem Statement
Food Cart Operators like Fatima rely on their phones but may miss app push notifications in a noisy environment. They need reliable native SMS alerts when new pre-orders arrive so they can start cooking immediately.

## Research Report
- **Tool Evaluated**: Twilio
- **Persona Value**: High for immediate operational awareness.
- **Advantages**: Global coverage, incredibly reliable, programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Cloud vs Standalone**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

## Design Doc
- **Integration Trigger**: User toggles "Send me SMS for new orders" in Settings. Order is paid.
- **Action**: Operations agent triggers a Twilio API call to send an SMS to the business owner.
- **User Interface**: Settings toggle for SMS notifications.

## Implementation Prompt
Integrate the Twilio SDK to send outbound SMS notifications. Add a setting for the business owner to opt-in to SMS alerts for new orders. Ensure phone number formatting is handled correctly globally (E.164).
- **Acceptance Criteria**: Merchant toggles SMS alerts on. When a new order is paid, the merchant receives an SMS notification with order details.

## Priority
P2

## Estimated Scope
Medium
