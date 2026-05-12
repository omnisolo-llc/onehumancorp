# SavvyCal Scheduling Integration

## Problem Statement
Small business owners lose time playing "email ping-pong" trying to schedule appointments or consultations. Existing tools can feel impersonal, complicated to set up, or generate links that confuse clients. They need a simple, personalized way to let clients book time directly on their calendar.

## Research Report
SavvyCal is a modern scheduling tool designed to be more intuitive and collaborative than traditional options like Calendly.
- **Ease of Use**: SavvyCal allows schedulers to overlay their own calendar over the recipient's, making it easier to find mutual availability. The interface is highly visual and user-friendly for non-technical users.
- **Pricing**: Basic plan starts around $12/user/month.
- **Reputation**: Highly rated for its modern design and ease of use, particularly for avoiding the "power dynamic" awkwardness of standard scheduling links.
- **Environment**: Primarily Cloud-based. Standalone mode might require a lighter, local alternative or utilizing an open-source option like Cal.com if strict local-only operation is mandatory.

## Design Doc
**Trigger**: Business owner navigates to "Scheduling" and creates a "Booking Link".
**Action**: User connects their Google/Outlook Calendar. SavvyCal generates a personalized booking page.
**User Experience**: The business owner shares a clean, branded link. The client clicks the link and easily selects an open time slot that works for both parties without leaving the page.

## Implementation Prompt
Implement a scheduling feature that allows the business owner to connect their calendar and generate a simple, shareable booking link. The client should be able to see available times and book an appointment directly. Ensure the booking page uses OHC's styling to look professional and personalized.

## Priority
P1

## Estimated Scope
Medium
