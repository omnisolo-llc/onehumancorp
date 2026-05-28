# [booking] Autonomous AI Voice & Chat Receptionist

## Problem Statement
Carlos (handyman) and Leo (music tutor) are losing business because they can't answer calls or DMs while they are working. Carlos is often on a ladder or driving, and Leo is in the middle of a lesson. Setting up a complex booking system like Calendly or Acuity feels like "another job" they don't have time for. They need a system that "just works" to capture leads and schedule appointments without them touching a screen.

## Research Report
- **Market Gap**: Durable and Wix offer "AI Chatbots," but they are largely reactive and web-only. They don't handle voice calls ( Carlos's primary lead source) or proactively manage calendar conflicts across personal and business lives.
- **Competitor Comparison**:
  - **Durable**: Integrated booking, but requires manual setup of services and availability. No voice component.
  - **Shopify**: Requires apps (e.g., Appointly) which adds cost and complexity Maya finds "overwhelming."
  - **GoDaddy**: Has a basic "Smart Line" but no agentic logic to book appointments.
- **User Evidence**: SMB forums highlight "Phone Tag" as the #1 reason for lost leads in home services. Handymen report missing 30-50% of calls during peak hours.

## Design Doc
### High-Level Architecture
- **Voice Agent**: Integrates with Twilio/OHC-Voice to answer calls. Uses LLM to extract intent (Booking, Quote, Question).
- **Teammate Mesh Integration**: The Voice Agent communicates with the `BookingService` and `CalendarAgent`.
- **Conflict Resolver**: Proactively checks the user's OIDC-linked personal calendar and SIPDB-stored business availability.

### UI/Mobile UX Flow (375px)
1. **CEO Dashboard**: A simple toggle "AI Receptionist: ON/OFF".
2. **Activity Feed**: Cards showing "AI handled a call from [Name] - Booked for Tuesday at 2 PM. [Approve/Reschedule]".
3. **Voice Persona**: Choose a "Vibe" (Professional, Friendly, Urgent).

### AI Agent Integration
- **The Gatekeeper**: An agent that lives on the Twilio webhook, transcribes in real-time, and uses RAG to answer business FAQs from the "Storefront Knowledge Base."

## Implementation Prompt
**Outcome**: A Carlos/Leo can enable a "Voice AI" that answers their business phone number, answers FAQs about their services/pricing, and books appointments directly into their calendar.
**Critical User Journey**: User receives a call -> AI answers "Hi, I'm Carlos's assistant..." -> AI checks availability -> AI books appointment -> User gets a push notification to "Confirm."
**Acceptance Criteria**:
- Successfully handle a voice-to-booking flow without human intervention.
- Synchronize with Google/Outlook calendars.
- Display transcript and booking in the OHC Mobile Dashboard.

**Priority**: P0
**Estimated Scope**: Large
