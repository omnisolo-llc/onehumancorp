# OHC Tool Integration Research Report

## Methodology
Research was conducted across seven target categories focusing on tools providing significant value to small business owners. Evaluation criteria prioritized usability for non-technical users, pricing models suitable for small businesses, and compatibility with both Cloud and Standalone environments.

## 1. Social Media Integration
**Problem Statement:** Small business owners struggle to manage customer communications scattered across Instagram, Facebook, WhatsApp, and TikTok.
**Target Tool:** ManyChat / Chatwoot
**Evaluation:**
Chatwoot offers an excellent open-source, API-first omnichannel inbox. It's particularly well-suited for OHC because it can be self-hosted (Standalone mode) or used as a SaaS (Cloud mode). ManyChat is more powerful for pure Instagram/FB automation but less flexible for a unified inbox.
**Recommendation:** Chatwoot.

## 2. Calendar & Scheduling
**Problem Statement:** Back-and-forth emails to schedule appointments or consultations waste time and lead to missed opportunities.
**Target Tool:** Cal.com
**Evaluation:**
Cal.com is open-source, highly customizable, and provides a white-label scheduling experience. It natively supports Google/Outlook calendars and Zoom/Meet integration. Its self-hosting capability makes it a perfect fit for OHC Standalone mode, while their API handles Cloud mode.
**Recommendation:** Cal.com.

## 3. Email Marketing
**Problem Statement:** Reaching out to existing customers with promotions is too complex with traditional enterprise tools.
**Target Tool:** MailerLite / Listmonk
**Evaluation:**
MailerLite is very user-friendly but SaaS-only. Listmonk is an open-source newsletter and mailing list manager with a modern dashboard. It supports high-performance delivery and is easily packaged for Standalone mode, while OHC can wrap its API for a simplified Cloud experience.
**Recommendation:** Listmonk for Standalone/Hybrid, MailerLite for pure SaaS ease.

## 4. Payment Processing
**Problem Statement:** Collecting payments requires managing different gateways, especially for international or non-credit-card transactions.
**Target Tool:** Stripe (with specific regional focus) / Razorpay (India) / Mercado Pago (LATAM)
**Evaluation:**
Stripe remains the easiest global default, but integrating a flexible gateway aggregator like Hyperswitch (open source) allows OHC to support regional providers (Razorpay, Mercado Pago) without building N integrations.
**Recommendation:** Hyperswitch.

## 5. Shipping & Logistics
**Problem Statement:** Calculating rates, printing labels, and tracking packages is a manual, error-prone process.
**Target Tool:** Shippo / EasyPost
**Evaluation:**
Both offer robust APIs for multi-carrier shipping. EasyPost is generally favored for developer experience and reliability. Shippo has very competitive USPS rates for US-based small businesses.
**Recommendation:** Shippo for US focus, EasyPost for global.

## 6. SMS & Notifications
**Problem Statement:** Important updates (like appointment reminders or order updates) are missed when sent via email, especially for users with lower tech/English proficiency.
**Target Tool:** Twilio / MessageBird
**Evaluation:**
Twilio is the industry standard with the best global coverage. For a small business owner, OHC needs to abstract the complexity of Twilio's API, providing a simple interface to purchase numbers and send messages.
**Recommendation:** Twilio (abstracted via OHC).

## 7. Video Conferencing
**Problem Statement:** Creating and sharing meeting links for remote services is clunky.
**Target Tool:** Zoom / Google Meet / Jitsi
**Evaluation:**
Jitsi Meet is open-source and can be embedded directly into the OHC platform without requiring the business owner or customer to create new accounts or install apps. This is ideal for a seamless, white-labeled experience.
**Recommendation:** Jitsi.

---
*Detailed Issue Briefs have been generated in the `docs/research/` directory.*
