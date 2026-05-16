# Title: Integrate Zoom for Auto-Generated Online Consultations

## Problem Statement
Coaches, tutors, and consultants who offer online sessions waste time manually creating Zoom links for every new booking and emailing them to clients. They need a system that automatically generates a unique video meeting link as soon as a client books a time slot.

## Research Report
Zoom is the ubiquitous platform for video conferencing.
- **Ease of Use:** Very high familiarity for end-clients. For business owners, OAuth integration means they can connect their account with one click without messing with API keys.
- **Pricing:** The basic tier is free (40-minute limit on group meetings, but 1:1 is unlimited). Pro tier is $14.99/month. Highly accessible.
- **Reputation:** The global standard for video meetings.
- **Competitors:** Google Meet, Microsoft Teams. Google Meet is also highly requested, but Zoom remains the most versatile standalone app that isn't tied tightly to a specific email ecosystem.
- **Cloud vs Standalone:** Server-to-Server OAuth or standard user OAuth works well in Cloud. In Standalone, users would need to create their own Zoom app credentials, which is complex. An alternative for Standalone is simply allowing users to paste a static personal meeting link.

## Design Doc
When a "Virtual" appointment is booked through OHC, the system will automatically generate a unique Zoom meeting.
- **Trigger:** A new booking is confirmed for an event type marked "Online/Virtual".
- **Action:** OHC calls the Zoom API to create a scheduled meeting and retrieves the join URL.
- **User Interface:** The business owner connects their Zoom account via OAuth in settings. When an online appointment is booked, the OHC calendar event and the client's confirmation email both prominently display a "Join Zoom Meeting" button with the unique link.

## Implementation Prompt
Build a Zoom integration via OAuth. Allow the business owner to connect their Zoom account in the integration settings. Modify the booking flow so that if an event type is marked as "Virtual", OHC automatically calls the Zoom API to generate a unique meeting link for that specific appointment. Embed this link in the resulting appointment record and ensure it is included in the confirmation email sent to the client.

## Priority
P2

## Estimated Scope
Medium