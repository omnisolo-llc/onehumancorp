# OHC Tool Integration Research Report (Q4)

## Executive Summary
This research identifies and evaluates seven third-party tool integrations designed to solve real-world pain points for small business owners using OHC. The focus remains strictly on the "User-First Lens" — evaluating tools based on their ability to save time, increase sales, or reduce operational friction for non-technical users.

All selected tools support OHC's dual-environment architecture (Cloud and Standalone), primarily via standard REST APIs, webhooks, or open-source self-hosting capabilities.

## Methodology
Research was conducted across seven distinct operational categories essential for small business success: Social Media Integration, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, and Video Conferencing. Tools were evaluated against the following criteria:
1. **Ease of Use**: Can a non-technical user configure and benefit from it?
2. **Pricing**: Is it affordable on a small business budget (preferably with a free tier)?
3. **Reputation**: Is the tool reliable and trusted by the market?
4. **Environment Compatibility**: Does it function effectively in both OHC Cloud and Standalone deployments?

## Summary of Findings

| Category | Recommended Tool | Priority | Scope | Key Benefit for Business Owner |
| :--- | :--- | :--- | :--- | :--- |
| **Calendar & Scheduling** | **Cal.com** | P0 | Small | Eliminates back-and-forth scheduling via a simple booking link. |
| **Social Media Inbox** | **Manychat** | P1 | Medium | Consolidates Instagram, Facebook, and WhatsApp into one inbox. |
| **Payment Processing** | **Mercado Pago** | P1 | Large | Enables local payment methods (Pix, OXXO) for LATAM markets. |
| **Shipping & Logistics** | **EasyPost** | P1 | Medium | Automates label purchasing and tracking from the OHC dashboard. |
| **SMS Notifications** | **Twilio** | P1 | Medium | Reliable order/appointment SMS alerts for low-email customers. |
| **Email Marketing** | **Loops.so** | P2 | Medium | Dead-simple newsletter sending without the bloat of Mailchimp. |
| **Video Conferencing** | **Zoom** | P2 | Medium | Auto-generates meeting links for virtual consultations. |

## Detailed Insights & Recommendations

### 1. Calendar & Scheduling: Cal.com (P0)
**Why it matters**: Scheduling is the biggest time-sink for service-based businesses.
**Why Cal.com**: It is an open-source alternative to Calendly. It is highly respected, has a generous free tier for individuals, and critically, its open-source nature means it aligns perfectly with OHC's Standalone/Local deployment philosophy.

### 2. Social Media Integration: Manychat (P1)
**Why it matters**: Modern commerce happens in DMs. Missing a DM often means missing a sale.
**Why Manychat**: It natively handles the complex Graph APIs for Facebook/Instagram and WhatsApp Business API. By proxying this through Manychat, OHC avoids building complex, shifting social media API integrations directly.

### 3. Payment Processing: Mercado Pago (P1)
**Why it matters**: Stripe is excellent but lacks deep penetration in LATAM.
**Why Mercado Pago**: It is the undisputed leader in Latin America. Supporting it natively in OHC opens the platform to massive markets in Brazil, Mexico, and Argentina where alternative payment methods (APMs) dominate over traditional credit cards.

### 4. Shipping & Logistics: EasyPost (P1)
**Why it matters**: Copy-pasting addresses into carrier websites is slow and error-prone.
**Why EasyPost**: It abstracts away hundreds of carrier APIs into one unified interface. The developer tier is practically free for small volumes, making it an invisible but powerful engine for OHC merchants.

### 5. SMS & Notifications: Twilio (P1)
**Why it matters**: Email open rates are dropping, and many blue-collar or older customers prefer SMS.
**Why Twilio**: It is the gold standard for SMS delivery. The integration is simple, and the pay-as-you-go model ensures merchants only pay exactly for what they send.

### 6. Email Marketing: Loops.so (P2)
**Why it matters**: Retaining existing customers via email is cheaper than acquiring new ones.
**Why Loops.so**: Traditional tools (Mailchimp, Klaviyo) are intimidating for beginners. Loops focuses on plain-text, high-conversion emails with zero learning curve.

### 7. Video Conferencing: Zoom (P2)
**Why it matters**: Manual link generation is tedious.
**Why Zoom**: It remains the verb for video calls. Automating link generation for booked appointments creates a professional, seamless experience for the end customer.

## Next Steps
Actionable issue briefs have been generated for each of the tools above and stored in the `docs/research/` directory. Implementers can pick these up based on the assigned priority queue.

- [x] Research completed
- [x] Issue Briefs generated
- [x] Ready for implementation handoff
