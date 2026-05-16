# Tool Integration Research Report

**Prepared by:** Principal Integrations Engineer (L7)
**Mission:** Expand OHC's capabilities by discovering and evaluating tools that solve real problems for small business owners in Cloud and Standalone environments.

## Executive Summary
This report details the evaluation of seven categories of tools critical for small business owners: Social Media Integration, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, and Video Conferencing. For each category, a primary tool was selected based on ease of use for non-technical users, pricing, reliability, and capability to function in both Cloud and Standalone modes.

Detailed issue briefs have been created in `docs/research/` for each selected tool, outlining the problem statement, research findings, design document, and implementation prompt.

---

## 1. Social Media Integration
**Selected Tool:** Manychat
**Issue Brief:** `docs/research/[social_media]_manychat.md`

**Evaluation:**
Manychat offers robust integrations with Instagram, Facebook Messenger, and WhatsApp. It is user-friendly and features a generous free tier. The primary benefit for OHC users is the ability to aggregate messages from multiple platforms into a single unified inbox, reducing the friction of managing multiple apps and preventing lost sales.

**Integration Risks:**
While Cloud integration is straightforward via webhooks, Standalone mode requires either a public tunneling service (e.g., ngrok) or polling, which introduces technical complexity.

---

## 2. Calendar & Scheduling
**Selected Tool:** Calendly
**Issue Brief:** `docs/research/[calendar]_calendly.md`

**Evaluation:**
Calendly is the undisputed leader in automated scheduling. It offers a very capable free tier and intuitive setup. Integrating Calendly will allow business owners to share booking links that automatically sync with their availability, eliminating back-and-forth emails.

**Integration Risks:**
Similar to Manychat, Standalone mode requires a polling service or user-managed API keys for two-way data sync, whereas Cloud mode seamlessly handles webhooks. Embedding the iframe works well in both modes.

---

## 3. Email Marketing
**Selected Tool:** Mailchimp
**Issue Brief:** `docs/research/[email]_mailchimp.md`

**Evaluation:**
Mailchimp provides an excellent drag-and-drop builder and high deliverability rates. Its free tier (up to 500 contacts) is perfect for micro-businesses. By syncing OHC customer lists with Mailchimp, users can effortlessly manage marketing campaigns.

**Integration Risks:**
The API is robust, but managing OAuth flow and maintaining sync state across distributed Standalone instances requires careful handling of API rate limits and token expirations.

---

## 4. Payment Processing (LATAM Focus)
**Selected Tool:** Mercado Pago
**Issue Brief:** `docs/research/[payment]_mercadopago.md`

**Evaluation:**
Mercado Pago is essential for LATAM markets where Stripe is less dominant. It supports local payment methods (e.g., Pix, OXXO) and installments. There are no monthly fees, and the familiar Checkout Pro experience boosts conversion rates.

**Integration Risks:**
Handling IPN (Instant Payment Notification) webhooks in Standalone mode requires public endpoints. OHC will need to ensure secure and reliable processing of these webhooks to mark invoices as paid accurately.

---

## 5. Shipping & Logistics
**Selected Tool:** EasyPost
**Issue Brief:** `docs/research/[shipping]_easypost.md`

**Evaluation:**
EasyPost offers a unified API for numerous carriers and a generous free tier (120,000 free shipments/year). It streamlines label generation and tracking directly from the OHC dashboard, saving significant time for e-commerce and craft businesses.

**Integration Risks:**
Requires managing potentially complex logic for fetching live rates based on dimensions and addresses. In Standalone mode, users must manage their own EasyPost API keys, which may be a slight hurdle for completely non-technical users.

---

## 6. SMS & Notifications
**Selected Tool:** Twilio
**Issue Brief:** `docs/research/[sms]_twilio.md`

**Evaluation:**
Twilio provides highly reliable, pay-as-you-go SMS capabilities. This is critical for reducing no-shows (via appointment reminders) and communicating with customers who may not check email regularly.

**Integration Risks:**
Compliance with local regulations (e.g., A2P 10DLC in the US) can be complex. OHC must guide users through the necessary registration steps or abstract them where possible. In Standalone mode, users manage their own Twilio credentials.

---

## 7. Video Conferencing
**Selected Tool:** Zoom
**Issue Brief:** `docs/research/[video]_zoom.md`

**Evaluation:**
Zoom is universally recognized and offers a free basic tier. Automating the generation of unique meeting links for virtual appointments saves time and reduces administrative overhead for coaches, tutors, and consultants.

**Integration Risks:**
Server-to-Server OAuth works well in the Cloud, but Standalone mode may require users to create their own Zoom app credentials, which is complex. Supporting static personal meeting links as a fallback for Standalone users is recommended.

---

## Next Steps
1. **Review and Prioritize:** The Product and Engineering teams should review these issue briefs. Calendly (P0) and Mailchimp (P1) and Manychat (P1) and Twilio (P1) should be prioritized for near-term implementation.
2. **Technical Spikes:** Implementers should begin technical spikes on handling webhooks in Standalone mode, as this is the primary shared risk across several integrations (Manychat, Calendly, Mercado Pago).
3. **Implementation:** Proceed with implementing the features according to the provided issue briefs, ensuring a user-first experience.