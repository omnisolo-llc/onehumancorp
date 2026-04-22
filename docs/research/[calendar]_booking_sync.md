# 📅 Calendar & Scheduling: Booking Sync

## Title
Universal Calendar & Booking Sync Integration

## Problem Statement
Service providers like Leo (The Music Tutor) and Carlos (The Handyman) rely on scheduling to run their businesses. They often use personal Google or Apple Calendars. Without a unified booking system, they risk double-booking, have to manually negotiate times with clients, and struggle to manage deposits. They need a simple booking page that automatically syncs with their existing calendars and handles payment collection upfront.

## Research Report
- **Goal**: Evaluate tools that provide calendar synchronization (Google, Outlook, Apple) and booking availability logic.
- **Tools Evaluated**:
    - **Nylas**: Excellent API for calendar sync across almost all providers. Handles complex recurring events well. However, pricing can be steep for small businesses.
    - **Cronofy**: Strong enterprise calendar integration, good privacy controls. Focused more on B2B.
    - **Cal.com (Open Source)**: Highly customizable, robust API, and specifically built for scheduling. Supports webhooks, routing, and payments. Can be self-hosted (perfect for Standalone mode) or consumed via API.
- **Recommendation**: Integrate with **Cal.com** (specifically its infrastructure API). It provides the exact primitive needed: connecting a user's calendar and determining free/busy slots. It aligns with our need for both Cloud (managed) and Standalone (self-hosted) deployments.
- **User Impact**: Leo can connect his Google Calendar. Customers see his available slots on his OHC storefront, book a guitar lesson, and pay via Stripe. The event appears on Leo's calendar, and the OHC "Operations" agent automatically sends a confirmation email.

## Design Doc
- **Component**: `BookingAgent`
- **Responsibilities**:
    - Handle OAuth flow for users to connect their external calendars.
    - Fetch free/busy times from the provider and calculate available booking slots based on business rules (e.g., buffer times).
    - Handle the booking transaction: reserve slot -> process payment -> confirm slot.
    - Sync confirmed OHC bookings back to the user's external calendar.
- **Integration Point**: The OHC Frontend booking component will query the `BookingAgent` for available slots and submit booking requests.

## Implementation Prompt
Implement the Universal Calendar Sync integration. Create a service that manages calendar OAuth connections (Google, Outlook). Implement an endpoint that calculates available booking slots by overlaying external busy times with the user's defined working hours. Implement the booking creation flow that writes the event back to the external calendar. Ensure it works seamlessly for both Cloud and Standalone users.

## Priority
P0

## Estimated Scope
Medium
