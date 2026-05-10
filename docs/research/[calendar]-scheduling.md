# Automated Calendar & Scheduling Integration

## Title
Automated Calendar & Scheduling Integration

## Problem Statement
Small service-based businesses (e.g., consultants, tutors, salons) spend a disproportionate amount of time negotiating meeting times via email or text. This manual process is inefficient, prone to double-booking, and creates a high-friction experience for clients, leading to drop-offs. They need a way for clients to self-serve bookings based on real-time availability.

## Research Report
*   **Tool:** Google Calendar API, Microsoft Graph API (Outlook).
*   **Market Analysis:** Automated scheduling is a baseline expectation for modern service businesses.
*   **Competitor Analysis:** Calendly and Acuity Scheduling dominate this space. Building scheduling directly into OHC reduces the need for external subscriptions.
*   **Ease of Use:** Must provide a simple, shareable booking link. The owner simply connects their existing calendar and sets working hours.
*   **Pricing:** Core APIs are generally free or included in existing Google Workspace / Microsoft 365 subscriptions.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Straightforward OAuth integration.
    *   *Standalone:* Doable, but OAuth flows and incoming webhook notifications for calendar events require handling for local environments (e.g., polling or local tunneling).

## Design Doc
*   **User Journey:** The business owner connects their Google or Outlook calendar in OHC settings. They define their available hours and appointment types (e.g., "30-min consultation"). OHC generates a public booking page URL. Clients visit this page, see available slots, and book a time. OHC automatically creates the event in the owner's calendar and sends a confirmation to the client.
*   **Triggers:** Client booking action on the public page; changes in the underlying connected calendar (to block out busy times).
*   **Actions:**
    *   Read free/busy time from connected calendars.
    *   Create new calendar events.
    *   Generate and send confirmation notifications.
*   **Visuals:** A calendar connection settings panel. A clean, mobile-friendly public booking interface.

## Implementation Prompt
Create a calendar synchronization and scheduling feature that allows a business owner to connect their Google Calendar or Outlook account. The system must present a public-facing booking page where clients can schedule appointments based on the owner's real-time availability. Ensure robust handling of timezone conversions and prevent double-booking. The setup process for the business owner should be minimal and intuitive, abstracting away API complexities.

## Priority
P0

## Estimated Scope
Medium
