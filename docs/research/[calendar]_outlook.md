# Scout: Tool Integration Research [Q2]

## [Calendar] Issue Brief: Microsoft Outlook Integration

**Title**: Microsoft Outlook Calendar Sync for Service Bookings

**Problem Statement**:
Business owners who already use Office 365 for their work (like professional consultants, boutique owners, or tutors) find it frustrating to manage two separate calendars. They need their OHC service bookings to automatically appear in their Outlook calendar and for OHC to respect their existing Outlook appointments to prevent double-booking, all without the complexity of third-party tools like Calendly.

**Research Report**:
- **Tool**: Microsoft Graph API (Outlook Calendar).
- **Evaluation**: The Graph API is the official and most robust way to integrate with the Microsoft 365 ecosystem.
- **Ease of Use**: High. Users just click "Sign in with Microsoft" and grant calendar permissions.
- **Pricing**: Free for users with an existing Microsoft 365 / Outlook account.
- **Reputation**: Highly reliable and the gold standard for corporate and professional scheduling.
- **Cloud vs. Standalone**: Works in both. Standard OAuth flow.

**Design Doc**:
- User navigates to the Sales dashboard and selects "Sync Outlook Calendar."
- Uses standard OAuth 2.0 to grant OHC access to `Calendars.ReadWrite`.
- OHC's booking widget queries the Graph API for "Free/Busy" status to show available slots.
- When a booking is made, OHC pushes the event details (customer name, service, link) directly to the user's Outlook calendar.
- "The Manager" AI alerts the user if an Outlook event conflicts with a tentative OHC booking.

**Implementation Prompt**:
Build a Microsoft Outlook integration using the Graph API. Implement the OAuth flow and sync logic to fetch availability and create events. Ensure the booking widget dynamically updates based on the user's Outlook schedule.
- **Acceptance Criteria**: Merchant can connect Outlook account. OHC booking widget reflects Outlook "Busy" times. New OHC bookings appear in Outlook.
- **Priority**: P1
- **Estimated Scope**: Medium
