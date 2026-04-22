# External Tools Integration Research Report

## Overview
This report summarizes research into external tools that can be integrated into the OHC platform to provide essential capabilities for small business owners. The evaluated categories include Social Media Integration, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, and Video Conferencing.

The findings have been documented as structured issue briefs within the `docs/research/` directory.

## Evaluated Tools

### 1. Social Media Integration: Buffer
**Problem Addressed:**
- Unified management and scheduling of social media posts across various platforms (Instagram, Facebook, X, etc.) to save time and maintain a consistent online presence.

**Evaluation:**
Buffer offers a highly intuitive platform perfect for non-technical users and small businesses, featuring a generous free tier. The robust Buffer API will allow OHC to let business owners compose and schedule posts directly from the OHC dashboard, which are then passed to Buffer for publication.

### 2. Calendar & Scheduling: Cal.com & Google Calendar
**Problems Addressed:**
- Avoiding double-bookings across personal and business schedules.
- Providing a seamless booking interface for customers (e.g., Carlos the handyman, Leo the tutor).

**Cal.com Evaluation:**
Cal.com is an open-source, highly customizable scheduling tool with a robust API. It provides a frictionless booking experience for end-users and a generous free tier. The API allows OHC to generate booking links and embed the interface directly into tenant websites.

**Google Calendar Evaluation:**
A ubiquitous tool for managing personal and professional time. Two-way sync via the Google Calendar API is critical to ensure OHC availability is updated based on personal events and vice-versa.

### 3. Email Marketing: Mailchimp
**Problem Addressed:**
- Enabling non-technical business owners (e.g., Priya the boutique owner) to run email campaigns, send newsletters, and trigger automations without needing separate, unlinked CRM tools.

**Evaluation:**
Mailchimp offers an accessible drag-and-drop builder, AI content generation, and a powerful Marketing API. It is an industry leader with a solid free tier (up to 500 contacts). OHC can sync customer data seamlessly, allowing business owners to launch campaigns from the OHC dashboard or jump to Mailchimp via SSO.

### 4. Payment Processing: Mercado Pago
**Problem Addressed:**
- Stripe is not universally adopted or preferred in Latin American markets. Users need localized payment options (Pix, Boleto, OXXO).

**Evaluation:**
Mercado Pago dominates the LATAM market. It offers robust APIs and pre-built checkout modules. Integrating Mercado Pago ensures that businesses operating in countries like Brazil, Mexico, and Argentina can offer trusted, local payment methods, boosting conversion rates.

### 5. Shipping & Logistics: Shippo
**Problem Addressed:**
- Businesses shipping physical goods need a simple way to compare rates across carriers (USPS, UPS, FedEx), print labels, and track packages without managing multiple carrier accounts.

**Evaluation:**
Shippo provides a unified REST API for multi-carrier shipping. It offers a pay-as-you-go model which is friendly for small businesses. OHC can integrate Shippo to fetch real-time rates during checkout and allow business owners to purchase and print labels directly from the OHC fulfillment dashboard.

### 6. SMS & Notifications: Twilio
**Problem Addressed:**
- Customers, especially those with limited internet access or those relying on mobile devices (e.g., Fatima's food cart customers), need timely updates via text message.

**Evaluation:**
Twilio is the industry standard for programmable SMS. It allows OHC to provision phone numbers per tenant and configure automated outbound messages (e.g., order ready, appointment reminders). It also supports inbound webhooks, routing customer replies straight into the OHC unified inbox.

### 7. Video Conferencing: Zoom
**Problem Addressed:**
- Online service providers need automated generation of video links for virtual appointments without manual administrative overhead.

**Evaluation:**
Zoom is universally recognized and highly reliable. Through its API, OHC can automatically generate unique meeting links whenever a virtual service is booked, embedding the link in calendar invites and confirmation emails seamlessly.

## Next Steps
The structured issue briefs have been created and committed to the `docs/research/` directory. These briefs detail the problem statements, design approaches, implementation prompts, and priority levels for each integration. Implementers can use these briefs to begin building out the integrations.
