# Scout: Tool Integration Research Report

## Overview
This report outlines potential tool integrations for the OHC platform to empower non-technical small business owners, bridging gaps in operations, marketing, sales, and customer success. The research is structured around 7 key categories identified for investigation.

## 1. Social Media Integration
**Recommendation:** Meta Graph API (Instagram, Facebook Messenger, WhatsApp)
**Why:** Unifies customer communication across the three most dominant platforms for SMBs into a single inbox.

### Issue Brief: Meta Graph API Integration

**Title**: Implement Unified Social Media Inbox via Meta Graph API
**Problem Statement**: Small business owners (like Maya the baker) manage customer inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Context switching between apps leads to missed messages, delayed responses, and lost sales. They need a single, unified inbox within OHC, managed by the "Customer Success" AI agent.
**Research Report**:
- **Tool**: Meta Graph API (specifically Messenger API for Instagram, Messenger API, and WhatsApp Business API).
- **Ease of Use (End User)**: Seamless. After an initial OAuth connection, messages flow into the OHC inbox natively.
- **Pricing**: Instagram/Messenger API is generally free for standard usage. WhatsApp Business API charges per conversation (user-initiated vs. business-initiated), which needs to be factored into OHC pricing tiers.
- **Cloud vs. Standalone**: Works in Cloud (webhooks to OHC servers). For Standalone, requires a cloud proxy to receive webhooks and forward them to the local instance, or polling (less ideal).
**Design Doc**:
- **Trigger**: User connects their Facebook/Instagram/WhatsApp Business accounts via an "Integrations" UI.
- **Action**: Webhooks are established. Incoming messages trigger the Customer Success AI agent to draft responses or alert the user. The user can reply directly from the OHC interface.
- **UI**: A unified "Inbox" view aggregating messages from all three channels, visually tagged by source.
**Implementation Prompt**: Implement an integration with the Meta Graph API that allows users to connect their Instagram Business, Facebook Page, and WhatsApp Business accounts. Ensure incoming messages from these channels appear in a unified OHC inbox. The system must support sending replies back to the respective platforms.
**Priority**: P0
**Estimated Scope**: Large


## 2. Calendar & Scheduling
**Recommendation:** Cal.com API
**Why:** Developer-friendly, highly customizable scheduling API that handles timezones, conflicts, and meeting link generation natively, offering a better embedded experience than directly wrangling Google/Outlook APIs.

### Issue Brief: Cal.com API Integration

**Title**: Implement Seamless Booking and Scheduling via Cal.com API
**Problem Statement**: Service-based businesses (like Carlos the handyman or Leo the tutor) need a way for customers to book time slots without back-and-forth emails. Building a robust calendar system from scratch (handling timezones, double-booking, Google/Outlook sync) is error-prone.
**Research Report**:
- **Tool**: Cal.com API v2.
- **Ease of Use (End User)**: Invisible. The business owner sets their availability in OHC, and OHC handles the rest. Customers see a simple date/time picker on the storefront.
- **Pricing**: Open-source/Self-hosted (free, but maintenance overhead) or Managed Platform (starts at $15/user/mo, enterprise pricing available for white-label API usage).
- **Cloud vs. Standalone**: Cal.com offers a self-hosted option, making it viable for OHC's Standalone mode if bundled, or via API for the Cloud mode.
**Design Doc**:
- **Trigger**: User creates a "Service" product type and defines availability hours.
- **Action**: OHC creates a managed user/event type via Cal.com API. The storefront embeds a booking widget or renders a custom UI powered by Cal.com's available slots API.
- **UI**: "Availability" settings in the dashboard. Booking calendar component on the public storefront.
**Implementation Prompt**: Integrate the Cal.com API to power booking functionality for service-based products. Business owners should be able to set their available hours, and customers should be select an open slot during checkout. Ensure booked slots are automatically blocked off.
**Priority**: P0
**Estimated Scope**: Medium


## 3. Email Marketing
**Recommendation:** Resend API
**Why:** Modern, highly reliable, and exceptionally developer-friendly email API. Allows for easy template management and broadcast sending, fitting perfectly into the "Marketing & Advertising" department.

