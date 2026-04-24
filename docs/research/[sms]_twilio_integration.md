# Integrate Twilio for Reliable SMS Notifications

## Problem Statement
Fatima (Food Cart) operates in a noisy environment on a slow mobile connection. She needs instant SMS alerts for new orders, and her customers need SMS pickup confirmations.

## Research Report
- **Tool Evaluated**: Twilio
- **Ease of Use**: Industry standard, highly reliable global SMS API.
- **Pricing**: Pay per message, very cheap.
- **Standalone/Cloud**: Excellent for both.
- **Persona Fit**: Critical for Fatima and local service businesses.

## Design Doc
- **Integration Point**: Operations Agent, Notification System.
- **Trigger**: Order placed or Order ready for pickup.
- **Action**: Send SMS via Twilio API to the owner or customer.
- **User View**: Fatima gets a text: "New Order: 2x Chicken over Rice. Reply READY when done."

## Implementation Prompt
Implement a Twilio SMS sender service. Add a notification preference panel where business owners can enable SMS alerts. Allow the system to send automated SMS updates to customers for order readiness.

## Priority
P1

## Estimated Scope
Small
