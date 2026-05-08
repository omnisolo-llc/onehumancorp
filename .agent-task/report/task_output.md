# OHC Tool Integration Research Report (Q4)

## Overview
This report evaluates potential third-party tool integrations for the OHC platform, focusing on solving real-world problems for non-technical small business owners.

### Social Media Integration (Meta APIs) (Meta Graph API (Instagram/Facebook/WhatsApp))
- **Problem Solved:** As a small business owner, I receive messages from customers on Instagram, Facebook, and WhatsApp, but managing them across different apps is overwhelming and I miss inquiries.
- **Key Advantages/Risks:** Meta provides the Graph API, which unifies access to Instagram Direct, Facebook Messenger, and WhatsApp Business. It requires a Business Manager account and app review. Pricing is based on WhatsApp conversation categories (service/marketing), while Messenger/IG are generally free. It can be complex to setup OAuth for non-technical users.
- **Pricing:** Free for Messenger/IG; WhatsApp is pay-per-conversation.
- **Mode Compatibility:** Cloud-friendly (OAuth). Standalone requires careful webhook tunneling (e.g., ngrok) or polling mechanisms.

### Calendar & Scheduling (Cal.com) (Cal.com)
- **Problem Solved:** I spend too much time going back and forth with clients trying to find a time to meet or schedule a service. Double bookings happen often.
- **Key Advantages/Risks:** Cal.com is an open-source scheduling tool that offers a robust API (v2) and supports self-hosting. It handles timezone resolution, calendar syncing (Google, Outlook), and integrates with video conferencing. It has a generous free tier and clear developer docs. It's highly customizable and brandable.
- **Pricing:** Free for individuals; Team plans start at $12/user/month.
- **Mode Compatibility:** Excellent for both. Can use their hosted API for Cloud, and run a self-hosted Cal.com instance alongside OHC Standalone.

### Email Marketing (MailerLite) (MailerLite)
- **Problem Solved:** I want to send newsletters and promotional offers to my customer list, but I don't know how to design them or manage my subscribers without paying for a complex enterprise tool.
- **Key Advantages/Risks:** MailerLite is exceptionally user-friendly for small businesses. It offers a great drag-and-drop editor, automation workflows, and high deliverability. The free tier supports up to 1,000 subscribers and 12,000 emails/month. The API is RESTful and straightforward.
- **Pricing:** Free up to 1K subscribers; Paid starts at $9/mo.
- **Mode Compatibility:** Works well for both, relying on standard API calls.

### Payment Processing (Mercado Pago) (Mercado Pago)
- **Problem Solved:** Stripe isn't widely used or supported for my customers in Latin America. I need to accept local payment methods like Pix in Brazil.
- **Key Advantages/Risks:** Mercado Pago is the leading payment gateway in LATAM, supporting local payment methods (Pix, Boleto, local credit cards). They offer Checkout Pro (hosted) and Transparent Checkout (API). It's critical for LATAM market penetration.
- **Pricing:** Varies by country, typically 3.99% - 4.99% + fixed fee per transaction.
- **Mode Compatibility:** Cloud handles webhooks easily. Standalone requires webhook proxying or polling for payment status.

### Shipping & Logistics (Shippo) (Shippo)
- **Problem Solved:** Calculating shipping costs and printing labels for my handmade products takes hours every week. I have to guess the shipping cost during checkout.
- **Key Advantages/Risks:** Shippo provides a single API to access rates and print labels for 85+ global carriers (USPS, UPS, FedEx, DHL). It offers discounted rates and is tailored for SMBs and e-commerce platforms. The API handles address validation, rating, and tracking.
- **Pricing:** Free tier available (pay for postage only); Pro tier starts at $19/mo.
- **Mode Compatibility:** Works seamlessly in both environments via standard REST API.

### SMS & Notifications (Twilio) (Twilio)
- **Problem Solved:** My customers often don't check their emails. I need to send them appointment reminders and order updates via text message so they actually see them.
- **Key Advantages/Risks:** Twilio is the industry standard for programmable SMS. It provides reliable global delivery, handles opt-outs automatically, and has extensive documentation. Registration for A2P 10DLC (US) can be a hurdle for small businesses but Twilio Trust Hub assists with this.
- **Pricing:** Pay-as-you-go, approx $0.0079 per SMS in the US + monthly phone number fee.
- **Mode Compatibility:** Works in both modes via standard API calls.

### Video Conferencing (Zoom) (Zoom)
- **Problem Solved:** When a client books an online consultation, I have to manually create a Zoom meeting and email them the link. It's tedious and error-prone.
- **Key Advantages/Risks:** Zoom's API allows automatic meeting creation. Authentication requires OAuth 2.0 (Server-to-Server for internal apps, or standard OAuth for user-facing integrations). It's globally recognized and trusted by consumers.
- **Pricing:** API access requires a Pro account or higher (starts at $15.99/mo).
- **Mode Compatibility:** Cloud utilizes standard OAuth. Standalone may require Server-to-Server OAuth configuration by the user.