### Issue Brief: Resend API Integration

**Title**: Implement Email Marketing Campaigns via Resend API
**Problem Statement**: Business owners (like Priya the boutique owner) need to send newsletters, promotional blasts, and automated "back in stock" emails to their customer list. Standard transactional email APIs are too complex for non-technical users to design templates.
**Research Report**:
- **Tool**: Resend API.
- **Ease of Use (End User)**: Transparent. Users write plain text or use an AI-assisted rich text editor in OHC; Resend handles the delivery and rendering.
- **Pricing**: Generous free tier (3,000 emails/mo). Paid tiers are affordable ($20/mo for 50,000 emails).
- **Cloud vs. Standalone**: Cloud-only API. Standalone users would need to provide their own Resend API key or use a local SMTP relay.
**Design Doc**:
- **Trigger**: User initiates an email campaign via the "Marketing" department, or an automated trigger fires (e.g., abandoned cart).
- **Action**: The AI agent drafts the content. OHC compiles it into an HTML template and sends it via the Resend API to the target audience segment.
- **UI**: A "Campaigns" tab where users can view sent emails, open rates, and click rates (via Resend webhooks).
**Implementation Prompt**: Integrate the Resend API to enable outbound email marketing. Provide a UI for users to draft and send broadcast emails to their customer lists. Implement webhook listeners to track and display open and click metrics in the OHC dashboard.
**Priority**: P1
**Estimated Scope**: Medium


## 4. Payment Processing (LATAM Focus)
**Recommendation:** Mercado Pago API
**Why:** The dominant payment processor in Latin America, essential for users outside the core Stripe-supported regions. Crucial for the "Finance & Payments" department's global reach.

### Issue Brief: Mercado Pago Integration

**Title**: Implement LATAM Payment Processing via Mercado Pago
**Problem Statement**: OHC currently relies on Stripe, which is not fully supported or preferred in many Latin American countries. To truly serve "everyone," OHC needs a payment processor that handles local payment methods (e.g., PIX in Brazil, OXXO in Mexico).
**Research Report**:
- **Tool**: Mercado Pago API.
- **Ease of Use (End User)**: Standard OAuth flow to connect their Mercado Pago account. Familiar checkout experience for their local customers.
- **Pricing**: Percentage per transaction + fixed fee (varies significantly by country and payment method).
- **Cloud vs. Standalone**: Works in both. Webhooks required for asynchronous payment confirmation (e.g., cash payments at convenience stores).
**Design Doc**:
- **Trigger**: User selects "Mercado Pago" as their payment provider in settings and completes the OAuth flow.
- **Action**: Checkout sessions route through Mercado Pago instead of Stripe. Webhooks handle payment status updates (pending -> approved).
- **UI**: "Connect Mercado Pago" button in settings. Mercado Pago checkout options presented to buyers based on their region.
**Implementation Prompt**: Integrate the Mercado Pago checkout API as an alternative to Stripe. Allow business owners to connect their Mercado Pago accounts. The checkout flow must support Mercado Pago redirection or embedded checkout, handling asynchronous payment confirmations via webhooks.
**Priority**: P1
**Estimated Scope**: Large


## 5. Shipping & Logistics
**Recommendation:** Shippo API
**Why:** Abstracts away the complexity of multiple carriers (USPS, FedEx, UPS, international) into a single API. Perfect for the "Operations" department to auto-generate shipping rates and labels.

### Issue Brief: Shippo API Integration

**Title**: Implement Automated Shipping Rates and Labels via Shippo
**Problem Statement**: Businesses selling physical goods need to calculate accurate shipping rates at checkout and easily print shipping labels. Doing this manually per order is a massive time sink.
**Research Report**:
- **Tool**: Shippo API.
- **Ease of Use (End User)**: Highly automated. Rates appear automatically at checkout. The owner clicks "Print Label" in the OHC dashboard.
- **Pricing**: Pay-as-you-go ($0.05 per label) or monthly subscription for volume. Often provides discounted carrier rates.
- **Cloud vs. Standalone**: Cloud API. Works in both modes, though Standalone requires internet access to fetch rates and generate labels.
**Design Doc**:
- **Trigger**: Customer reaches checkout (fetches rates). Business owner clicks "Fulfill Order" (generates label).
- **Action**: OHC queries Shippo for rates based on package weight/dimensions and destination. Upon fulfillment, OHC purchases the label via Shippo and provides a printable PDF link.
- **UI**: "Shipping" settings to define box sizes. Real-time rate display at checkout. "Generate Label" button on the order detail page.
**Implementation Prompt**: Integrate the Shippo API to provide real-time shipping rates at checkout based on product weight and customer address. Add functionality in the order management UI for the business owner to purchase and download shipping labels for fulfilled orders.
**Priority**: P1
**Estimated Scope**: Medium


