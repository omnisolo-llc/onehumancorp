# Title: Integrate Calendly for Frictionless Client Scheduling

## Problem Statement
Small business owners, especially consultants, salon owners, and freelancers, lose hours to back-and-forth emails trying to schedule appointments with clients. They need a simple, self-serve booking page they can share via a link, which automatically syncs with their availability so they never get double-booked.

## Research Report
Calendly is the industry standard for automated scheduling.
- **Ease of Use:** Incredibly user-friendly for both the business owner and their clients. Setting up availability rules, event types, and connecting a Google/Outlook calendar takes minutes.
- **Pricing:** Offers a very capable free tier (one active event type). Paid plans start at $10/month to unlock multiple event types and automated workflows. This is highly affordable for micro-businesses.
- **Reputation:** Universally recognized, which builds trust when clients see the booking link.
- **Competitors:** Acuity Scheduling (better for complex class bookings, but steeper learning curve), Microsoft Bookings. Calendly remains the most straightforward for simple 1-on-1 meetings.
- **Cloud vs Standalone:** Excellent for Cloud mode via API/webhooks. Standalone mode can embed the booking page iframe, though two-way data sync back into a local OHC database requires user-managed API keys or a polling service.

## Design Doc
OHC will allow users to embed their Calendly scheduling page directly into their public profile or website, and sync booked appointments into the OHC internal calendar view.
- **Trigger:** A client books a meeting via the user's Calendly link or embedded widget.
- **Action:** OHC receives the booking confirmation and creates an appointment record linked to the specific client in the CRM.
- **User Interface:** The business owner sees upcoming appointments seamlessly overlaid on their OHC calendar. Clicking an appointment shows the client's details and any pre-booking questionnaire answers.

## Implementation Prompt
Build a "Booking Settings" module where users can paste their Calendly personal link or authenticate via API. Embed their Calendly booking widget on their public-facing OHC page. Additionally, listen for new bookings (via webhook or polling) and display these upcoming appointments in the internal OHC Calendar dashboard, linking them to existing or new customer profiles in the CRM.

## Priority
P0

## Estimated Scope
Small