# [sales] Proactive Lead & Booking Follow-up Agent ("The Salesperson")

## Problem Statement
Service providers like Carlos (handyman) and Leo (music tutor) lose up to 50% of potential leads because they are too busy working to respond to inquiries immediately. By the time they check their phone, the customer has already booked someone else. They need a teammate that handles the "first touch" instantly, qualified the lead, and sets up a booking without them lifting a finger.

## Research Report
- **Market Context**: Tools like Calendly or Cal.com require the user to *find* the link. Most customers start with a question ("How much for a leaky sink?").
- **Competitor Gap**: GoDaddy Airo and Durable offer basic "Contact Forms," but they don't *proactively* converse to close a booking.
- **User Evidence**:
    - *r/smallbusiness*: "I'm a plumber. I miss half my calls because I'm under a sink. I wish someone could just book them in for me."
    - *Persona Evidence (Carlos)*: Misses leads when busy; quoting is a manual, late-night chore.

## Design Doc
### Architecture
- **Entity Relationship**: `Lead` <-> `BusinessMemory` (Pricing/Services) <-> `BookingSlot`.
- **Integration Points**: OHC Unified Inbox, `BookingService`, `CalendarService`.
- **Agent Integration**: "The Salesperson" agent uses `BusinessMemory` to answer basic service questions and "Agentic Reasoning" to determine when to offer a booking link or a draft quote.

### UI/UX Flow (375px Mobile)
1. **Incoming Lead Notification**: "New message from Sarah: 'Can you fix a clogged drain tomorrow?'"
2. **Agent Draft**: Below the message, a "Suggest Reply" section shows: "I've drafted a quote ($150) and found a 2pm slot for you. Send?"
3. **Approval**: User swipes right on the draft to send the quote and booking link via SMS/DM.

## Implementation Prompt
Implement the "The Salesperson" proactive follow-up system. The system must:
1. Monitor the OHC Unified Inbox for new messages.
2. Use business context (services offered, price ranges) to draft responses to common inquiries.
3. Check the `BookingService` for real-time availability.
4. Create a "Next Best Action" in the user's dashboard: "Reply to Sarah with Quote & Booking Link".
5. Ensure the "Grandmother Test" is met: the user should never have to type a manual quote if the agent has enough context.

**Acceptance Criteria**:
- 1-tap approval for draft quotes.
- Automatic booking link insertion based on real-time calendar availability.
- "Suggest Reply" feature remains in "Draft" state until user approves (The Advisor pattern).

## Priority: P0
## Estimated Scope: Medium
