# Issue Brief: Frictionless Mobile-First Appointment Scheduling & Payments

## Problem Statement
Service providers like Leo (music tutor, 22) struggle immensely with coordinating times via text message. Existing scheduling tools like Calendly feel too corporate for micro-businesses and are often completely disconnected from their primary payment systems, leading to awkward 'Venmo me later' scenarios and high rates of unpaid no-shows.

## Research Report
A massive segment of the SMB market (tutors, handymen, consultants, barbers) sells time, not physical products. Integrating scheduling directly with payment capture ensures that no-shows are penalized and revenue is secured upfront. App Store reviews for dedicated booking apps frequently cite terrible mobile interfaces for the admin, making it hard to block out personal time or adjust schedules on the fly.

OHC needs a unified scheduling primitive that treats time as inventory and seamlessly couples it with the checkout flow.

## Design Doc
**High-Level Architecture & Entities:**
- `Service`: A bookable product type.
- `Availability`: Rules defining when the service can be booked.
- `Booking`: The transactional instance linking a Customer, Time Slot, and Order/Payment.
- Integrations: Calendar sync (Google Calendar, Apple Calendar via CalDAV/OAuth) to prevent double booking.

**Mobile UX Flow:**
1. **Admin Setup:** User defines a service ('1hr Piano Lesson, $50'). Connects Google Calendar via simple OAuth flow.
2. **Customer View:** Customer lands on OHC storefront, sees an elegant, mobile-optimized calendar.
3. **Checkout:** Customer selects a time, enters details, and pays via Stripe in a single, uninterrupted flow.
4. **Confirmation:** Both parties receive immediate calendar invites and SMS reminders.

**AI Agent Integration Points:**
- Agent can autonomously suggest optimal buffer times between appointments based on location data.
- Agent handles natural language rescheduling requests via DM.

## Implementation Prompt
Create a unified booking and payment flow designed specifically for service-based businesses. The system must allow an owner to define complex availability rules and require upfront payment or a partial deposit to confirm a time slot.

**Critical User Journey (CUJ):**
1. Customer views service page and selects a date/time.
2. System verifies real-time availability against connected calendars.
3. Customer completes checkout.
4. Booking record is created, calendar events are dispatched, and payment is captured.

**Acceptance Criteria:**
- A customer must be able to successfully book a time slot and complete a payment.
- The system must prevent double booking if a time slot is already occupied in the system.
- The admin interface must allow the business owner to easily block out specific dates or adjust standard hours from a mobile device.

## Priority
P1

## Estimated Scope
Large
