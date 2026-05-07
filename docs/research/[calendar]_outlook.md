# Scout: Tool Integration Research

## [Calendar] Issue Brief: Outlook Calendar Integration
**Title**: Native Outlook Calendar Sync for Scheduling
**Problem Statement**:
Leo (Music Tutor) and many professional service providers use Microsoft Outlook/Office 365 for their primary calendar. Currently, OHC only syncs with Google Calendar, forcing Leo to manually cross-reference his schedules, which leads to double-booking errors.

**Research Report**:
- **Tool**: Microsoft Graph API (Calendar).
- **Evaluation**:
  - **Ease of Use**: High. Standard Microsoft OAuth flow.
  - **Pricing**: Free for standard Outlook users; included in Microsoft 365 subscriptions.
  - **Reputation**: Essential for the "Professional Services" persona.
  - **Cloud vs. Standalone**: Works in both.
- **Key Advantages**: Expands OHC's reach to the significant portion of SMBs that prefer the Microsoft ecosystem over Google.
- **Risks**: Complex recurring event logic in the Graph API.

**Design Doc**:
- **User Flow**: User goes to "Settings > Calendar" and selects "Connect Outlook".
- **Integration**: OHC uses Microsoft Graph API to fetch "Busy" blocks.
- **User Experience**: The OHC storefront booking widget automatically hides slots that are occupied in the user's Outlook calendar. New OHC bookings are pushed to Outlook automatically.

**Implementation Prompt**:
Implement Microsoft Graph API integration to support Outlook Calendar synchronization. Users must be able to authenticate via OAuth. The system should fetch free/busy status to inform the OHC booking widget and create events in Outlook when a customer books a service.

**Priority**: P1
**Estimated Scope**: Medium
