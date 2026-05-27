# Tool Integration Research Report

## 1. Social Media Integration
**Tool:** ManyChat
**Problem Solved:** Scattered messages across IG, FB, WhatsApp lead to slow response times for business owners like Maya (The Home Baker) who risk missing sales.
**Persona Value:** Extremely high. Core communication channels for small businesses.
**Advantages:** Highly rated by SMBs (G2), clear UI, provides a unified inbox pulling messages via API.
**Risks:** Complex pricing tiers scaling with contact counts.
**Pricing:** Free tier available (up to 1,000 contacts). Pro tier starts at $15/mo.
**Modes:** Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy/relay).

## 2. Calendar & Scheduling
**Tool:** Cal.com
**Problem Solved:** Eliminates back-and-forth emails for service providers like Leo (Music Tutor) trying to schedule appointments without double booking.
**Persona Value:** High. Simplifies the booking process for both owner and client.
**Advantages:** Open-source, highly customizable, white-label API. Self-serve booking links perfectly align with OHC.
**Risks:** Reliance on an external API for core scheduling logic.
**Pricing:** Team plans available. Great free tier for individuals.
**Modes:** Cloud (easy); Standalone (perfectly supports self-hosting, keeping excellent privacy).

## 3. Email Marketing
**Tool:** Brevo (formerly Sendinblue)
**Problem Solved:** Allows users like Priya (Boutique Owner) to re-engage past customers easily without violating spam laws or using complex tools.
**Persona Value:** High for retention and marketing.
**Advantages:** Generous free tier, easy drag-and-drop editor, strong API for one-way contact sync.
**Risks:** Stricter account approval processes for new accounts to prevent spam.
**Pricing:** Free tier up to 300 emails/day, paid tiers start around $25/mo.
**Modes:** Cloud (OAuth); Standalone (API Key).

## 4. Payment Processing
**Tool:** Mercado Pago
**Problem Solved:** Provides localized payment options for small business owners in markets (e.g., LATAM) where Stripe is unavailable or not preferred, preventing lost sales.
**Persona Value:** High for specific demographics (e.g., LATAM markets where local methods like Pix or OXXO dominate).
**Advantages:** Dominant in LATAM, standard API/webhook integration.
**Risks:** Regulatory complexities when operating outside of Latin America.
**Pricing:** Varies by region and transaction type.
**Modes:** Cloud and Standalone compatible via respective integrations.

## 5. Shipping & Logistics
**Tool:** Shippo
**Problem Solved:** Simplifies label generation and manual shipping rate calculations for physical product merchants like Priya.
**Persona Value:** High time-saver.
**Advantages:** Direct-from-dashboard label purchasing and printing, wide carrier support, pay-as-you-go pricing.
**Risks:** Reliance on carrier APIs which can occasionally be slow or down.
**Pricing:** Free tier for low volume (only pay for postage + 5¢ per label).
**Modes:** Cloud and Standalone compatible via API.

## 6. SMS & Notifications
**Tool:** Twilio
**Problem Solved:** Ensures reliable notifications for busy workers like Fatima (Food Cart Operator) who might miss emails or push notifications.
**Persona Value:** High for immediate operational awareness.
**Advantages:** Industry standard, incredibly reliable, programmable, cheap per-message cost.
**Risks:** A2P 10DLC compliance in the US requires business registration, potentially tough for informal businesses.
**Pricing:** Pay-as-you-go (~$0.0079 per SMS in US).
**Modes:** Cloud (Centralized OHC Twilio account); Standalone (User provides API key).

## 7. Video Conferencing
**Tool:** Google Meet
**Problem Solved:** Automates meeting link generation for online services like remote music lessons, eliminating manual creation of video links.
**Persona Value:** High. Reduces manual work and looks professional.
**Advantages:** Ubiquitous, free, zero friction for end customers. Auto-generation via Google Calendar API upon booking.
**Risks:** Requires Google Calendar connection.
**Pricing:** Free via Google Workspace API if using the user's existing Google Calendar/Meet integration.
**Modes:** Cloud (OAuth); Standalone (OAuth).
