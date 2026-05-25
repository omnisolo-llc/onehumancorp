# OHC Tool Integration Research Report

## Overview
This report evaluates third-party tools across various categories (Social Media, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, Video Conferencing) for integration into the OHC platform. The focus is exclusively on tools that solve real problems for non-technical small business owners, improving their operational efficiency and customer experience in both Cloud and Standalone environments.

## 1. Social Media Integration: Meta Business Suite API & WhatsApp Business API
*   **Problem Solved:** Business owners (like our target persona) often have to juggle multiple apps (Instagram, Facebook, WhatsApp) to reply to customer inquiries, leading to missed sales and slow response times.
*   **User Experience:** A unified "Inbox" tab inside OHC where all Facebook, Instagram DMs, and WhatsApp messages arrive. Owners can reply from one place without opening multiple apps.
*   **Advantages & Risks:**
    *   **Advantages:** Meta dominates the social commerce space. Unified API access simplifies development.
    *   **Risks:** Meta's OAuth and App Review process is notoriously strict and complex. Rate limits apply.
*   **Pricing Estimate:** Meta APIs are generally free for inbound messaging, but WhatsApp charges per conversation (around $0.015 - $0.08 depending on region).
*   **Environment Support:** Works in Cloud. Standalone requires the user to create their own Meta Developer App (high friction) or use a Cloud relay.

## 2. Calendar & Scheduling: Cal.com
*   **Problem Solved:** Back-and-forth messaging to find a suitable time for appointments, consultations, or lessons is frustrating and inefficient.
*   **User Experience:** A "Booking Page" feature in OHC where the owner sets their availability. Customers get a link to pick a time, which automatically syncs with the owner's Google or Outlook calendar.
*   **Advantages & Risks:**
    *   **Advantages:** Cal.com is open-source, highly customizable, and offers a robust API and webhook system. It supports routing and multi-person scheduling.
    *   **Risks:** Integrating a full scheduling engine can be complex. Calendar sync issues (timezone bugs) are common.
*   **Pricing Estimate:** Free for individuals. API access/white-labeling is tiered (Platform starts at $0.003/booking or fixed monthly).
*   **Environment Support:** Works perfectly in Cloud and Standalone (self-hostable).

## 3. Email Marketing: Resend
*   **Problem Solved:** Small businesses need to send newsletters, promotions, or transactional emails to their customer list but find tools like Mailchimp too bloated and expensive.
*   **User Experience:** An "Email Campaigns" tab in OHC. The owner selects a list of customers, types a simple email using a rich text editor, and clicks send.
*   **Advantages & Risks:**
    *   **Advantages:** Developer-first API, excellent deliverability, modern React Email templates. Very easy to integrate.
    *   **Risks:** Strict spam compliance rules. Business owners might need domain verification (DNS records), which is hard for non-technical users.
*   **Pricing Estimate:** Free up to 3,000 emails/month. $20/mo for 50,000 emails.
*   **Environment Support:** Cloud only (requires API key). Standalone users must provide their own API key.

## 4. Payment Processing: Mercado Pago (LATAM Focus)
*   **Problem Solved:** Stripe is not available or preferred everywhere. In regions like LATAM, local solutions offering local payment methods (like Pix in Brazil) are essential.
*   **User Experience:** In the "Payments" setup, an option to connect Mercado Pago. When a customer checks out, they see familiar local payment options.
*   **Advantages & Risks:**
    *   **Advantages:** Dominates the LATAM market. Supports installments and local bank transfers.
    *   **Risks:** API documentation can be fragmented. Dispute resolution is different from US providers.
*   **Pricing Estimate:** Varies by country, typically ~3.99% to 4.99% + fixed fee per transaction.
*   **Environment Support:** Cloud and Standalone (uses standard OAuth/API keys).

## 5. Shipping & Logistics: Shippo
*   **Problem Solved:** Calculating shipping rates manually and going to the post office to buy labels wastes hours of the business owner's time.
*   **User Experience:** When an order is placed, the owner clicks "Generate Label" in OHC. They enter box dimensions and print a discounted shipping label instantly.
*   **Advantages & Risks:**
    *   **Advantages:** Connects to 85+ carriers globally. Excellent API for rate calculation and label generation.
    *   **Risks:** Label generation requires precise weight/dimension data, which users often get wrong, leading to adjustments/fees.
*   **Pricing Estimate:** $0.05 per label (Pay As You Go tier) or volume-based subscriptions.
*   **Environment Support:** Cloud and Standalone (API driven).

## 6. SMS & Notifications: Twilio
*   **Problem Solved:** Customers (and sometimes the business owner) miss important updates (order ready, appointment reminder) if they don't check email. SMS is immediate.
*   **User Experience:** A toggle in settings: "Send SMS reminders to customers." Owners can also buy a dedicated business phone number to send/receive texts via the OHC inbox.
*   **Advantages & Risks:**
    *   **Advantages:** Global reach, highly reliable, supports WhatsApp via the same API.
    *   **Risks:** Complex regulatory compliance (A2P 10DLC in the US). Fraud prevention is crucial.
*   **Pricing Estimate:** ~$0.0079 per SMS sent/received (US pricing).
*   **Environment Support:** Cloud and Standalone (API driven).

## 7. Video Conferencing: Zoom API
*   **Problem Solved:** For online service providers (tutors, consultants), manually creating and sending a video link for every booking is tedious.
*   **User Experience:** When a customer books an online service, a Zoom link is automatically generated and added to the calendar invite and confirmation email.
*   **Advantages & Risks:**
    *   **Advantages:** Ubiquitous, everyone knows how to use it.
    *   **Risks:** Zoom OAuth flow can be confusing. Requires the user to have a Zoom account.
*   **Pricing Estimate:** API access is included with Zoom Pro ($14.99/mo).
*   **Environment Support:** Cloud and Standalone (requires OAuth).

## Conclusion
The highest priority integration for immediate impact on daily operations is **Social Media Integration (Meta APIs)** for unified messaging, followed closely by **Calendar & Scheduling (Cal.com)** to eliminate booking friction.
