# Title: Cal.com Integration for Seamless Scheduling

## Problem Statement
Small business owners rely on scheduling consultations or services, but manually going back and forth via email to find a time is inefficient and frustrating. They need an automated way for clients to book available slots.

## Research Report
Cal.com is an open-source calendar scheduling infrastructure.
- **Ease of use:** Very easy for business owners to set availability and share a link.
- **Pricing:** Open source and free for individuals, paid for teams. Great fit for small businesses.
- **Reputation:** Excellent, modern API, developer-friendly.
- **Key advantages:** Fully open source, highly customizable, and respects user privacy.
- **Risks:** The self-hosted version might require maintenance overhead if we decide to bundle it with our Standalone offering rather than relying on their cloud API.
- **Environment:** Cloud works perfectly via API/webhooks. Standalone works well because Cal.com can run entirely self-hosted, fitting our private data ethos.

## Design Doc
- User goes to "Scheduling" and connects their Google or Outlook calendar to resolve conflicts.
- User sets availability rules (e.g., Mon-Fri 9-5).
- A public "Booking Page" is generated.
- When a customer books a slot, an event is created on the owner's calendar, and a confirmation email is sent to the customer.

## Implementation Prompt
Implement a scheduling interface where users can set their weekly availability. Generate a public booking link that displays available slots. When a client selects a time, create a meeting event in the system and notify the owner. Ensure it handles timezone differences gracefully.

## Priority
P1

## Estimated Scope
Medium