## 6. SMS & Notifications
**Recommendation:** Twilio Programmable Messaging API
**Why:** The industry standard for SMS. Critical for reaching users with low digital literacy or limited data plans (e.g., Fatima persona), providing real-time order updates and pickup notifications.

### Issue Brief: Twilio SMS Integration

**Title**: Implement SMS Notifications via Twilio Programmable Messaging
**Problem Statement**: Not all customers have smartphones or reliable internet, and some business operators (like Fatima the food cart owner) prefer simple text messages over app notifications for order alerts.
**Research Report**:
- **Tool**: Twilio Programmable Messaging API.
- **Ease of Use (End User)**: Completely invisible. They simply receive text messages on their phone.
- **Pricing**: Pay-as-you-go (approx. $0.0079 per outbound SMS in the US, varies globally). Requires A2P 10DLC registration in the US (compliance overhead).
- **Cloud vs. Standalone**: Cloud API. Works in both, but Standalone users might need to provide their own Twilio credentials if OHC doesn't want to act as a proxy/reseller.
**Design Doc**:
- **Trigger**: System events (New Order received, Order Ready for Pickup, Appointment Reminder).
- **Action**: OHC sends a formatted SMS string via the Twilio API to the specified phone number.
- **UI**: Notification preferences toggles ("Send me an SMS when I get a new order", "Send customer an SMS when order is ready").
**Implementation Prompt**: Integrate the Twilio API to send SMS notifications. Implement triggers for critical business events (e.g., new order alerts for the owner, pickup readiness for the customer). Provide UI settings for users to opt-in and configure SMS alerts.
**Priority**: P2
**Estimated Scope**: Medium


## 7. Video Conferencing
**Recommendation:** Zoom API
**Why:** Ubiquitous video conferencing solution. Integrating auto-generated meeting links directly into booking flows serves service-based personas (e.g., Leo the music tutor) perfectly.

### Issue Brief: Zoom API Integration

**Title**: Implement Auto-Generated Video Meeting Links via Zoom API
**Problem Statement**: Online service providers (like Leo the tutor) spend unnecessary time manually creating Zoom links for every booked appointment and sending them to clients.
**Research Report**:
- **Tool**: Zoom API (Server-to-Server OAuth or standard OAuth).
- **Ease of Use (End User)**: Seamless. The business owner connects their Zoom account once. Customers receive the link automatically upon booking.
- **Pricing**: Free to build the integration. The business owner needs a Zoom account (Free or Pro depending on their meeting length needs).
- **Cloud vs. Standalone**: Cloud API. Works in both modes.
**Design Doc**:
- **Trigger**: A booking is confirmed for a service marked as "Online/Video".
- **Action**: OHC calls the Zoom API to create a meeting scheduled for the booked time. The join URL is saved to the booking record and emailed to the customer.
- **UI**: "Connect Zoom" button in Integrations. A location dropdown on services including "Zoom Meeting".
**Implementation Prompt**: Integrate the Zoom API to automatically generate meeting links for booked online services. Allow users to authenticate their Zoom accounts. When an online service is booked, create a Zoom meeting and include the join link in the confirmation email and calendar event.
**Priority**: P2
**Estimated Scope**: Small


## Next Steps
1. Prioritize implementation based on user demand, starting with Social Media (Meta Graph API) and Scheduling (Cal.com).
2. Begin technical design for the top priorities, focusing on how the OHC AI Agents will interface with these APIs to abstract the complexity from the end-user.
