# 🔍 Scout: Tool Integration Research Report

## Executive Summary
This report summarizes the research and evaluation of seven tool integration categories designed to expand OneHumanCorp's (OHC) capabilities for small business owners. The focus is on tools that directly address pain points for non-technical users, ensuring high usability, fair pricing, and clear benefits across multiple business personas.

---

## 1. Social Media Integration
**Problem Statement:** Non-technical business owners receive messages across multiple platforms (Instagram, Facebook, WhatsApp). Checking each app separately causes missed leads and delayed responses.
**Tool Evaluated:** Meta Graph API (Instagram/Facebook) + WhatsApp Business API
**Market Context:** Unified inboxes are standard in expensive enterprise tools (like HubSpot) but often too complex for micro-businesses.
**Ease of Use (Persona Lens):**
- **Pros:** Business owners authenticate once via OAuth. All incoming DMs and comments appear in the OHC "Customer Success" unified inbox. AI can draft replies automatically.
- **Cons:** Meta's OAuth process can be intimidating due to multiple permission screens.
**Pricing:** API access is generally free for standard usage; WhatsApp charges per conversation.
**Deployment Modes:** Cloud (OAuth via OHC), Standalone (User provided App credentials).
**Design Doc:**
- User clicks "Connect Social Media" in the Operations department.
- Authenticates with Meta.
- Inbound messages trigger notifications in OHC.
- OHC "Customer Success" agent reads messages and suggests replies.
**Implementation Prompt:**
- **User-Facing Outcome:** Integrate Meta platforms so users can manage all customer chats in one OHC inbox.
- **Acceptance Criteria:** OAuth connection works, incoming messages sync to inbox, replies send correctly.
**Priority:** P0 (Critical)
**Estimated Scope:** Large

---

## 2. Calendar & Scheduling
**Problem Statement:** Managing appointments manually leads to double-booking and missed appointments. Business owners need a simple way for customers to pick a time that automatically syncs with their personal calendar.
**Tool Evaluated:** Google Calendar API
**Market Context:** Over 80% of target personas already use Google Calendar.
**Ease of Use (Persona Lens):**
- **Pros:** Single-click OAuth connection. No new calendar interface to learn.
- **Cons:** Permission scopes must be clearly explained so users aren't afraid to grant access.
**Pricing:** Free for standard usage.
**Deployment Modes:** Cloud and Standalone via standard OAuth 2.0 flow.
**Design Doc:**
- User connects Google Calendar in Operations.
- User sets working hours (e.g., M-F, 9-5).
- Storefront displays available slots based on real-time free/busy status.
- Bookings automatically create events in Google Calendar.
**Implementation Prompt:**
- **User-Facing Outcome:** Customers can book open timeslots that sync directly to the owner's Google Calendar.
- **Acceptance Criteria:** Calendar OAuth works, storefront shows only available times, booking creates event.
**Priority:** P0 (Critical)
**Estimated Scope:** Medium

---

## 3. Email Marketing
**Problem Statement:** Business owners want to notify past customers about new products or promotions but find tools like Mailchimp too complex to set up.
**Tool Evaluated:** SendGrid API
**Market Context:** Competitors offer basic email marketing, but setting up templates and managing lists is still a manual chore.
**Ease of Use (Persona Lens):**
- **Pros:** Invisible to the user. The "Marketing" AI drafts the email content and sends it to the customer list automatically.
- **Cons:** Requires domain authentication (SPF/DKIM) which is highly technical.
**Pricing:** Pay per email volume. Can be bundled into OHC Pro tiers.
**Deployment Modes:** Cloud (Managed SendGrid), Standalone (Bring Your Own API Key).
**Design Doc:**
- "Marketing & Advertising" agent suggests a campaign.
- Owner approves the AI-drafted email.
- Backend uses SendGrid API to dispatch emails.
**Implementation Prompt:**
- **User-Facing Outcome:** Business owners can send bulk emails to their customers drafted by AI.
- **Acceptance Criteria:** API integration sends emails reliably, unsubscribe links are included, open rates are tracked.
**Priority:** P1 (High)
**Estimated Scope:** Medium

---

