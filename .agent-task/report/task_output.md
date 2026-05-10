# OHC Market Research & Issue Brief: Automating SMB Growth

## Market Sizing & Strategic Direction

- **Total Addressable Market (TAM):** There are over 33 million small businesses in the US alone (US Chamber of Commerce), and globally over 330 million SMEs (World Bank). Upwards of 25-30% of these businesses still do not have a dedicated website, instead relying entirely on social media platforms (like Instagram DMs or Facebook groups) to conduct business.
- **Beachhead Market:** The initial focus should be on "Side Hustlers & Solopreneurs" (like Maya the baker and Carlos the handyman). This segment has the highest density of underserved users who lack technical skills and need simple, unified solutions for booking, inventory, and payment.
- **Geographic Expansion:** After securing the English-speaking market, LATAM (Spanish) is the prime expansion target, due to high mobile penetration and heavy reliance on WhatsApp for commerce.
- **Vertical Expansion:** After the horizontal launch, focusing on the Food & Beverage vertical (food carts, home bakers) will provide high value with specific POS integration needs.

## Top 10 SMB Pain Points (Validated by Reddit, Trustpilot, App Store)

1.  **Overwhelming Initial Setup:** (Frequency: Very High) Shopify and WordPress require technical knowledge or too much time. Users abandon setup within the first hour.
2.  **Fragmented Tools:** (Frequency: Very High) Using one tool for a website, another for booking (Calendly), another for payments (Venmo/Square), and another for messaging (Instagram).
3.  **Mobile Management:** (Frequency: High) Existing apps (like Shopify's setup app) are poor for running the business exclusively from a mobile phone.
4.  **No Automated Follow-Ups:** (Frequency: High) Losing leads because the owner is too busy to manually reply to DMs or emails.
5.  **Inventory Sync Issues:** (Frequency: Medium) Keeping track of what's sold online vs. in-person.
6.  **Writing Copy:** (Frequency: Medium) Struggling to write professional product descriptions or marketing emails.
7.  **Unclear Pricing/Upsells:** (Frequency: Medium) Hidden fees or aggressive upselling (frequent complaint about GoDaddy).
8.  **Poor Customer Support:** (Frequency: Medium) Lack of immediate, understandable help when things break.
9.  **No Native Booking:** (Frequency: Medium) Service businesses struggle to integrate booking into standard e-commerce platforms.
10. **Lack of Business Insights:** (Frequency: Low but impactful) Not knowing what products are profitable or where customers are dropping off.

## Feature Gap Matrix

| Feature | Shopify | Wix | OHC (current) | OHC (gap/advantage) |
| :--- | :--- | :--- | :--- | :--- |
| **Setup Speed** | Slow | Medium | Very Fast (under 10m) | **Advantage:** Mobile-first, AI-driven setup. |
| **AI Assistants** | Chatbot (Sidekick) | One-time Builder (ADI) | Built-in Agents | **Advantage:** Invisible, continuous AI automation. |
| **Mobile App (Setup)** | Poor | Limited | Strong (Slint/Rust) | **Advantage:** 100% mobile parity constraint. |
| **Booking Engine** | Requires App | Built-in | Needs implementation | **Gap:** Native booking for service SMBs. |
| **Social DM Inbox** | Inbox App | Ascend | Needs implementation | **Gap:** Unified inbox for IG/FB/WhatsApp. |
| **Free Tier** | 3 days | Yes (Ads) | Yes | **Advantage:** Useful free tier with soft limits. |

## OHC AI Differentiation Manifesto

The core differentiator for OHC is shifting from *AI as a Chatbot* to *AI as an Invisible Employee*.
1.  **Auto-replying to customer messages:** Integrates with social channels to answer common questions (hours, pricing) automatically. (Saves hours per day).
2.  **Auto-writing product descriptions:** Generates SEO-optimized descriptions from a single photo. (Saves ~30 min per upload).
3.  **Auto-generating social posts:** Creates draft posts based on new inventory or promotions. (Removes the biggest marketing barrier).
4.  **Auto-sending follow-up emails:** Detects abandoned carts or inactive customers and sends personalized recovery emails.
5.  **AI-generated weekly business insights:** Delivers a simple, plain-English summary of performance and actionable advice (e.g., "Your blueberry muffins sold out fast, consider raising the price by $0.50").

---

# [Feature] Unified Booking Engine for Service SMBs

## Problem Statement

Service-based small business owners—like Carlos the handyman or Leo the music tutor—currently have to duct-tape together multiple tools to run their business. They might use a simple website builder for their presence, but then have to embed complex, third-party widgets (like Calendly) for booking, and handle payments separately through Venmo or Square. This fragmentation is confusing to set up, looks unprofessional to clients, and makes it impossible to manage everything seamlessly from a mobile phone. They need a built-in, dead-simple way to let clients book time and pay in one step.

## Research Report

- **Target Persona:** Carlos (42, Handyman), Leo (22, Music Tutor).
- **Pain Point Addressed:** Fragmented Tools, No Native Booking, Mobile Management.
- **Competitor Landscape:**
    - Shopify: E-commerce first. Requires paid third-party apps for booking, which are often clunky and not mobile-optimized.
    - Wix: Offers a built-in booking engine (Wix Bookings), which is robust but can be complex to configure on mobile.
    - Squarespace: Acquired Acuity Scheduling, offers good integration but requires an additional subscription.
- **Evidence:** Reddit threads in r/smallbusiness frequently ask for "simple website builder with booking". App Store reviews for e-commerce platforms often complain about the lack of native service-business support.

## Design Doc

### High-Level Architecture
- **Entity Types:** `Service` (name, duration, price), `Availability` (business hours, exceptions), `Booking` (customer, service, time slot, payment status).
- **Key Relationships:** A `Business` has many `Services`. A `Service` has many `Bookings`. A `Booking` requires an `Availability` slot and a `Payment`.
- **Integration Points:** Needs to integrate deeply with the existing OHC scheduling and payment modules. Must sync with external calendars (Google Calendar integration - see `docs/research/[calendar]_google_calendar.md`).

### Mobile UX Flow (375px First)
1. **Setup Mode:** User taps "Add Service". They enter the name, duration (e.g., 60 mins), and price. They set their general weekly availability (e.g., Mon-Fri 9-5).
2. **Client View:** Client visits the OHC storefront. Taps "Book Now". Sees a simple calendar interface. Selects a day and available time slot.
3. **Checkout:** Client enters their name/phone and pays the deposit (or full amount) seamlessly via the existing checkout flow.
4. **Management:** Business owner gets a push notification: "New Booking: Leo at 2 PM". The dashboard shows upcoming bookings chronologically.

### AI Agent Integration Points
- **Auto-Scheduling:** An AI agent could analyze the owner's linked calendar and automatically adjust the `Availability` to avoid conflicts without manual intervention.
- **Smart Reminders:** An AI agent automatically sends SMS/Email reminders to the client 24 hours before the appointment to reduce no-shows.

## Implementation Prompt

Implement a "Native Booking Engine" feature tailored for service-based SMBs within the OHC platform.
- The outcome must allow a user to define a service (with duration and price) and set basic availability.
- The storefront must display a simple scheduling UI for clients to select available slots and complete checkout.
- The business dashboard must display a chronological list of upcoming bookings.
- **Critical User Journey (CUJ):** A user creates a new "Consultation" service, a client visits the site, books a time for tomorrow, completes payment, and the user views the new booking in their dashboard.
- **Acceptance Criteria:** Must work perfectly on mobile (375px). Must integrate with the existing payment system. Must include the UI to manage availability.

## Priority
P1

## Estimated Scope
Large