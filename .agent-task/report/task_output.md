# 🔍 Scout: Tool Integration Research Q4

## [Social] Unified Social Inbox Integration
**Problem Statement:** Small business owners struggle to keep up with customer messages scattered across Instagram, Facebook, WhatsApp, and TikTok. Missing a message means losing a sale or damaging their reputation.
**Research Report:**
- **Tool Evaluated:** Meta Graph API & Webhooks
- **Ease of Use:** Seamless for the user after initial OAuth connection. They receive all messages in one place.
- **Pricing:** Free API usage for standard messaging.
- **Reputation:** Meta is the industry standard for their platforms.
- **Cloud vs Standalone:** Works well in Cloud via standard OAuth and webhooks. Standalone might require a proxy or specific webhook tunneling (like ngrok or Cloudflare Tunnels) for incoming messages.
**Design Doc:**
- **Trigger:** User connects their social accounts via a settings page. Incoming messages trigger Webhooks.
- **Action:** OHC receives the webhook, parses the message, and displays it in the unified inbox UI. Replies sent from OHC are pushed back via the Graph API.
- **User View:** A single chat interface combining all platforms, badged with the source network's icon.
**Implementation Prompt:** Implement an OAuth flow for Meta platforms and a webhook receiver. Display incoming messages in a unified inbox and allow users to reply directly. Ensure messages sync accurately.
**Priority:** P0
**Estimated Scope:** Large

## [Calendar] Automated Booking & Sync
**Problem Statement:** Manually scheduling appointments leads to double bookings and lost time going back and forth over email or text to find a time that works.
**Research Report:**
- **Tool Evaluated:** Cal.com (Open Source Scheduling Infrastructure)
- **Ease of Use:** Extremely easy. Users get a personalized booking link to share with clients.
- **Pricing:** Self-hosted is free; Cloud plan has reasonable pricing for businesses.
- **Reputation:** Highly respected open-source alternative to Calendly.
- **Cloud vs Standalone:** Perfect fit. Can use their managed service for Cloud or bundle the open-source version for Standalone.
**Design Doc:**
- **Trigger:** User connects their calendar (Google/Outlook). Customer clicks the user's booking link.
- **Action:** Cal.com handles timezone conversion and conflict resolution, generating a calendar event and an optional video link.
- **User View:** Business owner sees new appointments appear in their OHC calendar view with notifications.
**Implementation Prompt:** Integrate Cal.com for scheduling. Allow users to connect their calendars, set availability, and share a booking link. Surface booked appointments in the main dashboard.
**Priority:** P0
**Estimated Scope:** Medium

## [Email] Customer Engagement & Campaigns
**Problem Statement:** Business owners want to send updates or promotions to their customer list but find enterprise tools like Mailchimp too complex and expensive.
**Research Report:**
- **Tool Evaluated:** Listmonk (Open Source Newsletter & Mailing List Manager)
- **Ease of Use:** Simple UI, great for basic campaigns.
- **Pricing:** Free (open-source) + SMTP costs (e.g., SendGrid/AWS SES).
- **Reputation:** Well-regarded for high performance and low resource usage.
- **Cloud vs Standalone:** Excellent for Standalone (can run locally). Cloud would require a hosted instance or SMTP integration.
**Design Doc:**
- **Trigger:** User selects a customer segment in OHC and drafts a message.
- **Action:** OHC pushes the campaign to Listmonk via API to handle the actual sending, tracking opens, and managing unsubscribes.
- **User View:** A simple WYSIWYG editor for drafting emails, and a dashboard showing open rates and clicks.
**Implementation Prompt:** Integrate Listmonk as the backend for email campaigns. Allow users to draft emails, select customer segments, send campaigns, and view basic analytics (opens/clicks) within OHC.
**Priority:** P1
**Estimated Scope:** Medium

