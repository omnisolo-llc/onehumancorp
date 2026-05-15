### Title
`[video]embedded_consultations`: Implement Auto-Generated Google Meet Links

### Problem Statement
Virtual services (tutoring, consulting) are booming. Currently, owners have to manually create a Zoom or Meet link and email it to the client after they book. This manual step often leads to forgotten links, confused clients, and lost revenue.

### Research Report
- **Tool**: Google Meet API (via Google Workspace/Calendar integration)
- **Pros**: Free, ubiquitous, requires no software installation for the client.
- **Cons**: Requires the business owner to have a Google account (very common, though).
- **Reputation**: Highly reliable and trusted by consumers.
- **Pricing**: Free with standard Google accounts.
- **Ease of Use for Non-Technical Users**: Completely invisible. The link just appears on the booking confirmation.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: A customer books a service defined as "Virtual" or "Online".
- **Action**: The OHC API server requests a meeting link via the Google API (often bundled with the Calendar event creation) and saves the URL.
- **User View**: The booking confirmation page and email automatically display the "Join Meeting" button.

### Implementation Prompt
Enhance the booking flow to automatically generate Google Meet video conferencing links for virtual appointments. This should be tightly coupled with the Google Calendar integration. Ensure the meeting link is prominently displayed in the customer's confirmation email, the business owner's schedule view, and accessible via the API for any custom frontend clients.

### Priority
P2

### Estimated Scope
Small
