# [Booking] Integrated Conversational Appointments

## Title
Native Conversational Booking Agent for Service Businesses

## Problem Statement
Service-based solopreneurs (like Carlos the Handyman or Leo the Tutor) lose leads because they cannot answer calls while working, and standard calendar booking pages feel too rigid for their clients. They need a system that feels like texting but functions like a rigorous booking system.

## Research Report
- **Frequency:** 25% of service business owners cite scheduling chaos and double-booking as major pains.
- **Competitor Gap:** Shopify ignores services. Squarespace offers Acuity, which is powerful but requires users to navigate complex calendar UIs.
- **Market Opportunity:** Service businesses represent a massive, underserved beachhead market.

## Design Doc
- **Core Entity:** `BookingAgent` linked to `TenantCalendar`.
- **Integration Points:** SMS/WhatsApp gateways, Calendar sync.
- **UX Flow:**
  - Customer texts the business: "Do you have time to fix a leaky pipe this week?"
  - AI replies: "Yes, Carlos has availability on Thursday at 2 PM or Friday at 10 AM. Which works for you?"
  - Customer replies, AI creates the tentative calendar block and notifies the owner.

## Implementation Prompt
Build an agentic booking system that interprets natural language requests for appointments, checks tenant availability, and negotiates a time slot with the customer via chat/SMS.
- The CUJ involves a customer interacting with the booking agent via a chat interface and successfully reserving a time slot, which then appears on the merchant's OHC dashboard.

## Priority
P0

## Estimated Scope
Large