# [video] Issue Brief: Auto-Generated Lesson Links

**Title**: Zoom API Integration for Online Consultations
**Problem Statement**: As an online music tutor like Leo, when a student books a lesson, I have to manually create a Zoom meeting and email them the link. I sometimes forget, leading to confusion. I need the system to automatically generate a unique Zoom link and send it to the student when they book.
**Research Report**:
- Evaluated Tools: Zoom API, Google Meet API (via Google Calendar), Daily.co.
- Ease of Use: Google Meet is essentially free and included if we build the Google Calendar sync. Zoom API requires OAuth but is heavily requested.
- Pricing: Zoom requires the user to have a Zoom account (free tier works for 40 mins). Google Meet is completely free.
- Reputation: Zoom is ubiquitous for online learning.
- Environment: Cloud and Standalone.
- Recommendation: If Google Calendar integration (Category 2) is built, automatically attach Google Meet links. Add Zoom API as a secondary option for users who specifically prefer Zoom.
**Design Doc**:
- **Integration Flow**: User connects Zoom via OAuth in the Operations settings.
- **Actions**: When a service is booked that is marked as "Online/Video", OHC calls the Zoom API to create a meeting for the scheduled time. The `join_url` is saved to the database and included in the confirmation email/calendar invite sent to the customer.
- **User Interface**: A toggle on the Service creation page: "Location: [In-Person | Online Video]". If Online Video is selected, a dropdown lets them choose their provider (Zoom or Meet).
**Implementation Prompt**: Implement Zoom API integration to automatically generate video meeting links for online service bookings. When a customer books an "Online Video" service, create a scheduled meeting via Zoom. Display the join link in the business owner's upcoming appointments view and the customer's confirmation page. Acceptance criteria: A valid Zoom link is generated upon booking, correctly scheduled for the booked time, and both parties can access the link.
**Priority**: P3
**Estimated Scope**: Medium
