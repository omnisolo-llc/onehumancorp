# Title: Automate Customer Bookings and Calendar Sync with Calendly

## Problem Statement
Small business owners waste hours playing email or text "ping-pong" with customers to find a time to meet, whether it's for a consultation, a lesson, or a service appointment. Double-bookings are common, and manually managing time zones for online meetings is confusing. They need a way to let customers pick available times themselves, without double-booking over existing appointments.

## Research Report
Calendly is the industry standard for automated scheduling.
- **Ease of Use**: Very high. Users simply connect their Google or Outlook calendar with one click, set their working hours, and share a link. The customer sees a clean, simple booking page.
- **Pricing**: Has a very generous "Always Free" tier for basic 1-on-1 meetings, which is perfect for most small businesses. Paid tiers (starting at $10/mo) add features like payment collection upon booking.
- **Reputation**: The most recognizable scheduling tool. Extremely reliable conflict resolution and timezone handling.
- **Comparison**: While Cal.com is a strong open-source alternative, Calendly remains more recognizable and often easier for non-technical users to conceptualize.
- **Cloud vs Standalone**: The frontend booking page is hosted by Calendly. Webhooks returning booking data to OHC work natively in Cloud mode but require tunnel forwarding in Standalone mode.

## Design Doc
- **Triggers & Actions**: The business owner connects their calendar. OHC displays their personalized booking link. When a customer uses the link to book a time, the event is automatically added to the owner's calendar, and a notification is shown in OHC. If the event is online, a meeting link (Zoom/Meet) is auto-generated.
- **User Experience**: In "App Settings", the user clicks "Connect Calendar". They are then given a "My Booking Link" which they can copy to paste into their Instagram bio or emails. OHC will also show a "Upcoming Appointments" widget on the main dashboard pulling data from these bookings.

## Implementation Prompt
Integrate a scheduling system into OHC to allow automated bookings.
- **User-Facing Outcome**: The business owner can generate a "Booking Link" to share with clients. Clients visiting the link can select from available time slots. Once booked, the appointment appears on the owner's dashboard and their personal calendar.
- **Acceptance Criteria**:
  - The business owner can authenticate their calendar (Google/Outlook) with one click.
  - The system provides a shareable booking URL.
  - New bookings trigger a notification within OHC.
  - Time zones are automatically handled so both the owner and the customer see the correct local time.

## Priority
P0

## Estimated Scope
Medium
