# Mobile-First Zero-Setup Booking System

## Problem Statement
Service-based small business owners like Carlos (handyman) and Leo (music tutor) rely on manual scheduling via text, email, or DMs. They lack a streamlined booking system because existing tools like Calendly are disconnected from their payment and CRM systems, and are difficult to manage on the go. They lose leads when they are busy and cannot respond immediately.

## Research Report
- **Competitor Audit:** Square Appointments is strong but heavily tied to their POS ecosystem. Calendly is popular but is just scheduling, not a full business platform. Wix Bookings is clunky on mobile.
- **Pain Points:** "Booking ping pong" (endless back-and-forth messages to find a time) is a top complaint in service business forums. Missing a call often means missing a job.
- **Market Sizing:** Service businesses form a massive segment of the non-employer small business market. Acquiring them early establishes a strong beachhead.

## Design Doc
### High-Level Architecture
- **Entity Types:** `Service`, `Availability`, `Booking`, `Customer`.
- **Key Relationships:** A `Booking` connects a `Customer`, `Service`, and specific time slot based on `Availability`.
- **Integration Points:** Payments (Stripe/Mercado Pago), SMS notifications (Twilio), Calendar Sync (Google/Apple).

### UI Wireframes / Screen Flow
- **Mobile UX (375px first):**
  1. Business owner defines a service ("1 Hour Guitar Lesson - $50") and links their personal calendar.
  2. The system generates a clean, single-page booking link.
  3. Customer clicks link via Instagram bio, selects an open time, and pays a deposit via Apple Pay/Google Pay.
  4. Both parties receive SMS confirmations. The appointment appears on the owner's calendar.

### AI Agent Integration
- An AI scheduling assistant auto-replies to DMs or emails with the booking link, eliminating "booking ping pong".
- The AI can automatically send reminders and follow-up requests for reviews.

## Implementation Prompt
Implement a unified booking engine that allows service providers to create bookable services directly from a mobile interface. The engine must support calendar synchronization, deposit handling, and automated SMS reminders. The Critical User Journey involves a service owner creating a new service and a customer successfully booking and paying a deposit through the generated link, all designed with a mobile-first philosophy.

## Priority
P1

## Estimated Scope
Medium
