# Title: Automated Booking & Calendar Sync

## Problem Statement
Service-based small business owners spend too much time going back and forth with clients to find a meeting time. Double bookings happen frequently because personal and business calendars aren't synced. They need a way to let clients book available slots automatically.

## Research Report
- **Tool Evaluated**: Cal.com
- **Persona Value**: High. Eliminates scheduling friction for service providers like Leo (Music Tutor).
- **Advantages**: Open-source, highly customizable, white-label API. Embeds seamlessly.
- **Risks**: Reliance on external scheduling logic.
- **Pricing**: Team plans available; open-source core.
- **Cloud vs Standalone**: Cloud works easily. Standalone requires managing OAuth tokens locally.

## Design Doc
- **Integration Trigger**: User sets working hours and connects Google/Outlook calendar.
- **Action**: OHC generates a public booking link via Cal.com. Bookings create events on the owner's calendar and block time in OHC.
- **User Interface**: "Availability" settings page and a public booking page for clients.

## Implementation Prompt
Build a scheduling feature using Cal.com that allows users to set weekly availability and connect a third-party calendar. Generate a shareable booking link. When a client books, the event must appear on the connected calendar and block out time.
- **Acceptance Criteria**: User sets hours and connects calendar. Test booking successfully blocks out that time on the connected calendar.

## Priority
P0

## Estimated Scope
Medium
