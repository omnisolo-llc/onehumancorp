# Integrate Calendly for Automated Scheduling

## Problem Statement
Small business owners, especially consultants and service providers, waste a lot of time going back and forth via email to find a suitable meeting time. They need a simple way to let clients book appointments directly into their calendar without creating double bookings or requiring manual confirmation.

## Research Report
*   **Tool:** Calendly (or similar scheduling APIs like Cal.com)
*   **Problem Solved:** Automates scheduling by syncing with the owner's Google/Outlook calendar and providing a public booking link.
*   **Ease of Use:** Extremely high. Owners just share a link, and clients pick an available slot.
*   **Pricing:** Free basic tier; Premium starts at $10/month.
*   **Reputation:** Industry standard, highly trusted, very reliable calendar sync.
*   **Environment:** Works well in both Cloud and Standalone modes (relies on outbound API calls to check availability/book slots).
*   **Advantages:** Eliminates scheduling friction; prevents double booking by checking real-time availability; automatic timezone conversion for clients.
*   **Risks:** Calendar sync issues if the underlying Google/Microsoft token expires; users might struggle to configure complex availability rules initially.

## Design Doc
1.  **Trigger:** A "Setup Scheduling" card on the main dashboard.
2.  **Action:** User connects their primary calendar (Google/Outlook) and sets basic working hours (e.g., 9 AM - 5 PM). OHC generates a unique booking link.
3.  **User Interface:** The business owner sees an "Appointments" view showing upcoming bookings. They can easily copy their public booking link to share in emails or social media. When a client books, the appointment appears automatically in the owner's personal calendar and the OHC dashboard.
4.  **Notifications:** The system sends automated confirmation and reminder emails to the client.

## Implementation Prompt
Create an automated scheduling feature within OHC. Allow the business owner to connect their Google or Outlook calendar and define their working hours. Generate a public, shareable booking page for the business where clients can select an available time slot. The system must automatically check the owner's calendar for conflicts before showing available times. Once a client books, the event must be automatically added to the owner's calendar, and a confirmation must be sent to the client. Include timezone handling so the client sees availability in their local time.

## Priority
P1

## Estimated Scope
Medium
