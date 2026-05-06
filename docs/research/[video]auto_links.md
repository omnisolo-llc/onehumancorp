# [Video Conferencing] Auto-Links

## Title
Implement Automated Video Meeting Links

## Problem Statement
Small business owners offering online consultations, tutoring, or virtual fitness classes struggle with the logistics of managing meeting links. They manually create a Zoom meeting for every booking, copy the link, and email it to the client. This is time-consuming and leads to errors (e.g., sending the wrong link or forgetting entirely), resulting in missed sessions and a poor professional image.

## Research Report
### Zoom Evaluation
- **Overview:** Zoom Workplace is a proprietary videotelephony software program developed by Zoom Communications, widely recognized as the standard for video meetings.
- **Key Benefits for SMBs:**
  - **Ubiquity:** Almost all customers are familiar with Zoom and have it installed.
  - **Reliability:** High-quality video and audio with a robust infrastructure.
  - **Features:** Supports recording, screen sharing, and waiting rooms, which are essential for professional consultations.
- **Challenges/Risks:**
  - **Software Requirement:** Requires the user to download an app (unlike browser-based alternatives like Google Meet), adding a slight friction point.
  - **Security Settings:** Managing passwords and waiting rooms via API can be complex.
- **Ease of Use for Non-Technical Users:** High. Most users are already familiar with generating links manually; automating it removes the only friction point.
- **Cloud vs. Standalone:**
  - **Cloud:** Easily integrated via OAuth and API to generate links automatically upon booking.
  - **Standalone:** Integration is feasible; the standalone app can authenticate with the Zoom API to generate links for events created locally.
- **Pricing Estimate:** Free tier allows 40-minute meetings. Paid plans start at roughly $15/month.

## Design Doc
- **Integration Trigger:** A "Video Conferencing" settings page where the user clicks "Connect Zoom" to authorize via OAuth.
- **Actions Taken:**
  - When an online appointment is booked (either manually by the owner or via the calendar booking widget), OHC calls the Zoom API to create a unique meeting.
  - The generated join URL is automatically attached to the OHC calendar event and included in the automated confirmation/reminder emails to the customer.
- **User Experience:**
  - Business Owner: Connects their account once. Whenever an online meeting is booked, they just see a "Join Video Call" button on the event details in OHC.
  - Customer: Receives the meeting link seamlessly in their confirmation email without any manual intervention from the owner.
  - Simple Mode: Auto-generate standard links. Advanced Mode: Option to enforce waiting rooms or record meetings automatically.

## Implementation Prompt
Integrate Zoom to automate the generation of video conferencing links for online appointments. Create an OAuth flow in settings for the business owner to connect their Zoom account. Whenever a new online appointment is created within OHC, automatically generate a unique Zoom meeting link and append it to the appointment record. Ensure this link is prominently displayed to the business owner and included in any automated customer communications.

## Priority
P2

## Estimated Scope
Medium