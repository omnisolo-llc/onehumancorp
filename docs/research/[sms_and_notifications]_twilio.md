# SMS & Notifications - Twilio

## Problem Statement
Many customers ignore emails, and business owners (like food cart operators) need instant notifications on their phones when an order arrives. SMS is the most reliable channel for urgent updates.

## Research Report
Twilio is the standard for programmatic SMS.
- **Ease of Use**: API-first, transparent to the non-technical end-user.
- **Pricing**: Pay-per-message (around $0.0079 per SMS in the US). Very cheap.
- **Reputation**: Industry leader, highly reliable.
- **Cloud/Standalone**: Cloud API.

## Design Doc
- **Trigger**: A time-sensitive event occurs (e.g., new food order, appointment reminder).
- **Action**: OHC sends an SMS via Twilio API.
- **User View**: The business owner receives a text: "New Order #123: 2x Falafel Wrap". The customer receives a text: "Your order is ready for pickup!"

## Implementation Prompt
Integrate Twilio to send SMS notifications. Implement SMS alerts for business owners on new orders and SMS reminders for customers for upcoming bookings or order ready status.
- Acceptance Criteria: Business owner can opt-in to SMS order alerts. Customers receive SMS reminders for bookings.

## Priority
P0

## Estimated Scope
Medium

---
