# OHC Scout: Tool Integration Research Report

## Executive Summary
This report evaluates native integrations across seven key categories to support small business owners on the OHC platform. Following the Radical Simplicity ethos and User-First lens, each tool was assessed based on its ability to seamlessly embed into the OHC platform, removing the need for business owners to learn or manage third-party software.

The selected tools span Social Media, Calendar, Email Marketing, Payments, Shipping, SMS, and Video Conferencing.

---

## 1. Social Media Integration
**Chosen Solution**: Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Problem Solved**: Centralizes inquiries from Instagram, Facebook, and WhatsApp into a single OHC unified inbox, allowing the AI Customer Success Agent to manage conversations.
- **Why it Fits OHC**: Direct integration bypasses third-party SaaS fees and maintains simplicity for the user.
- **Priority**: P0 | **Scope**: Large

## 2. Calendar & Scheduling
**Chosen Solution**: Google Calendar API
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Problem Solved**: Eliminates the back-and-forth of scheduling by synchronizing OHC bookings directly with the user's existing Google Calendar.
- **Why it Fits OHC**: Zero configuration required beyond login, seamlessly replacing disjointed tools like Calendly.
- **Priority**: P1 | **Scope**: Medium

## 3. Email Marketing
**Chosen Solution**: Native Email Campaign Manager
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Problem Solved**: Allows automated customer outreach (e.g., new stock alerts) natively within OHC, managed by the AI Marketing Agent.
- **Why it Fits OHC**: Removes the complexity of learning Mailchimp; keeps list management unified inside OHC.
- **Priority**: P1 | **Scope**: Large

## 4. Payment Processing
**Chosen Solution**: Mercado Pago
- **Target Persona**: Global users outside the US/EU (specifically LATAM)
- **Problem Solved**: Provides trusted, local payment options (e.g., Pix, Pago Fácil) for non-US/EU merchants where Stripe is inaccessible.
- **Why it Fits OHC**: Broadens market reach with native checkout flows seamlessly connected to OHC order tracking.
- **Priority**: P2 | **Scope**: Large

## 5. Shipping & Logistics
**Chosen Solution**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Problem Solved**: Allows merchants to compare rates, purchase, and print shipping labels with one click natively inside OHC.
- **Why it Fits OHC**: Prevents manual data entry across disparate carrier sites, keeping fulfillment radically simple.
- **Priority**: P1 | **Scope**: Large

## 6. SMS & Notifications
**Chosen Solution**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Problem Solved**: Delivers highly reliable text alerts for new orders in noisy, fast-paced environments where push notifications fail.
- **Why it Fits OHC**: Operates invisibly in the background; merchants simply toggle a setting to receive SMS alerts.
- **Priority**: P2 | **Scope**: Medium

## 7. Video Conferencing
**Chosen Solution**: Zoom
- **Target Persona**: Leo (Music Tutor)
- **Problem Solved**: Automates the creation and sharing of meeting links for online services when booked.
- **Why it Fits OHC**: Eliminates manual copy-pasting, standardizing the connection flow directly from the OHC Sales dashboard.
- **Priority**: P1 | **Scope**: Medium

---
## Conclusion
By natively integrating these core infrastructure services, OHC can replace a fragmented suite of disparate tools (Calendly, Mailchimp, Manychat) with a single, AI-orchestrated environment. This directly delivers on the platform's promise of Radical Simplicity, ensuring that technical complexity is absorbed by the platform, leaving the business owner free to focus on their operations.
