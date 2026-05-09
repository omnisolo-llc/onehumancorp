# 🔍 Scout: Tool Integration Research Q4

## Executive Summary
This report details the evaluation of seven critical third-party tools across various categories that solve real problems for non-technical small business owners. Each tool was selected for its industry standing, ease of use, and compatibility with One Human Corp's (OHC) Hybrid Cloud/Standalone architecture.

The goal is to seamlessly integrate these capabilities into OHC so the business owner manages everything from a single, simple dashboard.

---

## 1. Social Media Integration: Hootsuite
*   **Problem:** Business owners lose track of DMs and comments across Instagram, Facebook, and TikTok, missing sales opportunities.
*   **Value to Persona:** Consolidates all messages into one unified OHC inbox. Owners don't need to juggle apps; they reply once in OHC and it routes to the correct social platform.
*   **Integration Risks:** Requires ongoing maintenance of API connections as social networks frequently change their Graph APIs.
*   **Pricing Estimate:** ~$99/month (Standard plan) for 10 social accounts.
*   **Hybrid Compatibility:** Yes. OHC Cloud handles webhooks, while Standalone instances poll or use a relay.

## 2. Calendar & Scheduling: Cal.com
*   **Problem:** Service businesses waste time with back-and-forth emails finding meeting times.
*   **Value to Persona:** Provides a professional booking link. Clients pick a time, and it automatically appears on the owner's OHC dashboard.
*   **Integration Risks:** Need robust two-way sync to prevent double-booking if the user modifies their calendar outside of OHC.
*   **Pricing Estimate:** Free tier available; Teams at $12/user/month.
*   **Hybrid Compatibility:** Yes. Open-source nature perfectly aligns with OHC's local-first capabilities.

## 3. Email Marketing: Mailchimp
*   **Problem:** Owners struggle to engage their existing customer list for repeat business without ending up in spam folders.
*   **Value to Persona:** Keeps customer lists perfectly synced. Owners can view open/click rates right on the OHC home screen.
*   **Integration Risks:** Strict compliance and opt-out rules must be enforced during synchronization.
*   **Pricing Estimate:** Free tier up to 250 contacts; Essentials from $13/month.
*   **Hybrid Compatibility:** Yes. API-based synchronization works in both modes.

## 4. Payment Processing: Stripe & Mercado Pago
*   **Problem:** Getting paid securely online is technically complex.
*   **Value to Persona:** One-click generation of payment links. Supports global cards (Stripe) and LATAM preferred methods (Mercado Pago).
*   **Integration Risks:** High security and PCI compliance requirements. Handling webhook failures gracefully is critical to ensure invoices are marked paid.
*   **Pricing Estimate:** Stripe (US): 2.9% + 30¢. Stripe (MX): 3.6% + 3 MXN. Mercado Pago varies but is comparable with local installment options.
*   **Hybrid Compatibility:** Yes. Standard cloud API integrations.

## 5. Shipping & Logistics: Shippo
*   **Problem:** Copy-pasting addresses and comparing shipping rates manually is a massive time sink for e-commerce owners.
*   **Value to Persona:** Automatically fetches the best rates and generates printable PDF labels directly from the OHC order view.
*   **Integration Risks:** Requires accurate dimension and weight data from the user to provide correct quotes.
*   **Pricing Estimate:** Free tier ($0.05/label using own account, or free using Shippo rates); Pro starts at $17/mo.
*   **Hybrid Compatibility:** Yes. REST API integrates smoothly.

## 6. SMS & Notifications: Twilio
*   **Problem:** Email open rates are low. Owners need to reach clients instantly for appointment reminders or urgent updates.
*   **Value to Persona:** Reliable, automated SMS delivery for reminders and marketing, managed entirely within OHC.
*   **Integration Risks:** Complex global regulatory landscape for SMS (e.g., 10DLC registration in the US).
*   **Pricing Estimate:** Pay-as-you-go, ~$0.0083/msg (US). Local phone numbers ~$1.15/mo.
*   **Hybrid Compatibility:** Yes. Backend service can be exposed to OHC's internal event bus in Cloud and Standalone modes.

## 7. Video Conferencing: Zoom
*   **Problem:** Creating and sharing meeting links manually for every online consultation is tedious and error-prone.
*   **Value to Persona:** Magical, automatic meeting link generation whenever a video call is booked, attached directly to client emails.
*   **Integration Risks:** OAuth token expiration and handling concurrent meetings correctly.
*   **Pricing Estimate:** Free tier (40-min limit); Pro at ~$15.99/user/month.
*   **Hybrid Compatibility:** Yes. Standard OAuth and API integration.

---
**Next Steps:**
- Prioritize integrations based on core user needs (P0: Cal.com, Twilio, Payment Processing).
- Assign Lead Implementers to begin technical discovery and proof-of-concept for the P0 tools.