# Scout: Tool Integration Research Report

## Executive Summary
This report details the research and evaluation of seven third-party tool integrations designed to empower small business owners using the OHC platform. The evaluations were conducted with a strict "User-First" lens, ensuring that all proposed integrations focus on simplicity, ease of use, and native experiences over technical complexity. These tools address core operational pain points across social media, scheduling, marketing, payments, shipping, notifications, and video conferencing.

---

## 1. Social Media Integration: Manychat
**Problem Solved**: Small business owners miss sales inquiries in Instagram/Facebook DMs and need automated, unified inbox routing.
**User Experience**: Merchants connect via OAuth in the OHC dashboard. DMs are pulled into the OHC unified inbox. Default quick-replies (store hours, links) are auto-provisioned. The Customer Service AI agent assists with complex queries.
**Advantages**: Industry leader in chat automation; highly reliable.
**Risks**: Requires an extra OAuth onboarding step; subject to Meta's API policy changes.
**Pricing**: Free tier up to 1,000 contacts; Pro starts at $15/month.
**Mode Compatibility**: Fully compatible with both Cloud and Standalone modes.

---

## 2. Calendar & Scheduling: Calendly
**Problem Solved**: Service-based businesses waste time emailing back-and-forth to schedule appointments.
**User Experience**: Merchants connect Calendly via OAuth. OHC imports event types. Customers book via a seamlessly embedded widget on the OHC storefront. Calendly handles conflict resolution, and OHC logs the appointment internally.
**Advantages**: Ubiquitous standard for scheduling; excellent conflict resolution out-of-the-box.
**Risks**: Free tier is limited to one event type, which may frustrate advanced users.
**Pricing**: Basic tier is free; Standard tier is $10/month.
**Mode Compatibility**: Fully compatible with both Cloud and Standalone modes.

---

## 3. Email Marketing: Brevo (formerly Sendinblue)
**Problem Solved**: Merchants need an easy way to send promotional newsletters to retain customers without using complex external CRM tools.
**User Experience**: From the OHC Marketing tab, merchants request an email campaign. The Marketing AI Agent generates the content. OHC syncs the customer list and sends the approved email via Brevo.
**Advantages**: Very generous free tier (300 emails/day), ideal for our SMB target market.
**Risks**: Domain authentication is required for high deliverability, which can be technically daunting for SMBs.
**Pricing**: Free tier available; Starter plan is $25/month for 20k emails.
**Mode Compatibility**: Works well in Cloud mode (centralized IPs) and Standalone (user API key).

---

## 4. Payment Processing: Paystack
**Problem Solved**: African merchants cannot use Stripe and need a trusted gateway that supports local methods like Mobile Money and USSD.
**User Experience**: Merchants in supported regions connect Paystack. Customers see a native "Pay with Paystack" option at checkout, utilizing local payment methods via an inline modal. Webhooks handle instant order fulfillment.
**Advantages**: Deeply trusted in Africa; excellent support for localized payment methods.
**Risks**: Cross-border settlement complexities and currency conversion rates.
**Pricing**: Transaction-based (e.g., 1.5% + NGN 100 locally); no monthly fees.
**Mode Compatibility**: Compatible with both Cloud and Standalone modes.

---

## 5. Shipping & Logistics: Sendle
**Problem Solved**: Sellers of physical goods struggle with calculating shipping rates and generating labels efficiently.
**User Experience**: Merchants configure basic product dimensions. At checkout, OHC provides real-time Sendle quotes. In the dashboard, merchants click one button to purchase a label, print it, and auto-email tracking info.
**Advantages**: 100% carbon-neutral; flat rates tailored specifically for small businesses.
**Risks**: Carrier network may be less expansive globally than major aggregators like Shippo or EasyPost.
**Pricing**: Pay per label; no subscription fees.
**Mode Compatibility**: Compatible with both Cloud and Standalone modes.

---

## 6. SMS & Notifications: Vonage
**Problem Solved**: Mobile-first merchants (e.g., food carts) miss app notifications and need immediate SMS alerts for new orders.
**User Experience**: Merchant verifies phone number in Settings. OHC routes order and booking notifications directly to their phone via SMS. AI determines optimal dispatch times.
**Advantages**: Exceptional global reach and competitive international pricing compared to Twilio.
**Risks**: Strict global compliance laws (e.g., US 10DLC) add friction to the setup process.
**Pricing**: Pay-per-message depending on destination.
**Mode Compatibility**: Cloud mode handles compliance centrally; Standalone requires the user to manage their own API keys and compliance.

---

## 7. Video Conferencing: Zoom
**Problem Solved**: Tutors and consultants manually generate and email meeting links for every booking.
**User Experience**: Merchant connects Zoom via OAuth. When an "Online Meeting" is booked, OHC automatically generates a unique Zoom link and embeds it in calendar invites and confirmation emails for both parties.
**Advantages**: The most recognizable brand in video conferencing; frictionless experience for clients.
**Risks**: Requires a stringent app review process for the OHC platform; merchants are bound by their personal Zoom tier limits.
**Pricing**: API is free, but merchants need their own Zoom account (free or paid).
**Mode Compatibility**: Fully compatible with both Cloud and Standalone modes.

---

## Proposed Next Steps
1. Review these 7 integrated briefs and prioritize implementation based on engineering bandwidth. Manychat (Social) and Sendle (Shipping) represent the highest immediate value for our core retail/boutique personas.
2. Initiate the App Directory registration process for Zoom and Calendly, as their review processes take the longest.
3. Proceed with detailed technical implementation designs for the P1 items.
