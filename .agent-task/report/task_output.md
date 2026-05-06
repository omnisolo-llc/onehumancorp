# OHC Tool Integration Research Report Q3

This report outlines the research and evaluation of various third-party tools categorized by their potential to solve real problems for small business owners using OHC (Open Home Controller). The focus is exclusively on tools that provide direct value to non-technical users in both Cloud and Standalone environments.

## Executive Summary
We investigated seven categories critical to small business operations:
1. Social Media Integration
2. Calendar & Scheduling
3. Email Marketing
4. Payment Processing
5. Shipping & Logistics
6. SMS & Notifications
7. Video Conferencing

For each category, an actionable issue brief has been generated in the `docs/research/` directory.

---

## 1. Social Media Integration
**Selected Tool:** ManyChat / Meta Graph API
**Problem:** Managing customer inquiries across Instagram, Facebook, and WhatsApp is chaotic.
**Evaluation:** ManyChat provides a robust API for consolidating messages into a unified inbox. It uses standard OAuth, making it highly accessible for non-technical users. Pricing is affordable (freemium, starting at $15/mo), and it supports both Cloud (webhooks) and Standalone (webhook relay) deployments.
**Issue Brief:** `[social-media]-manychat.md`

## 2. Calendar & Scheduling
**Selected Tool:** Cal.com
**Problem:** Back-and-forth scheduling with clients wastes time.
**Evaluation:** Cal.com is an open-source Calendly alternative. It allows business owners to connect Google/Outlook calendars and generate public booking pages. The open-source nature makes it perfect for Standalone self-hosting alongside OHC, while the Cloud API is extremely reliable. Generous free tier.
**Issue Brief:** `[calendar]-cal-dot-com.md`

## 3. Email Marketing
**Selected Tool:** Resend
**Problem:** Traditional tools like Mailchimp are disconnected from core operations and overly complex.
**Evaluation:** Resend offers a developer-first approach that allows OHC to build a highly simplified, native UI for marketing blasts and transactional emails. Excellent free tier (3,000 emails/mo). Works natively via Cloud API; Standalone requires users to provide API keys or SMTP.
**Issue Brief:** `[email]-resend.md`

## 4. Payment Processing
**Selected Tool:** Mercado Pago
**Problem:** Stripe presents high friction in Latin America due to limited local payment methods (e.g., Pix, cash).
**Evaluation:** Mercado Pago is the undisputed leader in LATAM. It supports local cards and alternative payment methods, reducing cross-border fees. Integrating it alongside Stripe provides crucial geographic flexibility.
**Issue Brief:** `[payment]-mercado-pago.md`

## 5. Shipping & Logistics
**Selected Tool:** EasyPost
**Problem:** Manual tracking and post office visits waste time for e-commerce sellers.
**Evaluation:** EasyPost provides a unified API for USPS, FedEx, UPS, and DHL. It offers commercial USPS pricing and automates label generation and tracking link sharing. High ease of use once configured.
**Issue Brief:** `[shipping]-easypost.md`

## 6. SMS & Notifications
**Selected Tool:** Twilio
**Problem:** High appointment no-show rates for clients who don't check email.
**Evaluation:** Twilio is the industry standard for SMS. By abstracting the API behind a simple OHC settings toggle, business owners can easily enable 24-hour SMS reminders. Very affordable per message.
**Issue Brief:** `[sms]-twilio.md`

## 7. Video Conferencing
**Selected Tool:** Zoom
**Problem:** Manually creating and sharing video links for online sessions is tedious.
**Evaluation:** Using Zoom's Server-to-Server OAuth, OHC can automatically generate unique meeting links upon booking and append them to calendar events. It leverages a globally recognized tool with a viable free tier for short meetings.
**Issue Brief:** `[video]-zoom.md`

---
## Next Steps
The engineering team should review the generated issue briefs in `docs/research/` to prioritize implementation in upcoming sprints based on user demand and estimated scope.
