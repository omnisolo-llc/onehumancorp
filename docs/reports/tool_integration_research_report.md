# Tool Integration Research Report

## 1. Social Media Integration
**Tool:** Meta Business Suite / Graph API (Instagram, Facebook, WhatsApp)
**Problem Solved:** Centralizes DMs and comments so business owners like Maya (The Home Baker) don't miss sales across apps.
**Persona Value:** Extremely high. Core communication channels for small businesses.
**Advantages:** Direct, no third-party SaaS fees, maintains Radical Simplicity.
**Risks:** Complex OAuth and stringent API reviews by Meta.
**Pricing:** Mostly free API usage; WhatsApp may have per-conversation fees.
**Modes:** Works natively in Cloud; Standalone requires a lightweight cloud proxy/relay.

## 2. Calendar & Scheduling
**Tool:** Cal.com
**Problem Solved:** Eliminates back-and-forth emails for service providers like Leo (Music Tutor) trying to schedule appointments without double booking.
**Persona Value:** High. Simplifies the booking process for both owner and client.
**Advantages:** Open-source, highly customizable, white-label API.
**Risks:** Reliance on an external API for core scheduling logic.
**Pricing:** Team plans available.
**Modes:** Cloud (easy); Standalone (requires managing OAuth tokens locally).

## 3. Email Marketing
**Tool:** Mailchimp
**Problem Solved:** Allows users like Priya (Boutique Owner) to re-engage past customers easily without exporting/importing lists.
**Persona Value:** High for retention and marketing.
**Advantages:** Market leader, great API, tagging, high deliverability.
**Risks:** Strict anti-spam policies might suspend users with bad lists.
**Pricing:** Free tier up to 500 contacts, paid tiers start around $13/mo.
**Modes:** Cloud (OAuth); Standalone (API Key).

## 4. Payment Processing
**Tool:** Alipay
**Problem Solved:** Provides localized payment options for small business owners in markets where Stripe is unavailable or not preferred, preventing lost sales.
**Persona Value:** High for specific demographics (e.g., Chinese tourists, users in Asian markets).
**Advantages:** Huge user base, interoperable with other platforms in Asia.
**Risks:** Regulatory complexities when operating outside China.
**Pricing:** Varies by region and transaction type.
**Modes:** Cloud and Standalone compatible via respective integrations.

## 5. Shipping & Logistics
**Tool:** EasyPost
**Problem Solved:** Simplifies label generation and tracking for physical product merchants like Priya.
**Persona Value:** High time-saver.
**Advantages:** Unified API for 100+ carriers, competitive pricing, handles webhooks well.
**Risks:** Reliance on carrier APIs which can occasionally be slow or down.
**Pricing:** Free tier for low volume, pennies per label after.
**Modes:** Cloud and Standalone compatible via API.

## 6. SMS & Notifications
**Tool:** Twilio
**Problem Solved:** Ensures reliable notifications for busy workers like Fatima (Food Cart Operator) who might miss push notifications.
**Persona Value:** High for immediate operational awareness.
**Advantages:** Global coverage, incredibly reliable, programmable.
**Risks:** A2P 10DLC compliance in the US requires business registration, potentially tough for informal businesses.
**Pricing:** Pay-as-you-go (~$0.0079 per SMS in US).
**Modes:** Cloud (Centralized OHC Twilio account); Standalone (User provides API key).

## 7. Video Conferencing
**Tool:** Zoom
**Problem Solved:** Automates meeting link generation for online services like music lessons.
**Persona Value:** High. Reduces manual work and looks professional.
**Advantages:** Ubiquitous, standard OAuth process.
**Risks:** OAuth requires annual app review and compliance checks.
**Pricing:** API is free for Zoom users, but merchant needs an account.
**Modes:** Cloud (OAuth); Standalone (Server-to-Server OAuth).
