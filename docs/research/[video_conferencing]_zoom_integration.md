# [Video Conferencing] Integrate Zoom for Online Services

## Problem Statement
Service providers offering online sessions (tutors, consultants, therapists) waste time manually creating Zoom links, copy-pasting them into emails, and dealing with customers who "can't find the link." They need a system where booking a slot automatically generates a unique video meeting link and attaches it to the calendar invite.

## Research Report
**Tool Analyzed:** Zoom (Video Communications API)

*   **Capabilities:** Programmatic meeting creation, participant management, recording access, and webhook events (e.g., meeting started/ended).
*   **Ease of Use (for Non-Technical Users):** Simple OAuth connection. Once linked, the user never has to manually generate a link again.
*   **Cloud vs. Standalone:**
    *   *Cloud:* REST APIs with Server-to-Server OAuth or standard user OAuth.
    *   *Standalone:* Requires cloud API access; Zoom cannot be self-hosted.
*   **Pricing:** Free tier available for basic API usage (meetings < 40 mins). Pro plans start around $15/month for longer meetings.
*   **Reputation:** The ubiquitous standard for video conferencing. High reliability and user familiarity.

## Design Doc
**Integration with OHC:**
*   **Trigger:** A customer books an "Online" service through the OHC booking system.
*   **Action:** OHC uses the business owner's linked Zoom account to generate a unique meeting URL via API.
*   **User Interface:** The business owner links their Zoom account via a simple OAuth flow in Settings. Customers see a prominent "Join Video Call" button on their booking confirmation page and email.
*   **AI Agent Synergy:** "The Operations Manager" appends the Zoom link to all reminder emails. "The Advisor" can note if the meeting was attended (via webhooks) and trigger a follow-up email.

## Implementation Prompt
Integrate the Zoom API to auto-generate meeting links for online bookings.
1.  Add an OAuth connection flow for owners to link their Zoom accounts.
2.  When defining a "Service," allow the owner to mark it as an "Online Meeting (Zoom)."
3.  Upon successful booking of such a service, call the Zoom API to create a meeting.
4.  Store the generated `join_url` and display it in both the owner's dashboard and the customer's confirmation emails.

## Priority
P2 (Medium) - Important for specific digital service personas (like Leo the Tutor).

## Estimated Scope
Medium
