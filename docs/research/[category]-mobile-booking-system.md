# Mobile-First Unified Booking System
## Title
Unified Mobile-First Booking and Scheduling System

## Problem Statement
Service-based businesses, such as Carlos (a handyman) or Leo (a music tutor), rely on manual text messages or disjointed web-based tools (like Calendly) to schedule appointments. They lack a unified system that handles scheduling, deposits, and automated reminders entirely from their phone. Existing solutions are either retail-focused (Shopify) or feel clunky on mobile (Wix Bookings). They need a simple, reliable booking system that guarantees they never double-book or miss a lead.

## Research Report
*   **Wix Bookings:** Capable, but the mobile management experience is reported as clunky by users. The setup process is desktop-heavy.
*   **Shopify:** Highly retail and product-focused. Booking requires expensive 3rd-party apps.
*   **Squarespace:** Acuity Scheduling is powerful but operates almost as a separate product, adding complexity.
*   **User Pain Points:** App store reviews for booking apps frequently complain about syncing issues with personal calendars and the difficulty of managing appointments on the go.
*   **OHC Advantage:** Building a native, mobile-first booking module integrated directly into the core OHC platform and SIPDB ensures seamless syncing and zero friction for the business owner.

## Design Doc
**High-level Architecture:**
*   **Data Model:** `Bookings` entity tied to `tenant_id` (RLS enforced), with `service_type`, `datetime`, and `status`. Syncable entity with `version` and `updated_at`.
*   **Integration:** Calendar sync (Google/Apple) for the owner.
*   **Automations:** Automated SMS/Email reminders for the client (reducing no-shows).

**UI Flow (Mobile First - 375px):**
1.  **Dashboard:** The owner's main view is a simplified, scrollable daily agenda.
2.  **Creation (Simple Mode):** Tapping '+' opens a quick form: "Who?", "What?", "When?".
3.  **Public Booking Page:** A mobile-optimized, fast-loading page for clients to select time slots.
4.  **Advanced Mode:** Hidden by default. Allows setting buffer times, deposit requirements, and custom reminder schedules.

## Implementation Prompt
Implement a unified booking and scheduling system designed primarily for mobile usage by service-based small businesses.
**Critical User Journey (CUJ):**
1. The business owner opens the OHC app and defines their availability and service type (e.g., "1-hour guitar lesson, $50").
2. The owner shares their OHC link on social media.
3. A client taps the link, views available slots on a mobile-friendly page, and books an appointment.
4. The owner receives an instant push notification, and the appointment appears on their unified agenda.

**Acceptance Criteria:**
* Must be fully functional and optimized for 375px viewports (Mobile-First).
* Must adhere to the Progressive Disclosure Pattern, keeping buffer times and advanced calendar settings hidden in 'Advanced Mode'.
* Entities must be offline-syncable (include `updated_at` and `version` columns).
* Must strictly enforce tenant isolation.

## Priority
P0

## Estimated Scope
Large
