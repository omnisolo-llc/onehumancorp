# Twilio SMS Integration for Order Notifications

## Problem Statement
Food cart operators like Fatima rely on immediate pickup. Customers often don't check their email promptly while walking down the street. SMS notifications are crucial to tell the customer "Your order is ready for pickup."

## Research Report
- **Tool**: Twilio Programmable SMS
- **Evaluation**: The industry standard for programmatic text messages. Highly reliable with global carrier routing.
- **Ease of Use for Persona**: Invisible to the business owner. They just mark an order as "Ready", and the customer receives an SMS.
- **Pricing**: Very cheap per message (fractions of a cent in the US, slightly higher internationally). Well within margin to offer as a premium feature or bundle.
- **Reputation**: Gold standard for SMS APIs.

## Design Doc
- **Integration Point**: "Customer Success" department.
- **Trigger**: Order status changes to "Ready for Pickup" or "Shipped".
- **Actions**:
  - OHC formats a short SMS message.
  - Dispatch via Twilio API.
  - Log delivery status via Twilio webhooks.
- **User View**: Business owner just toggles "Enable SMS notifications for customers". They click "Mark Order Ready", and the system does the rest.

## Implementation Prompt
Add an SMS notification toggle in the Store Settings. Integrate the Twilio API to send an SMS to the customer's phone number whenever an order status changes to "Ready for Pickup". Ensure phone number validation and formatting are handled.

## Priority
P0

## Estimated Scope
Medium
