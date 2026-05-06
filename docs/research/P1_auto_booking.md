# [Product Gap] Integrated Booking & Scheduling (The Manager)

## Title
Implement Native Booking and Scheduling for Service Businesses

## Problem Statement
Service-based solopreneurs (like Carlos the handyman and Leo the tutor) lose leads because they have no automated way for clients to book their time. Manual back-and-forth via text/email is slow and unprofessional. Existing tools (Calendly, Acuity) are separate subscriptions they have to pay for and integrate manually.

## Research Report
*   **Competitor Landscape:** Squarespace Acuity is a paid add-on ($16/mo). Wix Bookings is included in higher tiers. Shopify requires third-party apps for booking.
*   **User Pain Point Data:** High demand from non-retail SMBs. "I just need a simple calendar link" is a frequent request in `r/sweatystartup` and `r/smallbusiness`.
*   **OHC Advantage:** Native integration means zero setup for the user. The "Manager" agent can automatically manage the calendar and send reminders, creating a seamless experience.

## Design Doc
*   **Entities:** `Service` (type of booking), `Appointment`, `Availability`, `CalendarIntegration` (Google Calendar, Cal.com).
*   **Architecture:**
    *   Integration with Cal.com (open-source scheduling infrastructure) to handle calendar sync and time slot logic.
    *   `Service` entity linked to `Product` (a service is a type of product).
    *   Booking widget embedded in the tenant's public storefront.
*   **UI Wireframe/Flow (375px first):**
    *   **Screen 1: Services List.** User sees their available services (e.g., "1-Hour Consultation").
    *   **Screen 2: Add Service.** User inputs name, duration, price, and connects their Google Calendar.
    *   **Screen 3: Appointments View.** A simple list of upcoming bookings.
    *   **Public Storefront:** A clean calendar view for the end-customer to select a date/time.

## Implementation Prompt
Build a native booking system leveraging Cal.com for calendar synchronization. Allow users to define "Service" products with specific durations and prices. Provide a public-facing booking widget on the generated storefronts where customers can select available time slots. Ensure the 'Manager' agent automatically sends confirmation and reminder emails/SMS to both the business owner and the customer.

## Priority
P1

## Estimated Scope
Medium