## [Payment] Regional Payment Alternative
**Problem Statement:** Stripe isn't available or preferred everywhere. In regions like LATAM, business owners need local solutions to accept payments without high failure rates.
**Research Report:**
- **Tool Evaluated:** Mercado Pago API
- **Ease of Use:** Standard checkout flow, familiar to customers in the region.
- **Pricing:** Competitive percentage per transaction depending on the country.
- **Reputation:** Dominant and trusted in Latin America.
- **Cloud vs Standalone:** API-driven, works seamlessly in both environments.
**Design Doc:**
- **Trigger:** Customer receives an invoice or checkout link and chooses Mercado Pago.
- **Action:** OHC creates a preference via API and redirects the user to Mercado Pago's secure checkout, listening for IPN (Instant Payment Notifications) to mark the invoice as paid.
- **User View:** Business owner sees invoice status change to "Paid" automatically.
**Implementation Prompt:** Add Mercado Pago as a payment provider option. Allow users to connect their account, generate payment links for invoices, and automatically update invoice statuses based on payment webhooks.
**Priority:** P1
**Estimated Scope:** Medium

## [Shipping] Unified Logistics & Labels
**Problem Statement:** Calculating shipping rates manually and buying labels at the post office wastes hours for e-commerce or retail business owners.
**Research Report:**
- **Tool Evaluated:** Karrio (Open-Source Multi-Carrier Shipping API)
- **Ease of Use:** Abstracts away multiple carrier APIs into one.
- **Pricing:** Open source (free) or cloud hosted.
- **Reputation:** Growing popularity for simplifying logistics.
- **Cloud vs Standalone:** Very strong Standalone fit (can self-host).
**Design Doc:**
- **Trigger:** User marks an order as "Ready to Ship" and enters package dimensions.
- **Action:** OHC requests rates from Karrio, allows the user to purchase a label, and fetches the tracking number.
- **User View:** A simple panel to compare carrier rates, buy the label with one click, and print it.
**Implementation Prompt:** Integrate Karrio to provide shipping rates and label generation. Allow users to compare rates across their connected carriers, purchase labels, and track shipments directly from the order details page.
**Priority:** P2
**Estimated Scope:** Large

## [SMS] Reliable Global Notifications
**Problem Statement:** Many customers, especially in regions with lower smartphone or data usage, rely on SMS for critical updates (like appointment reminders or order readiness).
**Research Report:**
- **Tool Evaluated:** Twilio SMS API
- **Ease of Use:** Simple API, though regulatory compliance (A2P 10DLC) can be complex to set up.
- **Pricing:** Pay-per-message, relatively inexpensive.
- **Reputation:** Industry leader for reliability and global reach.
- **Cloud vs Standalone:** API-based, works flawlessly in both.
**Design Doc:**
- **Trigger:** System event (e.g., appointment tomorrow, order shipped).
- **Action:** OHC sends a templated message via the Twilio API.
- **User View:** Business owner toggles "Send SMS Reminders" in settings; sees a log of sent messages in the customer profile.
**Implementation Prompt:** Integrate Twilio for SMS notifications. Create a settings area for users to connect their Twilio credentials and toggle SMS reminders. Log sent messages in the customer history.
**Priority:** P0
**Estimated Scope:** Small

## [Video] Auto-Generated Consultations
**Problem Statement:** Service-based businesses (tutors, consultants) waste time manually creating and sending video links for remote sessions.
**Research Report:**
- **Tool Evaluated:** Jitsi Meet API
- **Ease of Use:** Extremely easy for end-users (no app download required, works in browser).
- **Pricing:** Free (open-source) or available as Jitsi as a Service (JaaS).
- **Reputation:** Highly respected open-source WebRTC solution.
- **Cloud vs Standalone:** Excellent Standalone fit (can be self-hosted alongside the app).
**Design Doc:**
- **Trigger:** An online appointment is booked.
- **Action:** OHC generates a unique Jitsi meeting URL and includes it in the calendar invite and reminders.
- **User View:** A "Join Meeting" button appears on the dashboard 10 minutes before the appointment.
**Implementation Prompt:** Integrate Jitsi Meet to auto-generate video conferencing links for appointments marked as 'online'. Embed a 'Join Meeting' button in the OHC dashboard for upcoming appointments.
**Priority:** P2
**Estimated Scope:** Small
