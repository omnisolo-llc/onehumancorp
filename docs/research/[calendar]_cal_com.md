# Integration Issue Brief: Calendar & Scheduling (Cal.com)

## Title
Automated Calendar & Scheduling Integration: Cal.com

## Problem Statement
Small business owners, especially those offering services (consultants, tutors, salons), waste significant time on back-and-forth emails or texts trying to find a suitable meeting time. They need a simple, professional way for clients to book time on their calendar that automatically handles timezones, prevents double-booking, and generates meeting links.

## Research Report
*   **Tool Evaluated**: Cal.com
*   **Ease of Use**: Extremely user-friendly and modern. It offers a clean booking page that business owners can share via a link or embed on their site.
*   **Market Position & Reputation**: A strong open-source alternative to Calendly. It is highly regarded by developers for its API and by users for its generous free tier and clean UI.
*   **Pricing**:
    *   **Individuals**: Free forever (unlimited events, 1 user, email/SMS notifications).
    *   **Teams**: $12/user/month (round-robin, routing).
    *   **Organizations**: $28/user/month.
*   **Cloud vs. Standalone Compatibility**: As an open-source tool, Cal.com can be self-hosted or used via their cloud SaaS. This makes it a perfect fit for OHC: Cloud users can connect to Cal.com's SaaS, while Standalone users could technically connect to a self-hosted instance if they prefer, or use the SaaS API.

## Design Doc
*   **Integration Trigger**: The user connects their Cal.com account in OHC using an API key or OAuth.
*   **Action Flow**:
    1.  OHC fetches the user's available "Event Types" from Cal.com.
    2.  When a booking is made via Cal.com, a webhook notifies OHC.
    3.  OHC creates a calendar event internally and can optionally trigger automated reminder flows.
*   **User Experience**: The business owner can easily insert their booking links into emails or SMS messages sent from OHC. Their OHC dashboard displays upcoming appointments synced directly from Cal.com.

## Implementation Prompt
Integrate Cal.com scheduling into the OHC platform. Provide a settings page for the user to authenticate with Cal.com. Once connected, OHC should display a widget of upcoming bookings on the dashboard. Add a feature in the OHC messaging composer (Email/SMS) that allows the user to quickly insert one of their Cal.com event links with a single click. Ensure webhooks are configured so that OHC's internal calendar syncs immediately when a new booking is created or canceled.

## Priority
P0

## Estimated Scope
Medium
