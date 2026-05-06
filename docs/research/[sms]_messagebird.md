# Integrate MessageBird for Global SMS Notifications

## Problem Statement
Many small business customers, especially in emerging markets or non-English speaking demographics (like Fatima's customers), do not regularly check email. They rely entirely on SMS for updates. Business owners need a reliable way to send order confirmations, appointment reminders, and critical updates directly to their customers' phones.

## Research Report
*   **Tool:** MessageBird (or Twilio, Vonage)
*   **Problem Solved:** Programmatic sending and receiving of SMS messages globally.
*   **Ease of Use:** Completely invisible to the business owner once set up. It operates automatically in the background based on business events.
*   **Pricing:** Pay-as-you-go per message (e.g., ~$0.0079 per SMS in the US, varies globally).
*   **Reputation:** Enterprise-grade reliability, excellent global carrier routing.
*   **Environment:** Works well in both Cloud and Standalone modes via API.
*   **Advantages:** Almost 100% open rate for SMS; critical for reaching demographics that ignore email; supports two-way messaging.
*   **Risks:** Strict local regulations (e.g., A2P 10DLC in the US, GDPR in Europe) require business verification before sending; SMS costs can add up if abused.

## Design Doc
1.  **Trigger:** A key business event occurs (e.g., Appointment booked, Order ready for pickup).
2.  **Action:** OHC automatically formats a short, localized text message and sends it via the SMS API.
3.  **User Interface:** The business owner configures SMS templates in settings (e.g., "Hi [Name], your order is ready!"). In the customer's profile, the owner can see a log of all SMS messages sent.
4.  **Compliance:** The system automatically appends "Reply STOP to unsubscribe" and handles opt-outs automatically so the owner doesn't break local laws.

## Implementation Prompt
Implement automated SMS notifications to improve customer communication. Allow business owners to configure templates for key events like order confirmations, shipping updates, and appointment reminders. When these events trigger, the system must automatically send the formatted SMS to the customer's verified phone number. Ensure that the system handles global phone number formatting (E.164) and automatically processes "STOP" replies to maintain compliance with telecommunication regulations. Log all sent messages in the customer's history.

## Priority
P1

## Estimated Scope
Small
