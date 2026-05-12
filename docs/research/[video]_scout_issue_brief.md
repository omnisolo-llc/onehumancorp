# Video Conferencing Research Brief

## Title
Automated Meeting Link Generation for Online Services

## Problem Statement
Many small businesses (tutors, consultants, therapists) operate entirely online. Manually creating a Zoom or Google Meet link for every appointment and emailing it to the client is tedious and error-prone. The links need to be generated automatically at the time of booking.

## Research Report
### Market Context
Video conferencing APIs have become highly standardized since 2020. Security (passcodes, waiting rooms) is now a default requirement.

### Tool Evaluations

#### 1. Zoom API
- **Ease of Use:** Extensive documentation, but complex OAuth flows.
- **Pricing:** Requires a paid Zoom account for API usage beyond basic limits.
- **Capabilities:** Generating meetings, managing recordings, retrieving attendance reports.
- **Reputation:** Ubiquitous. Clients trust it and usually have the app installed.

#### 2. Google Meet API
- **Ease of Use:** Easier if already embedded in the Google Workspace ecosystem.
- **Pricing:** Included with Google Workspace.
- **Capabilities:** Generates links natively when a calendar event is created via Google Calendar API.
- **Reputation:** Extremely frictionless, runs in the browser without requiring app downloads.

#### 3. Whereby
- **Ease of Use:** Very high API usability.
- **Pricing:** Embedded API pricing varies.
- **Capabilities:** Allows embedding the video call directly within the OHC interface (white-labeled).
- **Reputation:** Great for seamless, browser-based experiences without external branding.

### Recommended Direction
Prioritize Google Meet via the Google Calendar API as the default, as it requires no extra cost or separate accounts beyond what is needed for calendar sync. Add Zoom as a secondary option for users who specifically require it.

## Design Doc
### Trigger & Action
1. **Trigger:** A client books a "Virtual Consultation" service.
2. **Action:** OHC requests a meeting link from the selected provider API and attaches it to the calendar invite.
3. **User View:** The owner sees a "Join Meeting" button next to the appointment in their dashboard. The client receives an email with the direct link and passcode.

### Environment Support
- **Cloud Mode:** Handled via OAuth integration.
- **Standalone Mode:** User must authenticate their own Zoom or Google account.

## Implementation Prompt
Integrate video conferencing generation into the scheduling flow.
- Add a "Location" option to services: Physical Address vs. Video Call.
- When Video Call is selected, automatically generate a mock meeting link (e.g., meet.google.com/abc-defg-hij) upon booking.
- Display the link prominently in the appointment details UI.
- Include the link in the confirmation email payload.
- Acceptance criteria: Booking a virtual service automatically yields a validly formatted meeting URL visible to both the owner and client.

## Priority
P1 (High)

## Estimated Scope
Small

### Extended Video Conferencing Analysis
#### Meeting Security Defaults
Zoombombing and unauthorized access remain significant concerns. The automated link generator must securely configure meeting parameters by default—enforcing randomly generated passwords and enabling waiting rooms. This protects the professional image of the business owner.

#### Link Lifecycle Management
Meeting links should not persist indefinitely. The system must create unique meeting IDs for every single appointment rather than relying on a static Personal Meeting ID (PMI), which could result in clients accidentally dropping in on other appointments.

#### Recording Synchronization
For tutors or therapists, maintaining a secure archive of session recordings is vital. A potential advanced feature could automatically download the cloud recording upon meeting termination and securely attach it to the client's CRM profile within OHC.

#### Browser Compatibility
To ensure a frictionless experience for the client, the chosen integration (like Whereby or Google Meet) must have flawless browser support across mobile and desktop, completely eliminating the need for mandatory application downloads.

### User Persona Match
- **Fatima (Boutique Owner):** Low value. Her sales are driven by physical interactions or direct messaging, not video calls.
- **Carlos (Consultant):** Extremely high value. Video conferencing is the primary medium through which he delivers his service.

### Conclusion
By natively handling meeting generation, OHC removes the final manual step in the booking lifecycle, delivering a fully automated pipeline from initial calendar discovery to face-to-face consultation.
