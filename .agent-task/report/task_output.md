# Tool Integration Research Report Q2

## Overview
This report evaluates seven essential tools aimed at solving real-world problems for small business owners in the realms of scheduling, messaging, shipping, social media, email marketing, payment processing, and video conferencing. The focus is strictly on the non-technical user experience, ensuring that features pass the "Grandmother Test." All tools were evaluated for compatibility with OHC's Hybrid architecture (Cloud and Standalone).

---

## 1. Calendar & Scheduling: Cal.com
**Problem Solved:** Eliminates back-and-forth scheduling emails, preventing missed meetings and lost revenue for service-based businesses.
*   **Ease of Use:** Exceptional. Non-technical users simply share a generated booking link. The UI is modern and straightforward.
*   **Pricing:** Free for individual use. $12/month per user for teams.
*   **Environment:** As an open-source platform, it natively supports both Cloud and Standalone environments. Docker self-hosting makes it a perfect fit for OHC Standalone.
*   **AI Integration:** Features "Cal.ai," an AI scheduling assistant, aligning perfectly with OHC's agentic vision.
*   **Integration Approach:** OHC will connect via OAuth and listen to webhooks to sync events into the unified inbox.

---

## 2. SMS & Notifications: Twilio
**Problem Solved:** Ensures customers receive critical updates (like order confirmations) reliably, especially targeting demographics with low email open rates or limited English proficiency.
*   **Ease of Use:** The underlying API is highly technical, but the business owner's experience via OHC will be seamless. They will simply purchase a number within OHC and toggle notification rules.
*   **Pricing:** Pay-as-you-go. Extremely cost-effective (fractions of a cent per SMS in the US) plus small monthly fees for leased numbers.
*   **Environment:** Cloud-first via REST APIs. Works in Standalone mode as long as the local instance has outbound internet access and can process webhooks/polling for replies.
*   **AI Integration:** Offers Conversation Intelligence and seamless hooks for our AI agents to auto-respond to incoming text messages.
*   **Integration Approach:** OHC will use the Programmable Messaging API for dispatch and handle webhooks to route customer replies into the unified inbox.

---

## 3. Shipping & Logistics: Shippo
**Problem Solved:** Simplifies the complex, manual process of calculating shipping rates and printing labels, saving physical goods sellers hours of administrative work.
*   **Ease of Use:** Very high. Business owners just input box dimensions/weight and click "Print Label" to get deeply discounted carrier rates.
*   **Pricing:** API Starter plan has no monthly fees. First 30 labels are free, then 7¢ per label. Address validation is highly affordable (2¢ domestic, 8¢ international).
*   **Environment:** API-first. Works perfectly in Cloud. Functions in Standalone mode provided there is an internet connection to reach Shippo's rating and transaction endpoints.
*   **AI Integration:** Uses AI for generating "Estimated Delivery Dates," directly improving the customer experience.
*   **Integration Approach:** OHC will integrate the Rating API for checkout/fulfillment screens and the Transaction API to generate printable PDF labels directly in the owner's dashboard.

---

## 4. Social Media Integration: Ayrshare
**Problem Solved:** Enables business owners to schedule posts and manage their presence across multiple social networks without logging into separate apps.
*   **Ease of Use:** Simplifies the OAuth nightmare. Owners connect their accounts once in OHC and schedule content easily.
*   **Pricing:** Designed for platforms. $599/month for the first 30 user profiles, dropping significantly at higher volumes.
*   **Environment:** Cloud API. Works natively in OHC Cloud. Standalone instances require outbound internet for API calls.
*   **AI Integration:** Offers a "Max Pack" with AI tools for content generation and analysis.
*   **Integration Approach:** Implement the Social Media Messaging API for publishing and configure webhooks to receive comments into the unified inbox.

---

## 5. Email Marketing: Resend
**Problem Solved:** Allows business owners to send reliable newsletters, promotions, and transactional receipts without wrestling with complex legacy marketing tools or deliverability issues.
*   **Ease of Use:** Built for developers, but enables OHC to provide a dead-simple UI (like a WYSIWYG editor) while handling the complex email delivery backend silently.
*   **Pricing:** Very generous free tier (3,000 emails/mo). Pro tier is $20/mo for 50,000 emails.
*   **Environment:** Cloud API. Seamless in OHC Cloud. Functions in Standalone via outbound REST calls.
*   **AI Integration:** Provides AI Assistant capabilities, synergizing well with OHC agents generating email copy.
*   **Integration Approach:** Integrate the Email API for dispatching and webhooks for tracking open/click rates.

---

## 6. Payment Processing: Stripe
**Problem Solved:** Provides a fast, secure, and globally recognized way for businesses to accept online payments (credit cards, digital wallets) without the hassle of traditional merchant accounts.
*   **Ease of Use:** Frictionless checkout for end-customers. Simple onboarding for the business owner via Stripe Connect.
*   **Pricing:** Pay-as-you-go processing fees (e.g., 2.9% + 30¢). No monthly or setup fees.
*   **Environment:** Cloud API. Works in OHC Cloud and Standalone (requires outbound access to Stripe APIs).
*   **AI Integration:** Utilizes AI (Stripe Radar) to detect and block fraudulent transactions.
*   **Integration Approach:** Use Stripe Connect for merchant onboarding and Stripe Checkout/Elements for the payment flow.

---

## 7. Video Conferencing: Whereby
**Problem Solved:** Facilitates virtual consultations and tutoring without requiring clients to download software or navigate complex calendar links.
*   **Ease of Use:** The "Embedded" product allows 1-click video calls directly inside the browser, offering the lowest possible friction for both owner and client.
*   **Pricing:** Embedded "Build" plan is $9.99/mo (includes 2,000 participant minutes), plus $0.004 per additional minute.
*   **Environment:** Cloud API. Works perfectly in OHC Cloud and Standalone (requires outbound access to generate URLs).
*   **AI Integration:** Offers session transcriptions and live captions which can be fed into OHC agents.
*   **Integration Approach:** Use the Embedded API to generate ephemeral room URLs and display them in an iframe within the OHC dashboard.

---

## Next Steps
- Implementers should refer to the issue briefs in `docs/research/` to begin API integration design.
- Prioritize Twilio (P0) and Stripe (P0) due to their critical necessity for customer communication and revenue generation, respectively.
