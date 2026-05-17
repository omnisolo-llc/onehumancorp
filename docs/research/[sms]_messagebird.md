# MessageBird Integration Issue Brief

## Title
Integrate MessageBird for Reliable Global SMS Notifications

## Problem Statement
Small business owners, especially those serving older demographics or non-English speakers (like Fatima), find that emails are often ignored. They need a reliable way to send order updates and appointment reminders via SMS to ensure they are seen immediately.

## Research Report
- MessageBird is a global communications platform with strong SMS delivery capabilities worldwide.
- It offers better international rates and deliverability in certain regions compared to competitors.
- Pricing: Pay-as-you-go per SMS, generally competitive.
- Competitors: Twilio (industry standard, slightly more complex API), Sinch.
- Integration: Straightforward REST API for sending SMS messages.
- Cloud/Standalone: Fully supported in Cloud. Standalone mode users provide their API key.

## Design Doc
- Users enter their MessageBird API key in the "Notifications" settings.
- They can enable/disable SMS notifications for specific events (Order Confirmation, Appointment Reminder, Shipping Update).
- The "Concierge" AI drafts concise, translated SMS messages based on the customer's preferred language.
- The OHC backend calls the MessageBird API to dispatch the messages.

## Implementation Prompt
Implement a MessageBird SMS integration. Create a service module that accepts a phone number and a text message, and sends it via the MessageBird API. Integrate this service into the existing notification pipeline, allowing users to configure which events trigger SMS messages. Ensure phone numbers are validated and formatted to E.164 standard before sending.

## Priority
P2

## Estimated Scope
Small
