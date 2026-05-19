# [Booking] Cal.com Scheduling Integration

## Title
Native Cal.com Integration for Automated Booking & Scheduling

## Problem Statement
Leo (Independent Contractor) and Maya (Home Baker) spend hours going back and forth with clients via text and email to find suitable times for site estimates or cake tastings. Traditional booking involves playing phone tag or manually updating calendars, leading to double-bookings and lost revenue. They need a simple, white-labeled way to let customers book appointments directly on their OHC storefront without technical setup, while syncing seamlessly with their existing personal calendars.

## Research Report
- **Strategy**: Direct API and Webhook integration with Cal.com (a powerful open-source Calendly alternative).
- **Target Persona**: Leo (Contractor), Maya (Home Baker), and Priya (Boutique Owner for styling sessions).
- **Advantages**: Cal.com is developer-friendly, open-source, and offers a highly generous free tier for individuals (essential for solo SMBs). It handles complex scheduling logic, timezone math, and multi-calendar conflict resolution (Google Calendar, Outlook, Apple Calendar) natively. It also supports payment collection upon booking (via Stripe).
- **Risks**: Proper mapping of Cal.com event types to OHC service catalogs. Webhook delivery guarantees need robust handling in OHC to avoid missing new bookings.
- **Pricing**: Free tier for individuals covers unlimited bookings and basic integrations. Pro tier is $12/month for advanced features.
- **Ease of Use**: High. The business owner authorizes their calendar once. OHC automatically generates the booking widget, so the owner doesn't need to manually embed iframe code.
- **Compatibility**:
    - *Cloud Mode*: Integrates seamlessly via their managed SaaS API and webhooks.
    - *Standalone Mode*: Cal.com can be self-hosted, allowing completely private scheduling for local deployments.

## Design Doc
- **Integration with OHC**:
    - **Authentication**: User connects Cal.com via OAuth within the OHC "Operations" dashboard.
    - **Widget Embedding**: The OHC storefront builder provides a drag-and-drop "Booking Block" that automatically uses the Cal.com Embed API to show the user's availability on their public site.
    - **Event Syncing**: OHC registers webhook listeners for `booking.created`, `booking.rescheduled`, and `booking.cancelled`.
    - **AI Trigger**: When a new booking is created, the webhook triggers the "Ambassador" AI Agent to draft a personalized welcome/confirmation message (e.g., via WhatsApp) and updates the unified OHC timeline.
- **User View**: A visual "Calendar" tab in OHC that displays upcoming appointments without needing to log into a separate tool. On the storefront, customers see a clean, branded date/time picker.

## Implementation Prompt
Build a native integration for Cal.com. Implement the OAuth 2.0 flow for users to link their Cal.com accounts. Create backend endpoints to handle incoming webhooks for booking lifecycle events and store these records in the OHC database to populate the user's dashboard. Develop a Next.js storefront block component that accepts a Cal.com event link and renders the embed widget. Ensure the internal AI agent receives an event notification upon new bookings to trigger follow-up workflows. Do not include hardcoded API endpoints or specific database table definitions; design the system to fit seamlessly into the existing Next.js UI and Rust backend.

## Priority
P1

## Estimated Scope
Medium
