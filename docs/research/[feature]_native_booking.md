# Issue Brief: Native Zero-Config Booking System

## Problem Statement
Service-based small business owners (like Carlos the handyman and Leo the music tutor) lose money because they lack a simple, integrated way to let customers book their time. They are forced to rely on expensive third-party plugins (like Calendly or Acuity) or manage chaotic schedules via text messages and phone calls, leading to double-bookings and missed opportunities.

## Research Report
Our analysis reveals a critical feature gap: while Wix offers "Wix Bookings," it requires manual configuration, and Shopify requires paid plugins for simple service scheduling. A major pain point (#5 on our list) is the cost and complexity of these add-ons. Over 20% of SMB complaints center on missing leads because scheduling is too difficult for the customer.

## Design Doc
- **Key Entities**: `Service`, `Availability`, `Booking`, `CalendarEvent`.
- **Integration Points**: Native integration with the AI Receptionist and the user's mobile push notifications.
- **Mobile UX Flow**:
  1. User specifies their working hours and service duration in natural language (e.g., "I work 9-5 and jobs take 2 hours").
  2. The system automatically provisions a booking page.
  3. Customers can view availability and book directly; the user receives an instant mobile notification.

## Implementation Prompt
**User-Facing Outcome**: A fully native, zero-configuration booking system that requires no external plugins. It generates a public scheduling link automatically based on simple availability inputs.
**Critical User Journey**:
- User inputs their general availability.
- A customer visits the user's OHC site and selects an open time slot.
- The system blocks the time on the calendar and alerts the user immediately.
**Acceptance Criteria**:
- Must generate a bookable calendar without manual plugin installation.
- Must prevent double-booking.
- Must notify the business owner immediately upon a new booking.

## Priority
P1

## Estimated Scope
Medium
