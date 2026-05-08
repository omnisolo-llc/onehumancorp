# Title: Automatic Zoom Meeting Generation for Services
## Problem Statement
Consultants, tutors, and remote service providers spend tedious time manually creating Zoom links for every booking and emailing them to clients. If a meeting is rescheduled, the links get mixed up. They need this automated.

## Research Report
* **Tool:** Zoom API
* **What it does:** Creates, manages, and deletes Zoom meetings.
* **Ease of Use for Owners:** High. "Sign in with Zoom" OAuth flow.
* **Pricing:** Free API usage for Zoom Pro accounts.
* **Cloud vs. Standalone:** Similar to Google Calendar. Requires a registered OAuth app in Cloud, or a proxy for Standalone.

## Design Doc
* **Trigger:** Customer books a service marked as "Online Video Call".
* **Action:** OHC calls Zoom API to create a scheduled meeting with unique join URLs for host and participant.
* **User Experience:** The owner sees a "Join Meeting" button in their OHC dashboard when it's time for the appointment. The customer automatically receives their unique join link in the confirmation email.

## Implementation Prompt
Implement Zoom integration for digital service bookings. The owner should be able to connect their Zoom account. The acceptance criteria: when a customer books an online service, OHC must automatically generate a unique Zoom meeting and display the join link on the customer's confirmation page and in the owner's appointment dashboard.

## Priority
P2

## Estimated Scope
Medium
