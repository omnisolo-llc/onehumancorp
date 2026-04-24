# Research Report: Tool Integration for OneHumanCorp

This report summarizes the research and evaluation of 7 tool categories identified to expand OHC's capabilities for small business owners. Detailed issue briefs for each integration have been generated and stored in the `docs/research/` directory.

## 1. Social Media Integration
- **Selected Tool:** Meta Graph API & TikTok API
- **Problem Solved:** Unified inbox for managing cross-platform inquiries (Instagram, Facebook, WhatsApp, TikTok).
- **Why Chosen:** Direct integration avoids per-message aggregator costs (like MessageBird), preserving OHC's free tier viability.
- **Detailed Brief:** `docs/research/[social_media]_unified_inbox.md`

## 2. Calendar & Scheduling
- **Selected Tool:** Cal.com
- **Problem Solved:** Robust scheduling infrastructure to prevent double bookings across personal calendars (Google/Outlook).
- **Why Chosen:** Open-source core, extremely developer-friendly APIs, easily embeddable into the OHC platform.
- **Detailed Brief:** `docs/research/[calendar_scheduling]_cal_com_integration.md`

## 3. Email Marketing
- **Selected Tool:** Resend
- **Problem Solved:** Automated email campaigns driven by the Marketing AI Agent to customer lists.
- **Why Chosen:** Exceptional programmatic templating via React Email and modern API architecture.
- **Detailed Brief:** `docs/research/[email_marketing]_resend_integration.md`

## 4. Payment Processing
- **Selected Tool:** Mercado Pago
- **Problem Solved:** Expand global payment support beyond Stripe, particularly in LATAM for unbanked local payment methods (PIX, OXXO).
- **Why Chosen:** The dominant player in LATAM, high consumer trust, and comprehensive checkout methods.
- **Detailed Brief:** `docs/research/[payment_processing]_global_payments_mercado_pago.md`

## 5. Shipping & Logistics
- **Selected Tool:** EasyPost
- **Problem Solved:** Real-time shipping rate calculation at checkout and one-click label printing for physical goods.
- **Why Chosen:** Aggregates 100+ carriers via a single unified API, reliable and developer-friendly.
- **Detailed Brief:** `docs/research/[shipping_logistics]_easypost_integration.md`

## 6. SMS & Notifications
- **Selected Tool:** Twilio
- **Problem Solved:** Critical transactional SMS notifications for users with low internet connectivity or app notification fatigue.
- **Why Chosen:** Industry-leading global carrier coverage and unparalleled reliability.
- **Detailed Brief:** `docs/research/[sms_notifications]_twilio_integration.md`

## 7. Video Conferencing
- **Selected Tool:** Zoom API
- **Problem Solved:** Auto-generation of meeting links for online service bookings (e.g., tutoring, consulting).
- **Why Chosen:** Broadest consumer trust and deep API integration capabilities for creating unique session URLs.
- **Detailed Brief:** `docs/research/[video_conferencing]_zoom_integration.md`

## Next Steps
- Triage the generated issue briefs according to priority (P0 for Unified Inbox and SMS).
- Proceed with implementation spikes based on the provided Integration Prompts in each brief.
