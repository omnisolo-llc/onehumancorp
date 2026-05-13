# Issue Brief: Automated Booking and Calendar Sync

**Category**: Calendar & Scheduling

## Problem Statement
Business owners manually schedule appointments, leading to double-bookings and time wasted in back-and-forth emails. They need a simple way for clients to book available slots.

## Research Report

### Tool Evaluations

**1. Google Calendar API**
- **Ease of Use for User**: Very high. Most small business owners already use Google Workspace or free Gmail. The OAuth flow is standard and trusted.
- **Pricing**: Free to use the API within generous quota limits.
- **Conflict Resolution**: The API allows us to fetch free/busy schedules directly, allowing OHC to build a native booking page that never double-books.
- **Mode Compatibility**: Cloud mode stores OAuth refresh tokens securely in Postgres. Standalone mode stores them in the encrypted SQLite file. Both can poll or use Google's push notifications.

**2. Calendly**
- **Ease of Use for User**: Extremely easy, but requires them to manage a separate SaaS tool outside of OHC.
- **Pricing**: $10-$15/user/month. This is a significant cost for a micro-business.
- **Integration**: We could just embed Calendly via an iframe, but this breaks the unified OHC experience and forces the user to pay for two tools.

**3. Cal.com**
- **Ease of Use for User**: High. It's an open-source alternative to Calendly.
- **Pricing**: We could self-host Cal.com infrastructure, but that adds massive operational overhead.
- **Integration**: They have a robust API, but again, building a native Google Calendar sync in OHC is cleaner.

**Summary Recommendation**: Build a native scheduling engine in OHC that syncs directly with Google Calendar via their API. This saves the user $15/month and keeps them entirely within the OHC ecosystem.


## Design Doc
Integrate Google Calendar API and Microsoft Graph API (Outlook). Create a booking page generator in OHC. Cloud mode syncs calendars securely in Postgres; Standalone uses local SQLite. Resolve timezone differences automatically.

## Implementation Prompt
Create a scheduling module where users can connect their Google Calendar and generate a public booking link. Show calendar events and available slots.

## Priority
P1

## Estimated Scope
Medium