## 4. Payment Processing (LATAM Focus)
**Problem Statement:** Stripe is not widely used or supported in many Latin American countries, limiting OHC's global reach.
**Tool Evaluated:** Mercado Pago API
**Market Context:** Mercado Pago dominates LATAM e-commerce, offering local payment methods like Pix (Brazil) and OXXO (Mexico).
**Ease of Use (Persona Lens):**
- **Pros:** Connects directly to existing accounts. Instantly enables local checkout options.
- **Cons:** Asynchronous payments (like cash vouchers) mean orders stay "pending" for days.
**Pricing:** Percentage fee per transaction paid by the merchant.
**Deployment Modes:** Cloud (OAuth), Standalone (User provided API Keys).
**Design Doc:**
- User in supported region connects Mercado Pago via OAuth.
- Storefront checkout dynamically displays local payment options.
- Webhooks handle asynchronous payment confirmations.
**Implementation Prompt:**
- **User-Facing Outcome:** LATAM users can offer their customers local checkout options.
- **Acceptance Criteria:** Mercado Pago OAuth integration works, checkout handles regional payment methods, webhooks update order status.
**Priority:** P1 (High)
**Estimated Scope:** Large

---

## 5. Shipping & Logistics
**Problem Statement:** Calculating shipping costs manually is error-prone. Business owners need real-time rates and easy label printing.
**Tool Evaluated:** Shippo API
**Market Context:** Shipping is the #1 pain point for physical product sellers. Tools like Shopify have robust integrated shipping.
**Ease of Use (Persona Lens):**
- **Pros:** Aggregates carriers. Auto-calculates rates at checkout. One-click label generation.
- **Cons:** Requires users to accurately input product weights and box dimensions upfront.
**Pricing:** Pay per label generated.
**Deployment Modes:** Cloud only (due to complex carrier agreements), Standalone would require user's own carrier accounts.
**Design Doc:**
- User inputs product weight.
- At checkout, Shippo API calculates shipping cost.
- In Operations dashboard, owner clicks "Print Label", Shippo generates the PDF.
**Implementation Prompt:**
- **User-Facing Outcome:** Merchants can get live shipping rates and print labels without leaving OHC.
- **Acceptance Criteria:** Shipping rates fetch correctly at checkout, label PDF is generated and downloadable.
**Priority:** P1 (High)
**Estimated Scope:** Large

---

## 6. SMS & Notifications
**Problem Statement:** Email is too slow for time-sensitive updates (e.g., "Food is ready for pickup"). Many customers prefer texts.
**Tool Evaluated:** Twilio Messaging API
**Market Context:** SMS boasts a 90%+ open rate. Competitors often require third-party plugins.
**Ease of Use (Persona Lens):**
- **Pros:** Completely automated. Order status changes trigger SMS instantly.
- **Cons:** Complex regulatory compliance (e.g., A2P 10DLC registration) must be completely abstracted.
**Pricing:** Per message segment. Must be metered or included in premium tiers.
**Deployment Modes:** Cloud (OHC managed Twilio), Standalone (User provides Twilio SID/Auth).
**Design Doc:**
- Owner enables SMS notifications.
- Customers opt-in at checkout.
- Automated triggers (Order Confirmed, Ready for Pickup) send SMS via Twilio.
**Implementation Prompt:**
- **User-Facing Outcome:** Customers receive text updates about their orders automatically.
- **Acceptance Criteria:** Twilio API integration sends SMS upon order status change, STOP replies are handled.
**Priority:** P1 (High)
**Estimated Scope:** Medium

---

## 7. Video Conferencing
**Problem Statement:** Online tutors and consultants need to automatically generate meeting links for booked appointments without manual copy-pasting.
**Tool Evaluated:** Zoom API (and Google Meet via Calendar)
**Market Context:** Auto-generating links is standard for scheduling tools (Calendly).
**Ease of Use (Persona Lens):**
- **Pros:** Zero effort. When a customer books an online service, the link is right there.
- **Cons:** Connecting Zoom requires a separate OAuth flow if they don't use Google Calendar/Meet.
**Pricing:** Zoom API is free; requires a licensed Zoom account for meetings >40 mins.
**Deployment Modes:** Cloud (OAuth), Standalone (OAuth).
**Design Doc:**
- If user connects Google Calendar, use native Meet link generation (`conferenceData`).
- If user connects Zoom via OAuth, backend uses Zoom API to create a meeting.
**Implementation Prompt:**
- **User-Facing Outcome:** Online bookings automatically include a video meeting link.
- **Acceptance Criteria:** Zoom API generates meetings, link is included in email and calendar invites.
**Priority:** P2 (Medium)
**Estimated Scope:** Medium
