# 🔍 Scout: Tool Integration Research [Q2 2024]

## Executive Summary
This research mission focused on expanding OHC's capabilities by evaluating third-party tools that solve real problems for small business owners in both Cloud and Standalone environments. We prioritized tools that adhere to OHC's **Radical Simplicity** and **No Jargon** principles, focusing on ease of use, transparent pricing, and global applicability.

---

## 1. Social Media Integration: WhatsApp Business API
### Problem Statement
Fatima (Food Cart Operator) and Maya (Home Baker) do most of their business over WhatsApp. Managing orders and answering questions in a separate app is tedious. They need WhatsApp messages to flow directly into their OHC unified inbox.

### Research Report
- **Tool**: WhatsApp Business API
- **Evaluation**: The primary communication channel for SMBs in LATAM, India, and SE Asia.
- **Ease of Use**: High (post-verification).
- **Pricing**: Free for the first 1,000 service conversations/month; then per-conversation billing.
- **Reputation**: The global gold standard for personal/business chat.
- **Compatibility**: Cloud (Centralized), Standalone (Proxy/API Key).

### Design Doc
- OAuth-based connection via Meta in the OHC Operations dashboard.
- Unified Inbox integration where "The Ambassador" (Customer Success Agent) drafts replies and processes orders.

### Implementation Prompt
Implement WhatsApp Business API integration into the OHC Unified Inbox. Enable the Customer Success agent to draft and send replies based on the business catalog and FAQs.

---

## 2. Email Marketing: Loops.so
### Problem Statement
Priya (Boutique Owner) finds Mailchimp too complex. She needs a "no-nonsense" tool to send beautiful newsletters and product updates to her customers without learning a spaceship cockpit.

### Research Report
- **Tool**: Loops.so
- **Evaluation**: Built for simplicity and clean design. Focuses on the essentials.
- **Ease of Use**: Extremely high; intuitive editor.
- **Pricing**: Free for up to 1,000 subscribers and 2,000 emails/month.
- **Reputation**: High; known for being the "Linear" of email marketing.
- **Compatibility**: Both Cloud and Standalone (API Key).

### Design Doc
- API-based connection in the Marketing tab.
- Automatic audience synchronization from OHC orders.
- "The Promoter" agent drafts and sends campaigns directly from OHC.

### Implementation Prompt
Integrate Loops.so for native email marketing. Implement audience sync and a simplified campaign trigger for the Marketing agent.

---

## 3. Payment Processing: Razorpay (India Focus)
### Problem Statement
Ananya (Boutique Owner in Bangalore) needs to accept UPI and local credit cards. She requires a trusted local gateway that her customers are familiar with to reduce cart abandonment.

### Research Report
- **Tool**: Razorpay
- **Evaluation**: Leading payment gateway in India; deep UPI integration.
- **Ease of Use**: High; excellent onboarding for Indian merchants.
- **Pricing**: ~2% per domestic transaction; no monthly fees.
- **Reputation**: Market leader in India.
- **Compatibility**: Cloud (Managed), Standalone (API Key).

### Design Doc
- Regional payment provider option during setup.
- Native checkout widget supporting UPI deep links and local cards.
- Normalized reporting in "The Accountant" dashboard.

### Implementation Prompt
Add Razorpay as a native payment provider for the Indian market, focusing on a seamless UPI-first checkout experience.

---

## 4. Payment Processing: Paytm (India Focus)
### Problem Statement
Small Indian vendors want to offer the familiar "Paytm" experience, allowing customers to pay quickly using their Paytm wallet or linked UPI bank accounts.

### Research Report
- **Tool**: Paytm Payment Gateway
- **Evaluation**: Ubiquitous in India; massive brand trust.
- **Ease of Use**: Seamless for customers with the Paytm app.
- **Pricing**: ~0% for UPI; competitive rates for other methods.
- **Reputation**: Top-tier trust in India.
- **Compatibility**: Both Cloud and Standalone.

### Design Doc
- "Pay with Paytm" button at checkout.
- Support for dynamic UPI QR codes and app deep linking.

### Implementation Prompt
Integrate Paytm Payment Gateway to provide a UPI-first payment experience, supporting app redirects and dynamic QR code generation.

---

## 5. Video Conferencing: Whereby
### Problem Statement
Leo (Music Tutor) needs a simple way to meet students online. He wants a "browser-first" experience where students just click a link and join immediately without app downloads or account creation.

### Research Report
- **Tool**: Whereby
- **Evaluation**: Premium, high-quality, browser-based video meetings.
- **Ease of Use**: Highest in class; no downloads required for guests.
- **Pricing**: Free for 1-on-1 meetings; affordable Pro plans.
- **Reputation**: Excellent; focused on privacy and simplicity.
- **Compatibility**: Cloud (API), Standalone (Permanent Links).

### Design Doc
- Automatic room generation for "Online" service bookings.
- Links embedded in confirmation emails and user dashboards.
- Iframe-based meeting rooms within the OHC dashboard.

### Implementation Prompt
Integrate Whereby for automated meeting room generation for online services. Provide a one-click join experience for both merchants and customers.

---

## 6. Shipping & Logistics: Sendle
### Problem Statement
Priya (Boutique Owner) finds shipping "zones" and rates confusing. She wants a simple, flat-rate, carbon-neutral shipping option that aligns with her values.

### Research Report
- **Tool**: Sendle
- **Evaluation**: Carbon-neutral shipping with simple weight-based flat rates.
- **Ease of Use**: High; removes the complexity of postal zones.
- **Pricing**: Transparent flat rates; competitive for small businesses.
- **Reputation**: High; certified B Corp.
- **Compatibility**: Both Cloud and Standalone (API).

### Design Doc
- Automatic rate calculation during checkout based on weight class.
- One-click label printing in the Operations dashboard.
- Carbon-neutral badge displayed at checkout.

### Implementation Prompt
Integrate the Sendle API for shipping rate calculation and label generation. Highlight the eco-friendly benefits in the customer checkout UI.

---

## Conclusion & Next Steps
These six tools represent the best-in-class solutions for SMBs, balancing simplicity with powerful automation. The next phase should involve:
1. Prioritizing **WhatsApp (P1)** and **Razorpay (P1)** for global expansion.
2. Developing the **Unified Inbox** architecture to handle multi-channel DMs.
3. Building the **Checkout Provider Factory** to dynamically switch between Stripe, Mercado Pago, and Razorpay/Paytm based on merchant location.
