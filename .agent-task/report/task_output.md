# Research Report: Tool Integrations for Small Business Owners

This report consolidates research into tools that can be integrated into the OHC platform to solve real-world problems for small business owners in both Cloud and Standalone environments.

## 1. Social Media Integration: Unified Inbox
- **Problem**: Business owners miss inquiries scattered across Instagram, Facebook, WhatsApp, and TikTok.
- **Solution**: A unified inbox connecting Meta Graph API (Instagram/Messenger/WhatsApp). TikTok could be added later or via an aggregator like Ayrshare.
- **Ease of Use**: Simple OAuth connection.
- **Pricing**: Mostly free (Meta API), with WhatsApp per-conversation fees.
- **Compatibility**: Excellent in Cloud (central webhooks); requires proxy or developer setup for Standalone.

## 2. Calendar & Scheduling: Auto-Booking & Sync
- **Problem**: Back-and-forth messaging to schedule appointments wastes time and loses leads.
- **Solution**: Integrate Cal.com for scheduling logic, syncing with Google Calendar/Outlook via OAuth.
- **Ease of Use**: One-click connect; automated public booking page.
- **Pricing**: Cal.com is open-source/self-hostable. Google/Outlook APIs are free for basic use.
- **Compatibility**: Self-hosted Cal.com works perfectly in both Cloud and Standalone modes.

## 3. Email Marketing: Native Campaigns
- **Problem**: Third-party tools like Mailchimp are too complex and disconnected from native customer data.
- **Solution**: Build a native email campaign manager powered by SendGrid or AWS SES.
- **Ease of Use**: Very high; managed entirely within OHC with AI drafting assistance.
- **Pricing**: API costs scaled by volume (absorbed or billed).
- **Compatibility**: Cloud uses centralized keys; Standalone could proxy or require user SMTP credentials.

## 4. Payment Processing: Alternative Providers
- **Problem**: Stripe is not viable for many global markets (e.g., LATAM).
- **Solution**: Integrate Mercado Pago to support local payment methods like Pix and Pago Fácil.
- **Ease of Use**: Users select their region and connect their local provider via OAuth.
- **Pricing**: Standard payment processor transaction fees apply.
- **Compatibility**: OAuth and API integrations work well in both; webhooks require a public endpoint.

## 5. Shipping & Logistics: Easy Labels
- **Problem**: Manually copying addresses to carrier sites is tedious and error-prone.
- **Solution**: Integrate EasyPost to provide real-time rates and one-click label generation.
- **Ease of Use**: "Buy Label" button directly on the OHC order page.
- **Pricing**: EasyPost charges per label after a free tier.
- **Compatibility**: REST API works seamlessly in both environments.

## 6. SMS & Notifications: Reliable Alerts
- **Problem**: App notifications are easily missed in noisy environments (e.g., food carts).
- **Solution**: Integrate Twilio for automated SMS alerts for critical events like new orders.
- **Ease of Use**: Simple toggle in settings and phone number verification.
- **Pricing**: Pay-per-message.
- **Compatibility**: Cloud uses OHC's Twilio account; Standalone requires user's API keys.

## 7. Video Conferencing: Auto-Generated Links
- **Problem**: Manually creating and sending Zoom links for appointments is unprofessional and tedious.
- **Solution**: Zoom OAuth integration to auto-generate meeting links upon booking.
- **Ease of Use**: One-time OAuth connection; automated thereafter.
- **Pricing**: Free/Paid Zoom account required by the merchant.
- **Compatibility**: Standard OAuth flow supports both modes.

---
**Next Steps**: Implement the integrations prioritized as P0 (Unified Inbox, Calendar Sync) followed by P1 (Shipping, Email, Payments, Video) and P2 (SMS).
