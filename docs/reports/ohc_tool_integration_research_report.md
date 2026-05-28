# OHC Tool Integration Research Report
## Executive Summary
This report evaluates seven critical tool integration categories designed to empower small business owners using the One Human Corp (OHC) platform. The focus is on tools that directly solve real-world pain points like customer communication, scheduling, and payments, rather than internal infrastructure. All proposed integrations align with the "User-First Lens" and are designed to abstract technical complexity away from the business owner.

## Evaluated Categories & Recommendations

| Category | Recommended Tool(s) | Priority | Key Benefit for SMBs |
| :--- | :--- | :--- | :--- |
| **Social Media Integration** | Meta Graph API, WhatsApp Cloud API | P0 | Unified inbox prevents missed sales across IG, FB, and WA. |
| **SMS & Notifications** | Twilio | P0 | Reliable, high-visibility transactional updates (orders, appointments). |
| **Calendar & Scheduling** | Google Calendar API | P1 | Eliminates manual scheduling back-and-forth; auto-generates links. |
| **Payment Processing** | Mercado Pago, Paytm | P1 | Captures lost sales by supporting preferred local payment methods. |
| **Shipping & Logistics** | Shippo / EasyPost | P1 | Saves hours of manual data entry by automating rates and labels. |
| **Email Marketing** | Resend | P2 | Simplifies sending basic promotions to customer lists without complex tools. |
| **Video Conferencing** | Jitsi (or Zoom/Meet) | P2 | Seamless one-click online consultations for service businesses. |

## Detailed Findings

### 1. Social Media Integration: The Unified Inbox (P0)
*   **The Problem:** Fragmented communication across WhatsApp, Instagram, and Facebook leads to delayed responses and lost revenue.
*   **The Solution:** Integrate Meta's APIs to pull all messages into a single OHC feed.
*   **Why Meta?** They dominate SMB communication. The OAuth flow is familiar.
*   **Risk:** Meta's API approvals can be strict, and API changes require maintenance.

### 2. SMS & Notifications (P0)
*   **The Problem:** Email open rates are low. Urgent updates (e.g., "Your order is ready") get missed.
*   **The Solution:** Automated SMS via Twilio for key lifecycle events.
*   **Why Twilio?** Global reach and reliability.
*   **Risk:** SMS costs can scale quickly. OHC needs a clear billing model or usage caps to prevent abuse or unexpected costs for the platform/user.

### 3. Calendar & Scheduling (P1)
*   **The Problem:** Manual scheduling is inefficient.
*   **The Solution:** Google Calendar sync for availability and auto-booking.
*   **Why Google?** Ubiquity. Most users already use Google Calendar.
*   **Risk:** Handling complex timezone edge cases and resolving calendar conflicts reliably.

### 4. Payment Processing (Local Alternatives) (P1)
*   **The Problem:** Global users need local payment methods (e.g., Mercado Pago in LATAM) to convert sales. Stripe is insufficient globally.
*   **The Solution:** Modular checkout supporting regional gateways.
*   **Risk:** Increased complexity in the checkout flow and order state management across multiple providers.

### 5. Shipping & Logistics (P1)
*   **The Problem:** Manual label creation is the biggest bottleneck for new e-commerce SMBs.
*   **The Solution:** Shippo or EasyPost for real-time rates and 1-click label generation.
*   **Risk:** Address validation failures and edge cases in international shipping.

### 6. Email Marketing (P2)
*   **The Problem:** Traditional tools (Mailchimp) are too complex for simple newsletters.
*   **The Solution:** Integrated bulk sending via Resend.
*   **Why Resend?** Excellent developer experience and generous free tier.
*   **Risk:** Managing spam complaints and ensuring high deliverability for our users.

### 7. Video Conferencing (P2)
*   **The Problem:** Manual link generation for online services is tedious.
*   **The Solution:** Auto-generated links via Jitsi (or Zoom/Meet).
*   **Why Jitsi?** Lowest friction (no account required for link generation).
*   **Risk:** Jitsi's free tier quality can vary compared to Zoom.

## Next Steps
1.  **Prioritize P0s:** Begin immediate technical design for the Unified Inbox (Meta APIs) and SMS Notifications (Twilio).
2.  **Review Issue Briefs:** Implementers should review the detailed issue briefs located in `docs/research/` for specific requirements and user-facing designs.
3.  **Prototype:** Build a proof-of-concept for the Meta Graph API connection flow to validate the developer experience and webhook reliability.
