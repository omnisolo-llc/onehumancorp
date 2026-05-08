# [Calendar] Two-Way Sync and Scheduling

## Problem Statement
Small business owners often double-book themselves because their work schedule isn't synced with their personal calendar, or they spend too much time going back and forth with clients trying to find a suitable meeting time.

## Research Report
**Tools Evaluated:** Google Calendar API, Microsoft Graph API (Outlook)

*   **Ease of Use:** High for users. They just click "Connect Google Calendar" and authorize the app. No technical knowledge needed.
*   **Pricing:** The APIs themselves are generally free within standard usage limits (quotas apply but are usually sufficient for small businesses).
*   **Reputation:** These are the dominant calendar platforms globally.

## Design Doc
**Trigger:** User wants to set up a booking page or sync their availability.
**Action:** User connects their calendar provider via OAuth.
**User Sees:** A calendar view inside OHC showing their existing events. They can generate a public booking link where clients can only select time slots that are open on their connected calendar. Conflicts are automatically handled, and timezone differences are translated for the client.

## Implementation Prompt
Build a calendar integration and scheduling feature. The user needs a simple OAuth flow to connect Google Calendar or Outlook. Once connected, provide a UI to define "working hours". Generate a customizable booking page link that the business owner can share, which automatically checks for conflicts against their connected calendar before allowing a client to book.

## Priority
P0

## Estimated Scope
Medium

## Mode Compatibility
*   **Cloud:** Fully supported.
*   **Standalone:** Fully supported. The OAuth redirect can be handled locally or through a relay, and the local OHC instance can directly poll or receive updates from the calendar APIs.
