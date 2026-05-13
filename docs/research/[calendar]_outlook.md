# [Calendar & Scheduling] Outlook Calendar Integration

## Title
Outlook Calendar Sync for Professional Scheduling

## Problem Statement
Marcus the Consultant uses Outlook for his corporate clients. OHC currently only supports Google Calendar, forcing Marcus to manually copy appointments, leading to double bookings and lost credibility.

## Research Report
- **Strategy**: Integration with Microsoft Graph API for calendar events.
- **Advantages**: Opens up OHC to a massive B2B and enterprise-adjacent market segment.
- **Risks**: Microsoft Graph API can be complex. Token refresh and permission scopes need careful management.
- **Pricing**: Free with Microsoft 365 developer account; included in standard Microsoft 365 subscriptions.
- **Ease of Use**: Standard Microsoft OAuth flow. Very familiar to target users.
- **Compatibility**: Fully compatible with both Cloud and Standalone modes.

## Design Doc
- User clicks "Connect Outlook" in settings.
- Redirected to Microsoft login for authorization.
- OHC periodically syncs free/busy times to prevent scheduling conflicts in the OHC booking widget.
- OHC creates new events in the user's Outlook calendar when a booking is made.

## Implementation Prompt
Implement Microsoft Graph API integration for Calendar sync. Support OAuth connection, read free/busy status for availability checking, and write new events upon customer booking.

## Priority
P1

## Estimated Scope
Medium
