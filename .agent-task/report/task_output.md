# OHC Tool Integration Research Report

## Executive Summary
This report evaluates various third-party tools across key categories (Social Media, Calendar, Email Marketing, Payments, Shipping, SMS, Video) that address the core needs of small business owners using OHC. The focus is on tools that require minimal technical setup and offer seamless integration for non-technical users in both Cloud and Standalone environments.

## Category Evaluations

### 1. Social Media Integration: Meta Graph API (Instagram/FB/WhatsApp)
**Problem:** Business owners miss leads because messages are scattered across multiple social platforms.
**Solution:** Meta's Graph API provides unified access to Instagram DMs, Facebook Messenger, and WhatsApp.
**Integration Risk:** Complex OAuth flow and strict app review requirements.
**Pricing:** WhatsApp has conversational pricing; FB/IG are generally free.
**Standalone Support:** Requires cloud webhook proxy for local environments.

### 2. Calendar & Scheduling: Cal.com
**Problem:** Back-and-forth emails to schedule appointments waste time and cause drop-offs.
**Solution:** Cal.com offers open-source scheduling with deep integrations and customizable booking pages.
**Integration Risk:** Managing calendar conflicts and timezone sync accurately.
**Pricing:** Free for basic, reasonable pro tiers.
**Standalone Support:** Excellent (open-source nature fits local/self-hosted models).

### 3. Email Marketing: Resend
**Problem:** Engaging existing customers with promotions is too complicated with traditional tools.
**Solution:** Resend offers a developer-friendly API with modern templates and excellent deliverability.
**Integration Risk:** Handling unsubscribe compliance and bounce rates.
**Pricing:** Generous free tier; usage-based after.
**Standalone Support:** Yes, API-driven.

### 4. Payment Processing: Razorpay (India Focus)
**Problem:** Stripe doesn't serve all global markets, limiting sales in growing regions like India.
**Solution:** Razorpay dominates the Indian market with support for UPI, local cards, and wallets.
**Integration Risk:** Complex KYC for merchants; varying settlement times.
**Pricing:** Standard transaction fees (~2%).
**Standalone Support:** Yes, but requires secure webhook handling.

### 5. Shipping & Logistics: Shippo
**Problem:** Calculating shipping rates manually leads to undercharging or cart abandonment.
**Solution:** Shippo aggregates rates across multiple carriers (USPS, FedEx, UPS) and generates labels.
**Integration Risk:** Carrier API outages affecting real-time rate calculation at checkout.
**Pricing:** Pay per label or monthly subscription.
**Standalone Support:** Yes.

### 6. SMS & Notifications: Twilio
**Problem:** Email updates get ignored; urgent notifications (e.g., class changes) require SMS.
**Solution:** Twilio is the industry standard for programmatic SMS globally.
**Integration Risk:** Carrier filtering (A2P 10DLC compliance in US).
**Pricing:** Pay per message (~$0.0079 in US).
**Standalone Support:** Yes.

### 7. Video Conferencing: Google Meet API
**Problem:** Manually creating and sharing video links for online services is tedious.
**Solution:** Auto-generate Google Meet links upon booking confirmation.
**Integration Risk:** Requires Google Workspace OAuth integration.
**Pricing:** Included with Google Workspace.
**Standalone Support:** Yes, via API.

## Next Steps
The generated issue briefs outline specific implementation steps for the highest priority tools evaluated above.
