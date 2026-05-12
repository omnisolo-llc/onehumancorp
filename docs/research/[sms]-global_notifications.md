# Global SMS Notifications for Customers

## Problem Statement
Customers often miss emails, but read SMS messages, causing missed appointments or unread updates.

## Research Report
Twilio and Vonage evaluated. Twilio has better global coverage and documentation. Pricing is comparable. Both work well with both Cloud and Standalone modes via API.

## Design Doc
Allow business owners to configure automatic SMS reminders for appointments and order updates. Twilio handles the delivery. Status updates are shown in the OHC timeline.

## Implementation Prompt
Add SMS notification settings. Implement automated reminders 24 hours before appointments and when order status changes to 'Shipped'.

## Priority
P0

## Estimated Scope
Medium
