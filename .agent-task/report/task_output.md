# OHC Tool Integration Research Report

## Executive Summary
This report details the findings from researching seven distinct categories of tools aimed at expanding OneHumanCorp's (OHC) capabilities for small business owners. The research focused on the tools' ease of use for non-technical users, features, pricing, and potential integration with the OHC platform.

Individual issue briefs have been created in the `docs/research/` directory for each recommended integration.

## Investigated Categories & Recommended Tools

### 1. Calendar & Scheduling: Calendly
- **Brief:** `docs/research/[calendar]_calendly.md`
- **Target Persona:** Service providers (e.g., Carlos the Handyman, Leo the Music Tutor) who need automated booking to avoid manual back-and-forth.
- **Why Calendly:** A market leader in automated scheduling with a strong freemium model. It easily integrates with existing Google/Outlook calendars, making it highly accessible for non-technical users.
- **Integration Outcome:** Business owners can connect their account to embed a booking widget on their OHC storefront, allowing customers to directly book available slots.

### 2. Email Marketing: Mailchimp
- **Brief:** `docs/research/[email_marketing]_mailchimp.md`
- **Target Persona:** Boutiques and freelancers (e.g., Priya the Boutique Owner) looking to maintain relationships and send promotional offers.
- **Why Mailchimp:** Highly recognized, user-friendly drag-and-drop builder with an excellent free tier for new businesses.
- **Integration Outcome:** OHC will automatically sync the tenant's customer list to a Mailchimp Audience. The owner uses Mailchimp's UI to design and send the actual emails, keeping OHC's interface simple.

### 3. SMS & Notifications: Twilio
- **Brief:** `docs/research/[sms]_twilio.md`
- **Target Persona:** Highly mobile users or those with limited English/data connectivity (e.g., Fatima the Food Cart Operator) who need instant, reliable alerts.
- **Why Twilio:** Unmatched global carrier coverage and a robust, scalable API.
- **Integration Outcome:** OHC uses Twilio to dispatch automated transactional SMS alerts (e.g., "Order Ready", "Booking Reminder") directly to the owner's and customer's phones.

### 4. Video Conferencing: Zoom
- **Brief:** `docs/research/[video]_zoom.md`
- **Target Persona:** Online service providers (e.g., Leo the Music Tutor) needing secure, auto-generated meeting links.
- **Why Zoom:** Ubiquitous brand recognition and a strong API for programmatic meeting creation.
- **Integration Outcome:** When a virtual service is booked, OHC automatically creates a Zoom meeting (with waiting room/passcode enabled) and injects the link into calendar invites and confirmation emails.

### 5. Social Media Integration: WhatsApp (via Twilio API)
- **Brief:** `docs/research/[social_media]_whatsapp.md`
- **Target Persona:** Businesses in LATAM or global markets where WhatsApp is the primary communication channel, needing to separate personal and business messages.
- **Why WhatsApp (via Twilio):** Twilio's Programmable Messaging API abstracts the complexity of direct WhatsApp Business integration while providing a unified webhook structure.
- **Integration Outcome:** Inbound WhatsApp messages are routed to the merchant's unified OHC inbox. The owner can reply directly from OHC, maintaining a professional separation of communications.

### 6. Payment Processing: Mercado Pago
- **Brief:** `docs/research/[payment]_mercadolibre.md`
- **Target Persona:** Small business owners in Latin America needing localized payment methods (e.g., cash vouchers, local installments) beyond Stripe.
- **Why Mercado Pago:** The largest and most trusted e-commerce payment ecosystem in LATAM.
- **Integration Outcome:** LATAM merchants can set Mercado Pago as their active gateway. The integration handles the checkout redirect and processes asynchronous cash payment webhooks to update order statuses in OHC.

### 7. Shipping & Logistics: Deliverr (Flexport)
- **Brief:** `docs/research/[shipping]_deliverr.md`
- **Target Persona:** Product sellers (e.g., Priya the Boutique Owner) who want to offer fast, Amazon-style delivery without managing a warehouse.
- **Why Deliverr (Flexport):** Specifically built to offer 2-day fulfillment for independent merchants. It integrates well with multi-channel setups.
- **Integration Outcome:** OHC pushes physical product orders to the Flexport API. Once shipped, tracking data syncs back to OHC, automatically updating the order status and notifying the customer.
