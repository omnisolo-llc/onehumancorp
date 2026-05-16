# Title: Cal.com Calendar & Scheduling Integration

## Problem Statement
Small business owners often struggle with managing appointments and scheduling. Relying on back-and-forth emails or phone calls is time-consuming and leads to scheduling conflicts, missed meetings, and lost revenue. For a non-technical founder, they need a simple way to share their availability, let clients book automatically, and have it sync directly to their calendar without needing to manage complex software.

## Research Report
*   **Overview**: Cal.com is a leading open-source scheduling infrastructure platform. It allows users to connect their calendars (Google, Outlook, etc.) and generate booking links.
*   **Ease of Use**: It is very easy to use for non-technical users. They just connect their calendar, set their available hours, and share the link. It includes a user-friendly app interface.
*   **Reputation**: Strong reputation as an open-source alternative to Calendly, trusted by many fast-growing companies and individuals.
*   **Pricing**:
    *   **Individuals**: Free tier available (unlimited event types, calendar connections, and basic integrations).
    *   **Teams**: $12 per user/month (round-robin, collective events, team analytics).
    *   **Organizations**: $28 per user/month (SAML SSO, routing, custom domains).
*   **Environment (Cloud vs Standalone)**: As an open-source tool, Cal.com supports both Cloud (via their managed service) and Standalone environments. They offer Docker self-hosting instructions, meaning we can integrate it seamlessly regardless of the deployment mode OHC is running in.
*   **AI Integration**: Cal.com has introduced "Cal.ai," an AI-powered scheduling assistant, which aligns well with future AI agent workflows.

## Design Doc
*   **Trigger**: The business owner enables the Cal.com integration from the OHC dashboard and authenticates via OAuth.
*   **Action**: OHC creates a webhook subscription with Cal.com. When a new booking is made, rescheduled, or cancelled via the owner's Cal.com link, the event data is pushed to OHC. OHC will then log this activity in the owner's unified inbox or calendar view.
*   **User Interface**: A simple setup screen in OHC displaying a "Connect Cal.com" button. Once connected, a dashboard widget will display upcoming appointments and a quick-copy button for their booking link.

## Implementation Prompt
Implement a Cal.com integration feature where users can connect their Cal.com account to OHC. The outcome should allow users to see their upcoming appointments within the OHC interface and receive notifications for new bookings. The integration must securely handle OAuth authentication, store tokens appropriately, and gracefully handle webhook events from Cal.com. Ensure the UI clearly shows the connection status and provides a way to disconnect.

## Priority
P1

## Estimated Scope
Medium
