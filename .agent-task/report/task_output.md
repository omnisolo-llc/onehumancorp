# Scout: Tool Integration Research Report

## Overview
This report evaluates key tool integrations across several categories that are vital for small business owners using OneHumanCorp (OHC). The goal is to identify and document integrations that provide immediate, tangible value to our users, especially non-technical owners, in both Cloud and Standalone environments.

## Findings

### 1. Social Media: Meta Graph API
- **Focus:** Unified inbox for Instagram, Facebook, and WhatsApp.
- **Why:** Prevent missed sales due to scattered messages.
- **Cloud/Standalone Suitability:** Excellent for Cloud via webhooks. Standalone environments require additional polling or local tunnel solutions.
- **Ease of Use:** Requires standard OAuth, manageable for most users.
- **Brief Created:** `docs/research/[social-media]_meta-graph.md`

### 2. Calendar: Cal.com
- **Focus:** Automated booking and scheduling.
- **Why:** Eliminate the back-and-forth of email scheduling.
- **Cloud/Standalone Suitability:** Excellent for both. Open-source nature aligns perfectly with our Standalone (self-hosted) ethos.
- **Ease of Use:** Intuitive setup and public booking link generation.
- **Brief Created:** `docs/research/[calendar]_cal-dot-com.md`

### 3. Payments: Mercado Pago
- **Focus:** Payment processing tailored for Latin America (e.g., Pix, Boletos).
- **Why:** Critical for LATAM markets where Stripe is insufficient, avoiding lost sales at checkout.
- **Cloud/Standalone Suitability:** Strong in Cloud. Standalone requires webhook reachability or polling.
- **Ease of Use:** Simple OAuth connection for business owners.
- **Brief Created:** `docs/research/[payments]_mercado-pago.md`

### 4. SMS & Notifications: Twilio
- **Focus:** Global, reliable SMS notifications.
- **Why:** Crucial for communicating with customers who do not use email reliably, reducing no-shows and late payments.
- **Cloud/Standalone Suitability:** Perfect for both, as it primarily relies on outbound API calls.
- **Ease of Use:** Requires input of API credentials, but operation is entirely automated thereafter.
- **Brief Created:** `docs/research/[sms]_twilio.md`

### 5. Email Marketing: Mailchimp
- **Focus:** Automated email campaigns and contact syncing.
- **Why:** Replaces manual CSV exports, ensuring customers receive timely promotional materials.
- **Cloud/Standalone Suitability:** Excellent for both via outbound API calls.
- **Ease of Use:** Simple OAuth connection and highly intuitive drag-and-drop template builder in Mailchimp.
- **Brief Created:** `docs/research/[email-marketing]_mailchimp.md`

### 6. Shipping & Logistics: Shippo
- **Focus:** Real-time rate calculation and label generation.
- **Why:** Saves time on fulfilling physical orders by avoiding post office lines and manual label generation.
- **Cloud/Standalone Suitability:** Excellent for both via outbound API calls.
- **Ease of Use:** Simple API key/OAuth connection. Live rates work behind the scenes.
- **Brief Created:** `docs/research/[shipping]_shippo.md`

### 7. Video Conferencing: Zoom
- **Focus:** Auto-generating video links for booked appointments.
- **Why:** Eliminates manual meeting creation, preventing forgotten links and frustrated clients.
- **Cloud/Standalone Suitability:** Excellent for both via outbound API calls.
- **Ease of Use:** Standard OAuth connection. High familiarity among clients.
- **Brief Created:** `docs/research/[video]_zoom.md`

## Next Steps
- Review and prioritize the generated issue briefs.
- Begin technical design and prototyping for the P0 (Twilio) and P1 (Meta, Cal.com, Shippo) integrations.
- Investigate localized polling strategies for Meta and Mercado Pago in Standalone environments.
