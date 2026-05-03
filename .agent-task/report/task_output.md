# [calendar] Calendar & Scheduling Integration: Cal.com vs Google Calendar

## Title
Calendar & Scheduling Integration: Seamless Bookings for OHC

## Problem Statement
Small business owners—like Carlos the Freelance Handyman and Leo the Music Tutor—need an effortless way for customers to view availability, book time slots, and schedule services online. Currently, managing bookings involves manual back-and-forth emails, phone calls, or DMs, which leads to double-bookings, time zone confusion, and lost revenue from missed appointments. Our non-technical users need a drop-in calendar system that syncs their personal/business calendar automatically, handles time zones properly, and generates meeting links (e.g., Zoom/Google Meet) without requiring them to configure complex integrations themselves.

## Research Report
We evaluated two primary options for Calendar & Scheduling: Google Calendar API (raw integration) and Cal.com (open-source scheduling infrastructure).

**Google Calendar API (Direct)**
- **Pros**: Free, ubiquitous, deeply integrated into the Google ecosystem.
- **Cons**: High engineering effort to build scheduling UI, handle timezones, manage conflict resolution, and implement recurring bookings. Lacks built-in meeting link generation for external services (e.g., Zoom).
- **Target User Fit**: While powerful, asking non-technical users to manage OAuth scopes and raw calendar endpoints is complex. Building the entire scheduling UI from scratch on top of the API adds massive overhead.

**Cal.com (Open-Source Infrastructure)**
- **Pros**: Provides pre-built scheduling UI, advanced availability logic (buffer times, minimum notice), timezone detection, and out-of-the-box integrations with Google Calendar, Outlook, Apple Calendar, Zoom, and Stripe (for paid bookings).
- **Cons**: Requires either managing a self-hosted instance (for Standalone/local) or using their managed cloud service.
- **Target User Fit**: Excellent. Business owners can connect their existing calendars in one click, and OHC can embed the scheduling widget directly into the generated storefronts.
- **Pricing**: Cal.com has a generous free tier for individuals (perfect for our Free Tier users) and scalable platform pricing for SaaS integrations. It is also open-source, allowing local deployment in our Standalone environment.

**Conclusion**: Cal.com is the clear winner for OHC. It bridges the gap between raw calendar APIs and a fully-featured booking system, perfectly matching our need for drop-in scheduling that works in both Cloud and Standalone environments.

## Design Doc
**Trigger**: A business owner in the OHC dashboard enables the "Bookings" feature for a service.
**Action**:
1. The AI Operations Agent ("The Manager") prompts the user to connect their existing calendar (Google/Outlook/Apple) via Cal.com's OAuth flow.
2. The user sets their availability (e.g., Mon-Fri 9 AM-5 PM) through a simplified OHC interface, which syncs to Cal.com.
3. The AI Marketing Agent ("The Promoter") automatically embeds the Cal.com scheduling widget into the business's public storefront.
4. When a customer books a slot, Cal.com handles the calendar invite, generates a Zoom/Meet link, and fires a webhook to OHC.
5. The Operations Agent processes the webhook, updates the internal OHC database, and triggers the Customer Success Agent ("The Ambassador") to send a personalized booking confirmation to the customer.

**User View**: The business owner never leaves the OHC dashboard. They see a simple "Connect Calendar" button and a list of their upcoming bookings. The customer sees a beautifully integrated, brand-matching calendar widget on the storefront.

## Implementation Prompt
Implement the Cal.com integration to enable service bookings on OHC storefronts.

**Acceptance Criteria**:
1. Business owners can authenticate and connect their primary calendar (Google/Outlook) from the OHC dashboard.
2. Business owners can define their weekly availability and minimum notice periods for bookings.
3. A booking widget (via Cal.com) is available as a component in the storefront builder, correctly reflecting the owner's availability.
4. When a customer completes a booking, the OHC system receives a webhook from Cal.com, records the booking in the OHC database against the specific tenant, and triggers a confirmation notification to the customer.
5. The integration must be robust, handling webhook retries and correctly isolating tenant data.

## Priority
P0

## Estimated Scope
Medium
