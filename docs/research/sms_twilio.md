# Integrate Twilio for SMS Order & Appointment Notifications

## Problem Statement
Many small businesses serve customers who do not check their email regularly or have low English proficiency (like Fatima's bakery scenario). For these customers, SMS is the most reliable and direct way to receive order confirmations, delivery updates, or appointment reminders. Business owners need an automated way to send short SMS alerts directly to their customers' phones.

## Research Report
**Tool**: Twilio
Twilio is the industry leader for programmable SMS and voice.
- **Ease of use**: Requires account setup and purchasing a phone number, which can be slightly technical, but manageable with good UI guidance in OHC.
- **Pricing**: Pay-as-you-go. Roughly $0.0079 per message in the US, plus a small monthly fee for the phone number (~$1.15). Extremely affordable for small volumes.
- **Reputation**: Highly reliable, globally trusted, and handles local regulatory requirements (like A2P 10DLC in the US).
- **Environment**: REST API works universally across Cloud and Standalone environments.

## Design Doc
The integration will add an SMS notification channel to OHC's existing transactional email system.
- **Trigger**: An event occurs in OHC (e.g., "Order Paid", "Appointment Booked", "Order Shipped").
- **Actions**: If the user has Twilio configured and the customer provided a phone number, OHC fires off a short, templated SMS message via the Twilio API.
- **User View**: A "Notifications" settings tab where the owner can toggle which events send an SMS. They can see a log of recently sent messages and their delivery status.

## Implementation Prompt
Create an SMS integration using Twilio. In Settings -> Notifications, add a section for the user to provide their Twilio Account SID, Auth Token, and Sender Phone Number. Update the core order and appointment workflows to check for this configuration. If configured and a customer phone number is present, send a standardized SMS string (e.g., "Your order from [Store Name] is confirmed! Tracking: [Link]"). Ensure the system gracefully handles API errors or invalid phone numbers without crashing the main checkout flow.

## Priority
P1

## Estimated Scope
Medium
