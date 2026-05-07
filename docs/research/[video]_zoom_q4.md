# [Video] Auto-Generated Meetings with Zoom

**Title**: Implement Zoom Integration for Auto-Generated Meeting Links

**Problem Statement**:
When clients book online lessons, consultations, or meetings, the business owner currently has to manually create a Zoom link and email it to the client. This manual step often gets forgotten, leading to confusion at the time of the meeting.

**Research Report**:
- **Evaluated Tools**: Zoom, Google Meet, Microsoft Teams.
- **Findings**: Zoom is universally recognized and widely expected by clients for online meetings. Google Meet is a close second (often bundled with calendar integration). Providing a direct Zoom integration ensures the highest familiarity and reliability for video calls.
- **Ease of Use**: The user authorizes their Zoom account once. After that, meeting link generation is completely invisible and automatic.
- **Pricing**: Zoom requires a paid Pro account ($14.99/mo) for the business owner to host meetings longer than 40 minutes, but the API integration itself is free to use.
- **Cloud vs Standalone**: Functions seamlessly in both environments via standard OAuth2 flows.

**Design Doc**:
- **Trigger**: A new meeting or appointment is scheduled within OHC (either manually or via the booking integration).
- **Action**: OHC makes an API call to Zoom to create a new meeting instance associated with the owner's account and retrieves the join URL.
- **User View**: When viewing the meeting details in the OHC calendar, a large "Join Zoom Meeting" button is present. The generated link is also automatically included in the confirmation emails/SMS sent to the client.

**Implementation Prompt**:
Integrate a video conferencing solution (like Zoom) that automatically generates meeting links. Create a settings panel for the user to connect their Zoom account via OAuth. Modify the appointment/scheduling system so that when a new online meeting is created, a unique Zoom link is instantly generated and saved to the meeting details. Ensure this link is prominently displayed in the UI for both the owner and the customer.

**Priority**: P2
**Estimated Scope**: Medium
