# Issue Brief: Autonomous Booking Agent

## Title
Implement Autonomous Booking Agent for Service Businesses

## Problem Statement
Service-oriented small businesses (like Carlos, the handyman, or Leo, the music tutor) rely heavily on appointment scheduling. Currently, this process involves chaotic text threads, missed calls, and manual calendar management. Existing solutions require the business owner to set up complex scheduling rules and send links to clients. This manual overhead leads to lost leads and double-bookings.

## Research Report
While platforms like Wix offer scheduling modules, they are passive tools. The business owner must configure everything and drive traffic to the booking page. Research indicates that service professionals want a system that actively manages their calendar and follows up with leads. An "Autonomous Booking Agent" that can handle scheduling via natural language (SMS/Chat) represents a massive differentiator for OHC in the service vertical.

## Design Doc
```mermaid
graph TD
    A[Client Request (SMS/Chat)] --> B(Booking Agent)
    B -->|Checks Availability| C[(Owner Calendar)]
    C -->|Returns Slots| B
    B -->|Proposes Times| A
    A -->|Confirms Slot| B
    B -->|Updates| C
    B -->|Sends Reminder| A

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C premium;
```

*   **Architecture:** The agent integrates with the existing task queue and messaging infrastructure.
*   **Key Relationships:** Requires deep integration with a calendar entity and the user profile (for working hours/availability).
*   **UI Flow:** The dashboard shows a simple calendar view. The owner can set working hours in plain language ("I work Monday to Friday, 9 to 5").
*   **Mobile UX (375px):** The owner receives simple push notifications for new bookings ("Carlos booked a plumbing check for Tuesday at 2 PM").

## Implementation Prompt
Create an Autonomous Booking Agent that allows clients to schedule appointments via natural language conversation (e.g., via a chat widget or SMS integration). The agent must understand the owner's availability and negotiate appointment times with the client without the owner's intervention.

The Critical User Journey (CUJ):
1.  Client messages the business: "I need a tune-up sometime next week."
2.  Booking Agent checks the calendar and replies: "I can fit you in on Tuesday at 10 AM or Thursday at 2 PM. Do either of those work?"
3.  Client confirms Thursday.
4.  Agent books the slot, updates the calendar, and confirms with the client.

Acceptance Criteria:
*   The agent must successfully navigate a multi-turn conversation to finalize a booking.
*   The agent must handle conflicts (e.g., if a slot is taken during the conversation).
*   The owner must be able to configure their availability using plain language settings, not complex time matrix grids.

## Priority
P1

## Estimated Scope
Medium
