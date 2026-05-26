# Tool Integration Research Report

## Overview
This report evaluates 7 tool categories for native integration into the OHC (OneHumanCorp) platform, focusing on the needs of non-technical small business owners like Maya (Home Baker), Fatima (Food Cart), Priya (Boutique), and Leo (Music Tutor). The goal is to provide native, invisible AI-driven integrations that eliminate the need for merchants to juggle complex third-party dashboards.

---

## 1. Social Media Integration
- **Evaluated Tool:** WhatsApp Business API (via Meta Graph API)
- **Problem Solved:** Prevents missed sales due to manual messaging. Automates responses for businesses (like Fatima's food cart) where WhatsApp is the primary ordering channel.
- **Benefits:** Universal adoption. Integrates seamlessly into the OHC unified inbox, allowing the AI Ambassador to auto-reply.
- **Risks:** Strict API approval process and template policies by Meta.
- **Pricing:** First 1000 service conversations per month are free; standard regional pricing applies thereafter.
- **Compatibility:** Fully supported in Cloud. Standalone requires a webhook relay.

## 2. Calendar & Scheduling
- **Evaluated Tool:** Google Calendar & Google Meet (Google Workspace)
- **Problem Solved:** Eliminates manual calendar management and back-and-forth emails for service providers (like Leo).
- **Benefits:** Familiar OAuth flow. Prevents double-booking automatically.
- **Risks:** Requires Google Workspace for programmatic Meet link generation.
- **Pricing:** Free for basic calendar sync; Workspace subscription required for Meet.
- **Compatibility:** Supported in both Cloud and Standalone environments.

## 3. Email Marketing
- **Evaluated Tool:** Twilio SendGrid Email API
- **Problem Solved:** Allows merchants (like Priya) to broadcast emails directly from OHC without learning complex external tools like Mailchimp.
- **Benefits:** Native list management via OHC. AI-generated campaigns sent with one click.
- **Risks:** Requires internal implementation of unsubscribe management and audience lists.
- **Pricing:** Generous free tier (100/day); affordable scaling.
- **Compatibility:** Cloud (Centralized account). Standalone (Bring Your Own Key).

## 4. Payment Processing
- **Evaluated Tool:** Mercado Pago API
- **Problem Solved:** Provides essential local payment methods (Pix, OXXO, Pago Fácil) for LATAM merchants where Stripe is insufficient.
- **Benefits:** Trusted local checkout experience. High conversion rate in LATAM.
- **Risks:** Longer settlement times and less standardized API.
- **Pricing:** Standard transaction fees (varies by country).
- **Compatibility:** Cloud (Webhooks). Standalone (Requires webhook relay).

## 5. Shipping & Logistics
- **Evaluated Tool:** Shippo API
- **Problem Solved:** Automates live rate calculation and label generation natively, saving time for product sellers (like Maya).
- **Benefits:** One-click label printing natively inside OHC. Automated tracking notifications to customers.
- **Risks:** Customs declarations for international shipping can be complex to automate cleanly.
- **Pricing:** Free tier + nominal per-label fee.
- **Compatibility:** Cloud and Standalone supported.

## 6. SMS & Notifications
- **Evaluated Tool:** Twilio SMS API
- **Problem Solved:** Provides reliable native SMS alerts for time-sensitive orders (e.g., food pickup) where push notifications might be missed.
- **Benefits:** High open rates. Invisible to the user (simple settings toggle).
- **Risks:** A2P 10DLC compliance in the US requires business verification.
- **Pricing:** Pay-per-message.
- **Compatibility:** Cloud and Standalone supported.

## 7. Video Conferencing
- **Evaluated Tool:** Zoom API
- **Problem Solved:** Automates the creation and sharing of meeting links for online services (like tutoring).
- **Benefits:** Globally recognized platform. Removes manual link generation steps.
- **Risks:** Zoom requires an annual app review.
- **Pricing:** API is free but requires the merchant to have a Zoom account.
- **Compatibility:** Cloud (OAuth) and Standalone (Server-to-Server) supported.

## Conclusion
Actionable issue briefs have been created in `docs/research/` for all 7 tools evaluated, outlining the user problem, design, and implementation guidelines for the OHC platform.
