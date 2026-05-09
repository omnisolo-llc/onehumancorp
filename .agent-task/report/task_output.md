# OHC Tool Integration Research Report

## 1. Social Media Integration
**Problem:** Business owners miss customer inquiries spread across Instagram, Facebook, and WhatsApp, losing potential sales.
**Selected Tool:** Meta Graph API (Instagram/Facebook) & WhatsApp Business API
**Benefits:** Unified inbox for DMs and comments directly inside OHC. Reduces the need to check multiple apps.
**Risks:** High OAuth complexity; Meta frequently updates API terms; WhatsApp Business API has per-conversation pricing.
**Pricing Estimate:** Meta Graph API (Free); WhatsApp (Conversation-based pricing, varies by region).
**Environment:** Works in both Cloud and Standalone (with Cloud proxying webhooks to Standalone).

## 2. Calendar & Scheduling
**Problem:** Back-and-forth emails to schedule meetings or services waste time and lead to double bookings.
**Selected Tool:** Cal.com (Open Source Alternative to Calendly)
**Benefits:** Self-hosted capable, strong API, respects privacy. Allows customers to book directly on the owner's website.
**Risks:** Calendar conflict resolution edge cases (especially across timezones); requires reliable webhook delivery.
**Pricing Estimate:** Free for individuals, $15/user/mo for teams, or self-hosted (infrastructure cost).
**Environment:** Excellent for both Cloud and Standalone (self-hosted nature aligns with Standalone).

## 3. Email Marketing
**Problem:** Business owners struggle to reach existing customers with promotions without using complex, expensive tools like Mailchimp.
**Selected Tool:** Resend / Listmonk
**Benefits:** Resend offers an excellent developer API for transactional/marketing emails. Listmonk is a self-hosted alternative perfect for the Standalone mode.
**Risks:** Managing spam reputation, bounce rates, and strict unsubscribe compliance (CAN-SPAM/GDPR).
**Pricing Estimate:** Resend ($20/mo for 50k emails); Listmonk (Free/Open Source).
**Environment:** Cloud (Resend); Standalone (Listmonk).

## 4. Payment Processing
**Problem:** Stripe isn't supported everywhere or has high fees. Users in LATAM need local solutions.
**Selected Tool:** Mercado Pago
**Benefits:** Dominant in LATAM, supports local payment methods (Pix in Brazil, OXXO in Mexico).
**Risks:** Settlement delays, complex API for partial refunds, high currency volatility.
**Pricing Estimate:** Varies by country (typically ~3.99% + fixed fee).
**Environment:** Cloud and Standalone (API-based, requires secure webhook handling).

## 5. Shipping & Logistics
**Problem:** Calculating shipping rates manually leads to undercharging or overcharging customers.
**Selected Tool:** Shippo
**Benefits:** Connects to 85+ carriers globally. Simplifies label generation and provides real-time rates.
**Risks:** Carrier API downtime, label generation errors, international customs documentation complexity.
**Pricing Estimate:** Free tier available ($0.05 per label if using own carrier accounts).
**Environment:** Cloud and Standalone (API-based).

## 6. SMS & Notifications
**Problem:** Customers (and business owners like Fatima) may have low English proficiency or lack reliable internet, making SMS the most reliable channel.
**Selected Tool:** Twilio
**Benefits:** Global reach, high reliability, supports WhatsApp fallback.
**Risks:** Strict carrier regulations (A2P 10DLC in the US), high costs for international SMS.
**Pricing Estimate:** ~$0.0079 per SMS (US), higher internationally.
**Environment:** Cloud and Standalone.

## 7. Video Conferencing
**Problem:** Manually creating and sharing Zoom links for consultations is tedious and prone to errors.
**Selected Tool:** Google Meet API
**Benefits:** Deeply integrated with Google Workspace, reliable, widely adopted, no additional software required for users.
**Risks:** Requires Google OAuth (friction for non-Google users); link generation limits.
**Pricing Estimate:** Free with Google Workspace accounts.
**Environment:** Cloud and Standalone.
