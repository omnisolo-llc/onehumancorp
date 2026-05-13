# [feature] Native Service Booking & Calendar

## Problem Statement
Service providers like Leo (music tutor) and Carlos (handyman) cannot use traditional e-commerce platforms because they sell time, not physical goods. They are forced to duct-tape together a website builder (Wix) with a separate booking tool (Calendly), leading to double-bookings and a disjointed customer experience.

## Research Report
- **Validation:** High density of service workers in the SMB space (35% of TAM) are underserved by Shopify.
- **Competitor Landscape:**
  - *Shopify:* Requires paid 3rd party apps (e.g., BookThatApp) which are complex to configure.
  - *Squarespace:* Acquired Acuity, good integration but high overall cost.
- **Opportunity:** Treat "Time slots" as a native product type alongside physical goods.

## Design Doc
### Architecture High-Level
- **Entities:** `Service`, `Provider`, `AvailabilitySchedule`, `Booking`, `TimeSlot`.
- **Integration Points:** Calendar sync (Google Calendar/iCal) to prevent double booking.
- **Core Engine:** A time-slot generation algorithm that calculates available intervals based on provider schedules, existing bookings, and buffer times.

### UX Wireframes (Mobile First - 375px)
- **Customer View:** Taps "Book Now" -> Sees a clean date picker -> Selects an available time block -> Enters details & pays deposit.
- **Owner View:** A "Calendar" tab showing upcoming appointments. Push notifications for new bookings. Ability to block out personal time easily.

## Implementation Prompt
**User-Facing Outcome:** A handyman can list "1 Hour Consultation" as a service. Customers can see real-time availability and book a slot directly on the OHC-powered site, instantly syncing to the owner's mobile calendar.

**Critical User Journey:**
1. Owner creates a "Service" product, setting duration (e.g., 60 mins) and availability (Mon-Fri 9-5).
2. Customer visits site, selects an open slot, and completes checkout.
3. Owner receives a booking confirmation and the slot is removed from availability.

**Acceptance Criteria:**
- Core booking engine handling availability math.
- UI components for calendar selection.
- Prevention of double-booking within the OHC system.

## Priority
P1

## Estimated Scope
Large
