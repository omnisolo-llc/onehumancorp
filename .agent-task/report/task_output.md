# 🔍 Scout: Tool Integration Research [Q3]

This report evaluates tools across seven key categories to expand OHC's capabilities for small business owners in both Cloud and Standalone environments.

---

## 1. Social Media Integration

### Issue Brief: Unified Social Inbox Integration
**Title**: Integrate Social Media Direct Messages into Unified Inbox
**Problem Statement**: Small business owners like Fatima struggle to keep up with customer messages scattered across Instagram, Facebook, WhatsApp, and TikTok, leading to missed sales and slow response times.
**Research Report**:
- Evaluated Tools: ManyChat, Meta Business Suite API.
- Ease of use: High for Meta, moderate for ManyChat.
- Pricing: Free tier available, ~$15/mo for advanced features.
- These tools provide reliable webhooks and handle message parsing, reducing the burden on business owners.
**Design Doc**: The integration will allow users to connect their social accounts via OAuth. Once connected, incoming messages from these platforms will appear in their existing OHC unified inbox. Business owners can reply directly from OHC, and the message will be routed back to the appropriate social platform.
**Implementation Prompt**: Build a seamless connection flow for social accounts. Display social messages in the unified inbox with clear indicators of their source platform. Enable sending replies from the inbox back to the customer's social app. Ensure reliable delivery and error handling.
**Priority**: P0
**Estimated Scope**: Large

---

## 2. Calendar & Scheduling

### Issue Brief: Automated Calendar Sync & Scheduling
**Title**: Automatic Google Calendar Sync and Booking
**Problem Statement**: Managing appointments manually causes double-bookings and wastes time, frustrating both the business owner and their clients.
**Research Report**:
- Evaluated Tools: Calendly API, Cal.com.
- Ease of use: Cal.com offers great embeddability; Calendly is widely recognized.
- Pricing: Free basic tier, ~$12/mo for premium.
- Cal.com works well in both Cloud and Standalone modes and handles timezone math effortlessly.
**Design Doc**: Users will connect their calendar provider to OHC. A personalized booking link will be generated, which the business owner can share. When a client books a slot, it automatically updates the owner's calendar and prevents future conflicts.
**Implementation Prompt**: Create a settings area for calendar connection. Generate a shareable booking page that reflects the user's real-time availability. Handle timezone conversions automatically for both the owner and the client.
**Priority**: P1
**Estimated Scope**: Medium

---

## 3. Email Marketing

### Issue Brief: Customer Email Campaign Manager
**Title**: Integrated Email Marketing and Newsletter Tool
**Problem Statement**: Business owners need an easy way to announce promotions or updates to their customer list without learning complex email marketing software.
**Research Report**:
- Evaluated Tools: Mailchimp API, Resend.
- Ease of use: Mailchimp has a great visual editor; Resend is developer-friendly but can be wrapped nicely.
- Pricing: Free for up to 1,000 contacts, scalable pricing.
- Compliance (spam) and analytics (open rates) are well handled by these providers.
**Design Doc**: OHC will feature a simplified campaign builder where users can select an audience from their OHC customer list, write a message, and send it. OHC will display basic metrics like open rates directly in the dashboard.
**Implementation Prompt**: Implement an interface for drafting email campaigns. Provide audience selection based on the existing customer list. Show post-campaign analytics (sends, opens) in a clear, simple dashboard.
**Priority**: P1
**Estimated Scope**: Large

---

## 4. Payment Processing

### Issue Brief: Global Payment Provider Alternatives
**Title**: Localized Payment Processing Integration
**Problem Statement**: Stripe isn't supported or preferred in all regions, preventing small businesses in LATAM or Asia from easily collecting payments online.
**Research Report**:
- Evaluated Tools: Mercado Pago (LATAM), Razorpay (India).
- Ease of use: Good documentation for both; tailored for local markets.
- Pricing: Transaction fee based (~2-3%).
- Settlement speed is optimized for local currencies.
**Design Doc**: Provide a regional payment settings page. Depending on the user's country, allow connection to local providers like Mercado Pago. Invoices and online checkouts will route through the connected local gateway.
**Implementation Prompt**: Add region-specific payment gateway options in the checkout and invoicing flow. Ensure currencies are formatted correctly and local compliance requirements are met during the checkout experience.
**Priority**: P0
**Estimated Scope**: Medium

---

## 5. Shipping & Logistics

### Issue Brief: Automated Shipping Labels & Tracking
**Title**: Simplified Shipping Label Generation and Tracking
**Problem Statement**: Packing and shipping physical products is tedious. Manually calculating rates and buying labels takes time away from growing the business.
**Research Report**:
- Evaluated Tools: Shippo, EasyPost.
- Ease of use: Both simplify carrier integrations drastically.
- Pricing: ~$0.05 per label or small monthly fee.
- They offer broad carrier coverage (USPS, UPS, DHL) and reliable tracking updates.
**Design Doc**: For product orders, users can click a single button to generate a shipping label based on predefined package sizes. Tracking numbers will automatically be attached to the order and shared with the customer.
**Implementation Prompt**: Build a workflow for purchasing and printing shipping labels directly from an order screen. Automate tracking number updates and notifications to the customer.
**Priority**: P2
**Estimated Scope**: Large

---

## 6. SMS & Notifications

### Issue Brief: Automated SMS Notifications
**Title**: SMS Alerts for Appointments and Orders
**Problem Statement**: Email notifications are often missed. Business owners need a reliable way to remind clients of appointments or notify them of order status via text message.
**Research Report**:
- Evaluated Tools: Twilio, MessageBird.
- Ease of use: Straightforward API; global reach.
- Pricing: ~$0.01 to $0.05 per message depending on the country.
- High delivery reliability and compliance features for opt-outs.
**Design Doc**: Add a toggle in notification settings to enable SMS updates. Customers will receive automated text reminders for upcoming bookings or when their order ships. The system will handle opt-outs gracefully.
**Implementation Prompt**: Create user-facing settings to enable SMS reminders. Implement the notification triggers for appointments and order updates. Ensure the sender name represents the business owner clearly.
**Priority**: P1
**Estimated Scope**: Medium

---

## 7. Video Conferencing

### Issue Brief: Auto-Generated Meeting Links
**Title**: Seamless Video Call Link Generation
**Problem Statement**: Creating and sharing Zoom or Meet links manually for every online consultation is error-prone and unprofessional.
**Research Report**:
- Evaluated Tools: Zoom API, Google Meet (via Calendar sync).
- Ease of use: Native integrations are smooth for the end-user.
- Pricing: Requires user to have their own Zoom/Meet accounts (free or paid).
**Design Doc**: When an online service is booked, OHC will automatically generate a unique meeting link via the connected provider and embed it in the confirmation email and calendar event for both parties.
**Implementation Prompt**: Enhance the booking flow to support location type "Online Meeting". Automatically attach a unique meeting link to the booking details and notify all participants with a clear "Join Call" button.
**Priority**: P2
**Estimated Scope**: Small

---
