# [video] Automated Video Conferencing Links

## Title
Implement Automated Video Conferencing Links (Zoom/Meet Integration)

## Problem Statement
Service providers offering online consultations or classes, like Leo (The Music Tutor), need to generate unique meeting links for every booking. Doing this manually for every student—creating the meeting, copying the link, pasting it into an email—is tedious and prone to errors. They need OHC to automatically generate and attach secure video links to confirmation emails and calendar invites immediately upon booking.

## Research Report
### Market Evaluation
- **Zoom API**: The most requested video platform.
    - *Ease of use (for user)*: Requires Zoom OAuth connection.
    - *Ease of use (for OHC)*: Robust API, well-documented. Requires maintaining server-to-server OAuth or user-level OAuth tokens.
    - *Cloud vs. Standalone*: Works well in Cloud. In Standalone, users would need to set up their own Server-to-Server OAuth app in the Zoom App Marketplace, which is a significant hurdle.
- **Google Meet API**: Tightly coupled with Google Calendar.
    - *Ease of use (for user)*: If they connect Google Calendar, Meet links can often be auto-generated without a separate OAuth flow.
    - *Ease of use (for OHC)*: Easier to implement if Google Calendar sync is already in place.
    - *Cloud vs. Standalone*: Follows the same Cloud/Standalone constraints as the Google Calendar API integration.
- **Whereby / Daily.co**: Embedded video APIs.
    - *Pros*: OHC could offer native "OHC Video Calls" entirely within the browser without the user needing a Zoom account.
    - *Cons*: Users often prefer the familiarity of Zoom/Meet. Per-minute cost for embedded video is higher for OHC.

### Integration Risks & Considerations
- **Link Security**: Meeting links must be unique per booking to prevent "zoombombing" or previous clients joining subsequent sessions.
- **Token Expiry**: Similar to calendar sync, maintaining valid OAuth tokens for Zoom requires robust refresh logic to avoid silent failures when a booking is made.
- **Cancellations/Rescheduling**: If a meeting is rescheduled via the AI agent, the video meeting time must also be updated via the API, or a new link generated to ensure accurate host controls.

## Design Doc
### User Experience
1. **Connection**: In the "Operations" tab under "Services," Leo edits his "1-Hour Guitar Lesson" service. He toggles "Online Meeting" and selects "Connect Zoom." He completes the Zoom OAuth flow.
2. **Booking**: A student books a lesson on Leo's public page.
3. **Automation**: OHC calls the Zoom API, creates a unique meeting for that specific time, and saves the Join URL.
4. **Delivery**: The student receives a confirmation email (drafted by the "Customer Success" agent) that prominently displays the "Join Meeting" button. The link is also added to the calendar event synced to Leo's Google Calendar.

### System Flow
- OHC adds a `VideoProvider` integration architecture.
- When a service configured for online meetings is booked, the scheduling engine invokes the configured provider (e.g., Zoom).
- OHC makes a POST request to `/users/me/meetings` with the booking start time, duration, and a unique password setting.
- OHC stores the `join_url` (for the student) and `start_url` (for the host/user) in the database linked to the specific booking record.
- If the booking is canceled or rescheduled, OHC calls the respective PATCH/DELETE endpoint on the video provider.

## Implementation Prompt
Implement automated video conferencing link generation for service bookings, starting with Zoom API integration. Create an OAuth flow allowing users to connect their Zoom accounts. Modify the booking pipeline so that when an "Online" service is booked, a unique meeting is automatically created via the API. Ensure these links are embedded securely in customer confirmation emails and user calendar events. Handle the lifecycle of the meeting (creation, updating on reschedule, deletion on cancellation). Do not prescribe specific database schemas or API endpoints; focus on the reliability of the link generation and token management.

## Priority
P2

## Estimated Scope
Medium