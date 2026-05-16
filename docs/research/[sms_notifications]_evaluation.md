# [SMS & Notifications] Twilio vs MessageBird Evaluation

## Title
Automated SMS Reminders via Twilio or MessageBird

## Problem Statement
Small business owners (especially non-technical ones or those with lower English proficiency) rely heavily on SMS for critical customer communication. Missed appointments, unread emails, and delayed updates lead to lost revenue. They need an automated, reliable way to send SMS reminders globally without dealing with complex carrier compliance.

## Research Report
- **Strategy**: Direct API integration for automated SMS.
- **Persona**: Food service operators, local service providers, international merchants.
- **Advantages**: Excellent global coverage, simple API. Twilio is the industry standard; MessageBird has strong international presence.
- **Risks**: US A2P 10DLC compliance is still a hurdle for merchants sending to US numbers.
- **Pricing**: Pay-per-message (~$0.0079/msg in US for Twilio). Both are affordable.
- **Compatibility**:
  - **Cloud**: OHC manages a central account/compliance.
  - **Standalone**: Requires a guided setup wizard for the user to provide their own API key.

## Design Doc
- **Trigger**: An appointment is booked, or an order is ready for pickup.
- **Action**: OHC automatically sends a pre-configured SMS template to the customer.
- **User Interface**: Business owner sees a simple toggle: "Enable SMS Reminders". They can customize a basic text template without touching API keys.

## Implementation Prompt
Implement a notification toggle in user settings to enable SMS reminders for appointments. When enabled, send a generic text message 24 hours before the appointment. The UI should simply explain the message content and provide a field to customize the closing greeting.

## Priority
P1

## Estimated Scope
Medium
