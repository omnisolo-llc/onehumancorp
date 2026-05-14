# Automated Scheduling via Cal.com

## Problem Statement
Coordinating appointment times with clients via email or text is frustrating and time-consuming. Business owners need a way to share a link and let clients book an available slot automatically without double-booking.

## Research Report
Cal.com is an open-source scheduling infrastructure that offers robust APIs and UI components. It is a strong alternative to Calendly, offering better developer experience and self-hosting options (crucial for Standalone mode).
*   **Ease of use (end user):** High. The user sets their working hours, and the tool handles the rest.
*   **Pricing:** Free for individuals, making it highly attractive for solo business owners.
*   **Reputation:** Highly regarded in the developer community; reliable and feature-rich.

## Design Doc
OHC will integrate a "Booking" module.
1.  **Trigger:** User configures their "Availability" in OHC settings.
2.  **Action:** OHC generates a unique, public-facing booking URL powered by Cal.com under the hood.
3.  **User Sees:** The owner shares their booking link. Clients visit the link, select a service and time, and book. The appointment automatically appears on the owner's OHC dashboard and synced personal calendar.

## Implementation Prompt
Implement an automated scheduling feature using Cal.com infrastructure.
*   Create an onboarding flow for the business owner to define their weekly availability (e.g., Mon-Fri 9 AM - 5 PM) and service durations.
*   Generate a customer-facing booking page where clients can select a date and time.
*   Ensure booked appointments are visible in a new "Schedule" or "Calendar" view on the owner's dashboard.
*   Acceptance Criteria: A business owner can set their hours, a customer can book a slot via the public link, and the appointment appears on the owner's dashboard.

## Priority
P1

## Estimated Scope
Medium
