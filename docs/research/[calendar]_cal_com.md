# Cal.com API - Automated Scheduling

## Problem Statement
Scheduling appointments, consultations, or service calls involves endless back-and-forth emails or texts to find a suitable time, frustrating both the business owner and the customer.

## Research Report
Cal.com is an open-source scheduling infrastructure platform.
- **Ease of Use for SMBs**: High. Business owner simply connects their Google/Outlook calendar and defines working hours.
- **Pricing**: Open-source and highly affordable.
- **Reputation**: Rapidly growing open-source alternative to Calendly. Developer-friendly and highly customizable.
- **Competitive Analysis**: More flexible and affordable than Calendly, especially for API-driven integration.

## Design Doc
**Trigger**: Business owner navigates to "Scheduling" and connects their Google or Outlook calendar.
**Actions**:
- OHC uses Cal.com API to create an event type and a booking link.
- Customers visit the booking link (embedded in OHC storefront) and select a time.
- Cal.com handles calendar conflict resolution and timezones.
**User Experience**: A clean booking page where customers can pick a time slot.

## Implementation Prompt
**User-facing Outcome**: A business owner can connect their calendar to OHC and automatically generate a booking page for their services.
**Acceptance Criteria**:
- User can connect Google/Outlook calendar.
- User can define working hours and event durations.
- A public booking link is generated and works correctly.

## Priority
P1 (High)

## Estimated Scope
Medium
