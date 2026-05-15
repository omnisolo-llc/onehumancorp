# [AI] Smart Scheduling & Booking Agent

## Title
Smart Scheduling and Booking Agent

## Problem Statement
Service-based businesses (like Leo the music tutor or Carlos the handyman) lose track of their schedules, double-book themselves, and spend too much time negotiating meeting times with clients over text messages.

## Research Report
- **Competitor Landscape**:
  - Calendly is the standard but requires a separate subscription and integration.
  - Squarespace Scheduling (Acuity) is powerful but complex to configure.
- **User Pain Points**:
  - "I missed a $500 job because I forgot to write down the appointment from a text message." (App Store review).
- **Differentiation**:
  - OHC will feature an AI scheduling agent that can read SMS/email negotiations, propose times based on the merchant's calendar, and automatically book the slot.

## Design Doc
- **Architecture**:
  - Entity: `Appointment`, `Availability`, `Service`.
  - Integration: Calendar sync (Google/Apple), NLP for parsing date/time intents.
- **UI Wireframes/Flow**:
  - Mobile UX (375px): Calendar view with "AI Suggestions" overlay.
  - Customer texts: "Are you free next Tuesday?" -> AI drafts reply with available slots -> Merchant approves -> AI sends and books.

## Implementation Prompt
Implement the Smart Scheduling Agent. The Critical User Journey involves the system parsing incoming messages for booking intents, checking the merchant's calendar, and drafting a reply with available time slots. Once confirmed by the customer, the system automatically creates the calendar event.
- **Acceptance Criteria**:
  - NLP extraction of time/date requests from messages.
  - Integration with merchant calendar for availability checking.
  - Automatic event creation upon confirmation.

## Priority
P2

## Estimated Scope
Large
