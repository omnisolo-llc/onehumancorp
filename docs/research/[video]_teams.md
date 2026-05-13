# Title: Microsoft Teams Integration for Embedded Consultations

## Problem Statement
Consultants, tutors, and B2B service providers need to meet with clients via video. Manually creating a meeting link and emailing it to the client is tedious. They need a system that automatically generates a video link when a booking is made and attaches it to the calendar invite. While Zoom is popular, many B2B clients exclusively use Microsoft Teams.

## Research Report
**Market Analysis & Pain Points:**
- **Friction:** Manually creating Teams links for every appointment leads to errors and lost time.
- **Competitors:** Zoom, Google Meet. Teams is critical for B2B-focused SMBs.
- **Microsoft Graph API:** Allows creating online meetings and fetching join URLs.
- **Reputation & Ease of Use:** The OAuth flow can be complex (requires Azure AD app registration), which is a hurdle we must abstract away from the user.
- **Pricing:** Included with Microsoft 365 business plans.

**Key Advantages:**
- Essential for B2B service providers.
- Integrates deeply with Outlook calendars.

**Integration Risks:**
- Microsoft Graph API is notoriously complex and permissions can be confusing for end-users to grant.
- Token expiration and refresh logic requires robust handling.

**Environment Support:**
- **Cloud:** Full support.
- **Standalone:** Full support.

## Design Doc
**Trigger:**
User goes to "Integrations" -> "Video" and connects Microsoft Teams via OAuth.

**Action:**
When a new appointment is scheduled in OHC, the system calls the Microsoft Graph API to generate a Teams meeting link and injects it into the appointment details and confirmation emails.

**User View:**
The business owner simply sees a "Microsoft Teams" option when setting up their appointment types. When a client books, the owner sees the "Join Meeting" button directly in their OHC dashboard, and the client receives the link automatically. No manual copy-pasting is required.

## Implementation Prompt
Integrate Microsoft Teams for automatic video link generation.
- Build an OAuth flow targeting the Microsoft Graph API to authorize meeting creation.
- Update the booking engine to automatically generate a Teams link when an appointment is created (if Teams is selected as the location).
- Display a prominent "Join Video Call" button on the appointment details view for both the business owner and the client portal.
- (Do not prescribe the specific Graph API endpoints in the database; design the system to handle the OAuth and link injection smoothly.)

## Priority
P2

## Estimated Scope
Medium
