# Scout: Tool Integration Research Q2

This report details the evaluation of 7 integration tools across requested categories to expand OneHumanCorp's capabilities for small business owners.

## 1. Social Media Integration
**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**: Maya the Baker and Carlos the Handyman spend too much time jumping between Instagram DMs, Facebook Comments, and TikTok. They want a single inbox and a way to post to multiple platforms at once without understanding technical integrations.
**Research Report**:
- Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn).
- Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- Pricing: Free tier available, then scales per user.
- Fits OHC’s "The Promoter" agent to automate posts and "The Ambassador" to draft replies.
- Non-technical users benefit by never leaving the OHC interface.
- Works in Cloud mode well; Standalone mode might require personal Ayrshare API keys or direct OAuth.
**Design Doc**:
- Users link their social accounts via a simple OAuth popup in the "Marketing & Advertising" tab.
- "The Ambassador" AI monitors incoming DMs and drafts replies visible in a unified "Customer Inbox."
- "The Promoter" AI schedules and auto-posts images (e.g., new cake designs) to all linked platforms.
**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
**Priority**: P1
**Estimated Scope**: Large

## 2. Calendar & Scheduling
**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Leo the Music Tutor and Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs with their personal Google Calendar seamlessly.
**Research Report**:
- Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box.
- It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes.
- Free tier available for individuals; great for our free tier users.
- Alternative is building from scratch, which is error-prone.
**Design Doc**:
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- Users connect their Google/Outlook calendar via a one-click OAuth button in the "Operations" tab.
- When a customer books a slot on the OHC public page, Cal.com manages the calendar event and conflict resolution transparently.
**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
**Priority**: P0
**Estimated Scope**: Medium

## 3. Email Marketing
**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when new stock arrives but finds Mailchimp confusing and expensive. She just wants to say "send this to everyone who bought last month."
**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.
- It is lightweight (Go + PostgreSQL), aligning perfectly with the OHC backend stack.
- Zero extra SaaS costs for OHC Standalone users; minimal scaling costs for Cloud.
- Simplifies list management and supports template-based sending without complex drag-and-drop builders.
**Design Doc**:
- Customer Success ("The Ambassador") tags customers automatically (e.g., "bought-shoes").
- Users type a plain-text prompt: "Draft an email about our new summer dresses."
- AI generates the HTML, Listmonk handles the reliable batch delivery, bounce tracking, and open rate analytics.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium

## 4. Payment Processing
**Title**: Expand Payments with Mercado Pago for LATAM Users
**Problem Statement**: Non-US users in Latin America cannot rely solely on Stripe due to high fees, lack of local currency support, and specific local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- Mercado Pago is the dominant payment gateway in LATAM.
- Supports local payment methods which are critical for conversion (often >50% of transactions).
- API is well-documented. Settlement times are faster locally compared to cross-border Stripe.
- Works for both Cloud (via OHC platform account) and Standalone (user supplies API keys).
**Design Doc**:
- In the "Finance & Payments" settings, users select their region. If in LATAM, Mercado Pago is highlighted as the recommended provider.
- Setup involves standard OAuth flow or API key drop-in.
- Supports one-off payments and split payments for the eventual marketplace feature.
**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow.
**Priority**: P1
**Estimated Scope**: Large

## 5. Shipping & Logistics
**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking
**Problem Statement**: Priya the Boutique Owner hates manually copying addresses to USPS/FedEx to buy shipping labels. She wants one button to print a label and auto-email the tracking number.
**Research Report**:
- EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL).
- Competitive pricing (free tier for low volume, pennies per label after).
- Abstracts away complex carrier-specific APIs and handles tracking webhooks.
- Great fit for OHC physical product merchants.
**Design Doc**:
- Upon order placement, "Operations" calculates the shipping rate via EasyPost and charges the customer.
- In the Order details view, the business owner clicks "Print Label."
- EasyPost generates a PDF (auto-compressed and stored in GCS).
- Tracking updates via EasyPost webhooks trigger "The Ambassador" to email the customer automatically.
**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
**Priority**: P1
**Estimated Scope**: Medium

## 6. SMS & Notifications
**Title**: Integrate Twilio for Global SMS Alerts & Customer Notifications
**Problem Statement**: Fatima the Food Cart Operator doesn't have a reliable internet connection at her cart and relies on SMS text messages to know when a pre-order arrives.
**Research Report**:
- Twilio is the industry standard for SMS and WhatsApp messaging globally.
- Reliable delivery, deep global coverage.
- Supports WhatsApp, which is critical for markets outside the US.
- Simple API, integrates well with Go backend.
- Costs per message, can be passed to the tenant or subsidized in premium tiers.
**Design Doc**:
- Users can enable "SMS Notifications" in the "Operations" settings.
- When an order is placed, the OHC backend triggers a Twilio API call to text the business owner.
- Additionally, "The Ambassador" can send order confirmation texts to customers who prefer SMS over email.
**Implementation Prompt**: Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers.
**Priority**: P0
**Estimated Scope**: Small

## 7. Video Conferencing
**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons
**Problem Statement**: Leo the Music Tutor currently has to manually create Zoom links, email them to students, and deal with students losing the link. He needs an automated, branded video room.
**Research Report**:
- Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool.
- Requires no account for the student. Works natively in the browser and mobile.
- OHC can host a Jitsi instance (for Cloud mode) or point to public servers (for Standalone), saving users from needing a paid Zoom subscription.
- Completely seamless integration with no technical setup required by the user.
**Design Doc**:
- When a service is marked as "Online Meeting", OHC auto-generates a unique Jitsi URL (e.g., `meet.ohc.com/leo-guitar-session`).
- The link is automatically added to the calendar invite and the customer's dashboard.
- Users just click the link at the scheduled time to join the browser-based call.
**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
**Priority**: P2
**Estimated Scope**: Small
