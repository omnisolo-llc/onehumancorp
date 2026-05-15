# Scout Tool Integration Research Report

## Executive Summary
This report details the research and evaluation of potential tool integrations for the OHC platform across seven critical categories for small business owners: Social Media, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, and Video Conferencing.

The focus remains on solving real-world pain points for non-technical users, abstracting complexity where possible, and providing a unified business operating system.

## Evaluated Categories & Selected Tools

### 1. Social Media: TikTok Direct Messages
*   **Target Persona:** Retailers, creators, and service providers targeting Gen Z and Millennials.
*   **Problem Solved:** Centralizes customer inquiries and support requests originating from TikTok marketing efforts, preventing lost sales due to fragmented communication.
*   **Integration Approach:** OAuth connection pulling real-time webhooks into the OHC Unified Inbox.
*   **Detailed Brief:** `[social_media]_tiktok_dms.md`

### 2. Calendar & Scheduling: Acuity Scheduling
*   **Target Persona:** Service-based businesses (consultants, therapists, salons).
*   **Problem Solved:** Eliminates the back-and-forth of scheduling by offering self-serve booking links that sync automatically with the business owner's calendar.
*   **Integration Approach:** OAuth connection to sync appointment types and ingest webhooks for newly booked appointments, attaching them to OHC CRM profiles.
*   **Detailed Brief:** `[calendar]_acuity_scheduling.md`

### 3. Email Marketing: Brevo (formerly Sendinblue)
*   **Target Persona:** All businesses looking to engage their existing customer base with promotions or newsletters.
*   **Problem Solved:** Prevents data silos by allowing businesses to segment their OHC customer list and sync it directly to their email marketing platform.
*   **Integration Approach:** API Key/OAuth connection to sync selected CRM contacts to specific Brevo lists and retrieve high-level campaign metrics.
*   **Detailed Brief:** `[email_marketing]_brevo.md`

### 4. Payment Processing: Alipay
*   **Target Persona:** Businesses operating in the Chinese market, or international businesses serving Chinese tourists/expats.
*   **Problem Solved:** Enables acceptance of the dominant payment method for a massive demographic that relies on mobile wallets over credit cards.
*   **Integration Approach:** Merchant credential configuration in OHC settings, exposing Alipay as a checkout option and generating QR codes for mobile POS.
*   **Detailed Brief:** `[payment]_alipay.md`

### 5. Shipping & Logistics: ShipStation
*   **Target Persona:** E-commerce and product-based businesses.
*   **Problem Solved:** Automates the fulfillment process, eliminating manual data entry for rate comparison and label printing.
*   **Integration Approach:** API Key configuration to push OHC orders to ShipStation and receive tracking webhooks back to OHC.
*   **Detailed Brief:** `[shipping]_shipstation.md`

### 6. SMS & Notifications: MessageBird
*   **Target Persona:** Global businesses, or those serving demographics with lower email penetration or English proficiency.
*   **Problem Solved:** Provides reliable, direct communication via text messaging for reminders, updates, and promotions.
*   **Integration Approach:** Platform-level API configuration, exposing "SMS" as a channel in the OHC Unified Inbox.
*   **Detailed Brief:** `[sms]_messagebird.md`

### 7. Video Conferencing: Microsoft Teams
*   **Target Persona:** B2B service providers, consultants, and remote tutors.
*   **Problem Solved:** Automates the creation and distribution of video meeting links when appointments are booked.
*   **Integration Approach:** Microsoft Graph API via OAuth to generate `joinUrl`s attached to OHC calendar events.
*   **Detailed Brief:** `[video]_microsoft_teams.md`

## Next Steps
The implementer team should review the generated issue briefs located in `docs/research/` and prioritize development based on the provided P1/P2 ratings and overall platform roadmap. P1 priorities (TikTok DMs, Acuity, ShipStation, MessageBird) should be targeted for the immediate upcoming cycle.
