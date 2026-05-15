# [calendar] Acuity Scheduling Integration

## Problem Statement
Small business owners, such as consultants, therapists, and service providers, spend hours each week playing "email ping-pong" to find suitable meeting times with clients. This manual process is inefficient, prone to errors, and can frustrate clients. By integrating Acuity Scheduling into OHC, business owners can offer a seamless, self-serve booking experience that automatically syncs with their calendars and handles timezones effortlessly.

## Research Report
### Overview
Acuity Scheduling (a Squarespace company) is a robust online appointment scheduling tool tailored for businesses that need to manage complex booking rules, accept payments upfront, and offer various service types.

### Ease of Use
For the business owner, setting up Acuity involves defining their availability, services, and intake forms. The integration with OHC should be simple: an OAuth connection that pulls their existing Acuity configuration into the OHC platform. For the end-client, the booking experience is highly intuitive and mobile-friendly.

### Reputation
Acuity has a strong reputation for reliability, extensive customization options, and deep integrations with other business tools. It is widely trusted by service-based small businesses.

### Pricing
Acuity operates on a tiered subscription model, starting around $20/month. The OHC integration itself should be free, but the business owner will need an active Acuity subscription.

### Environment
Works in Cloud.

### AI Integration
Medium potential. AI could be used to analyze booking patterns, suggest optimal availability adjustments, or draft personalized pre-meeting and post-meeting emails based on the appointment type.

## Design Doc
1.  **Connection:** The user navigates to "Integrations" -> "Calendar & Scheduling" -> "Connect Acuity". This triggers the Acuity OAuth flow.
2.  **Configuration:** OHC fetches the user's appointment types and calendar links from Acuity. The user can embed these links on their OHC-hosted storefront or share them via the unified inbox.
3.  **Synchronization:** Webhooks ensure that when an appointment is booked, canceled, or rescheduled in Acuity, the corresponding event is updated in the OHC platform (e.g., adding a note to the customer's CRM profile).
4.  **Display:** Appointments are visible within the OHC calendar view alongside other events.

## Implementation Prompt
Implement an integration with Acuity Scheduling. The integration should allow the business owner to connect their Acuity account via OAuth. Once connected, OHC should fetch the user's public booking links and allow them to easily insert these links into emails or SMS messages sent from the unified inbox. Additionally, OHC should listen for Acuity webhooks to automatically log booked appointments to the respective customer's CRM profile within OHC.

## Priority
P1 (High) - Highly requested by service-oriented businesses.

## Estimated Scope
Medium
