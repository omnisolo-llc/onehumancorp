# [Video Conferencing] Zoom Integration

**Title**: Integrate Zoom for automatic online meeting link generation

**Problem Statement**: Small business owners like Carlos, who give online lessons or consultations, find it tedious to manually generate and email Zoom links for every booking. They struggle to keep track of links and share them efficiently. They need a tool that auto-generates video links securely and simply.

**Research Report**: Zoom is a massive, industry-leading video communications platform.
- **Ease of use**: High for both hosts and participants. It provides seamless meeting links and robust API support.
- **Pricing**: Freemium. A solid free tier exists (with a 40-minute limit on group meetings). Pro plans start around $15/month.
- **Reputation**: Market leader in video conferencing, highly reliable and ubiquitous since the 2020 remote work surge.
- **Cloud/Standalone**: Functions primarily via Cloud API. Standalone mode requires internet connectivity to generate and access meeting links.

**Design Doc**:
- **Trigger**: Business owner activates "Video Conferencing" in OHC and links their Zoom account via OAuth. A new booking or consultation is scheduled.
- **Action**: OHC calls the Zoom API to generate a unique meeting link for the scheduled time.
- **User Experience**: When an appointment is booked, the business owner and the client automatically receive the unique Zoom link in their email or SMS reminders without manual intervention.

**Implementation Prompt**: Create a one-click Zoom integration. When a user has a connected Zoom account, automatically generate a Zoom meeting link for any scheduled calendar event (e.g., via Calendly integration or manual entry) and display the link clearly in the appointment details.

**Priority**: P1
**Estimated Scope**: Medium