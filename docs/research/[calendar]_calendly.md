# Native Integration of Calendly for Appointment Scheduling

## Title
Native Integration of Calendly for Appointment Scheduling

## Problem Statement
Service-based small business owners (like tutors, consultants, or therapists) waste significant time in back-and-forth emails trying to find a suitable time for meetings. They need a simple, self-serve booking link that respects their availability and automatically creates calendar events, directly embedded in their OHC website.

## Research Report
- **Strategy**: Native API integration with Calendly for generating booking pages and synchronizing availability.
- **Target Persona**: Service-based professionals, consultants, tutors, and local businesses taking appointments.
- **Advantages**: Calendly is the standard for scheduling. Integrating it avoids building a complex calendar conflict resolution engine from scratch. High brand trust among end-users.
- **Risks**: Free tier of Calendly is limited (only one active event type). Users might need to upgrade their Calendly account for full functionality.
- **Pricing**: Basic is free (1 calendar, 1 event type). Standard tier is $10/mo.
- **Compatibility**: Compatible with both Cloud and Standalone modes via user-provided OAuth or API keys.

## Design Doc
- User navigates to "Scheduling" in OHC settings and connects their Calendly account via OAuth.
- OHC imports the user's active event types.
- The user can select which event types to feature on their OHC-generated storefront or share via direct links.
- When a customer books a slot, Calendly handles the conflict resolution and calendar syncing (Google Calendar, Outlook).
- Webhooks from Calendly notify OHC to create an internal "Appointment" record, triggering further workflows.
- **AI Integration**: The Operations Agent can proactively suggest opening more slots if demand is high, or automatically reschedule via email if a conflict arises.

## Implementation Prompt
Integrate Calendly to provide appointment booking capabilities. Implement OAuth flow for merchants to connect their Calendly accounts. Fetch their event types and allow them to embed the booking widget on their storefront. Listen for Calendly webhooks (`invitee.created`, `invitee.canceled`) to maintain an internal appointments ledger.
- **Acceptance Criteria**: Merchant can connect Calendly and select an event type. Customer can book via embedded Calendly widget. OHC creates an internal appointment record upon successful booking via webhook.
- **Priority**: P1
- **Estimated Scope**: Medium
