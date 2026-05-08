# [Agentic Core] Autonomous Booking Agent

## Problem Statement
Service-based small business owners (like Carlos the handyman or Leo the music tutor) lose significant revenue and time due to manual booking processes. They rely on disjointed tools (Instagram DMs, SMS, phone calls) to negotiate times, which interrupts their actual work. Existing solutions like Calendly are passive—they require the client to click a link and do the work—and don't integrate seamlessly with quoting and payment systems designed for SMBs. This creates a high drop-off rate and administrative burden.

## Research Report
Based on our competitive analysis and user pain point research (see main report):
- **User Evidence:** "People message me to book, but I lose track of who paid" and "I spend 2 hours a day replying to the same questions on Instagram" are top complaints among service professionals.
- **Competitor Gap:** Shopify lacks native booking (requires expensive apps). Wix has a functional but traditional, non-agentic booking system.
- **Opportunity:** OHC can leapfrog these by providing an invisible agent that handles the back-and-forth negotiation via the user's preferred channel (SMS/WhatsApp) and seamlessly updates the OHC calendar and ledger.

## Design Doc

### High-Level Architecture
- **Trigger:** An incoming message (SMS/Web Chat) recognized by the LLM as a booking intent.
- **Entities:**
    - `Service`: Defines the offering, duration, and price.
    - `Availability`: The business owner's linked calendar/working hours.
    - `Conversation Thread`: Context of the ongoing negotiation.
    - `Appointment`: The finalized booking.
- **Flow:**
    1. Customer messages: "Are you free Tuesday for a guitar lesson?"
    2. Booking Agent checks `Availability` and `Service` rules.
    3. Agent replies (e.g., "Yes! I have 2 PM or 4 PM. Which works better?")
    4. Customer confirms.
    5. Agent creates `Appointment` and sends an automated payment/confirmation link.
- **UI/UX (Mobile-First, 375px):**
    - The business owner does not see a complex configuration screen.
    - They see a simple toggle: "Enable AI Booking Assistant".
    - They set their hours visually (drag-and-drop on a mobile calendar view).
    - The owner receives push notifications when a booking is confirmed, not for the back-and-forth chatter.

## Implementation Prompt

**User-Facing Outcome:**
A business owner can turn on the "Booking Assistant" and instantly start accepting appointments without managing a Calendly link. Their customers can text the business number to book a service, and the AI will autonomously negotiate a time based on the owner's availability, finalize the appointment, and collect payment.

**Critical User Journey (CUJ):**
1. **Setup:** The owner (e.g., Leo) enables the agent and connects his Google Calendar (or sets hours manually in OHC).
2. **Intent:** A customer sends a message asking for a lesson.
3. **Negotiation:** The OHC Booking Agent intercepts the message, checks Leo's calendar, and proposes available slots in natural language.
4. **Confirmation:** The customer selects a time.
5. **Fulfillment:** The agent books the slot, texts the customer a payment link via OHC's payment gateway, and notifies Leo via push notification: "New booking: Sarah, Tuesday 2 PM. Paid."

**Acceptance Criteria:**
- The agent successfully interprets natural language booking requests.
- The agent respects the owner's availability and does not double-book.
- The system handles time zone conversions implicitly based on the owner's location.
- The owner can view all upcoming appointments in a simple list view.
- The feature is fully functional and configurable from a mobile viewport.

## Priority
**P0**

## Estimated Scope
**Large**
