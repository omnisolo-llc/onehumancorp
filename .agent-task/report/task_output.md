# 🔍 Scout: Tool Integration Research [Q2]

## Executive Summary
This research mission focused on expanding OHC's capabilities by identifying and evaluating high-impact tools for small business owners. The goal was to find integrations that solve real-world problems in both Cloud and Standalone environments, prioritizing ease of use, affordability, and "Radical Simplicity."

Seven key categories were explored:
1. **Social Media**: WhatsApp Business Platform
2. **Calendar**: Microsoft Outlook (Graph API)
3. **Email Marketing**: Brevo
4. **Payment Processing**: Razorpay (India Focus)
5. **Shipping**: ShipEngine
6. **SMS**: MessageBird
7. **Video Conferencing**: Whereby

---

## 🔬 Research Report & Findings

### 1. Social Media Integration: WhatsApp Business Platform
- **Problem**: SMBs in LATAM, India, and Europe rely heavily on WhatsApp, leading to manual response fatigue.
- **Finding**: The WhatsApp Business Platform (API) is the gold standard. It allows for automated replies via OHC's "Ambassador" agent.
- **SMB Value**: 1,000 free service conversations/month makes it accessible for small players.
- **Integration**: Requires Meta Business Verification. OHC should provide a guided "embedded signup" flow.

### 2. Calendar & Scheduling: Microsoft Outlook (Graph API)
- **Problem**: Office 365 users face friction managing dual calendars.
- **Finding**: Microsoft Graph API provides robust access to Outlook calendars.
- **SMB Value**: Professional-grade scheduling with zero additional cost for existing 365 users.
- **Integration**: Standard OAuth 2.0. Native "Free/Busy" lookups ensure zero double-booking.

### 3. Email Marketing: Brevo
- **Problem**: Mailchimp is often perceived as too expensive and complex for small boutiques.
- **Finding**: Brevo offers a volume-based pricing model and a generous free tier (300 emails/day).
- **SMB Value**: All-in-one suite (Email, SMS, WhatsApp) simplifies the vendor stack.
- **Integration**: Developer-friendly API. OHC can act as a simplified "wrapper" for AI-generated campaigns.

### 4. Payment Processing: Razorpay (India Focus)
- **Problem**: High demand for UPI and local card support in the Indian market.
- **Finding**: Razorpay is the dominant player in India, offering superior success rates for domestic transactions.
- **SMB Value**: Transparent 2% fees and deep integration with the "India Stack" (UPI).
- **Integration**: Standard API keys or OAuth. Normalizing Razorpay webhooks into OHC events is straightforward.

### 5. Shipping & Logistics: ShipEngine
- **Problem**: Manual rate comparison and label generation waste hours for ecommerce merchants.
- **Finding**: ShipEngine provides a unified API for 100+ carriers.
- **SMB Value**: Pay-per-label model (-bash.05) is perfect for low-volume sellers.
- **Integration**: OHC can automate the "Cheapest/Fastest" selection via the "Manager" agent.

### 6. SMS & Notifications: MessageBird
- **Problem**: Owners miss data-reliant app notifications in spotty connectivity areas.
- **Finding**: MessageBird (Bird) provides global reach with high-reliability SMS delivery.
- **SMB Value**: Pay-as-you-go pricing ensures owners only pay for what they use.
- **Integration**: Critical for order alerts and appointment reminders where data is unreliable.

### 7. Video Conferencing: Whereby
- **Problem**: Friction caused by forcing customers to download Zoom or create Google accounts.
- **Finding**: Whereby Embedded allows for zero-download, browser-based video calls.
- **SMB Value**: Branded, professional experience that lives directly inside the OHC storefront.
- **Integration**: Lightweight API for temporary room generation; can be embedded in an <iframe>.

---

## 🚀 Proposed Next Steps
1. **Phase 1: High Impact (P1)**: Prioritize **WhatsApp**, **Outlook**, and **Brevo** integrations. These address the "Operational Fatigue" and "Marketing Dread" pain points most directly.
2. **Phase 2: Regional Growth (P1)**: Implement **Razorpay** to capture the rapidly growing Indian SMB market.
3. **Phase 3: Operational Excellence (P2)**: Roll out **ShipEngine**, **MessageBird**, and **Whereby** to provide a full-stack experience for power users.

---

## 📝 Consolidated Issue Briefs

### [Social Media] WhatsApp Business Integration
**Title**: Integrate WhatsApp Business Platform for Automated Customer Support
**Problem**: Repetitive WhatsApp inquiries take hours to manage manually.
**Design**: "Ambassador" AI drafts/sends replies via Meta Webhooks.
**Priority**: P1 | **Scope**: Medium

### [Calendar] Microsoft Outlook Integration
**Title**: Microsoft Outlook Calendar Sync for Service Bookings
**Problem**: Double-booking and friction between OHC and Office 365 calendars.
**Design**: Bi-directional sync via Microsoft Graph API for Free/Busy status.
**Priority**: P1 | **Scope**: Medium

### [Email Marketing] Brevo Integration
**Title**: Native Email Marketing & Automation via Brevo
**Problem**: Complexity and cost of tools like Mailchimp for small boutiques.
**Design**: "The Promoter" AI manages campaigns via Brevo SMTP/API.
**Priority**: P1 | **Scope**: Medium

### [Payment Processing] Razorpay Integration
**Title**: Native Payment Integration for Indian Market via Razorpay
**Problem**: Lack of UPI/RuPay support for Indian merchants in standard gateways.
**Design**: Dynamic checkout switching to Razorpay for Indian tenants.
**Priority**: P1 | **Scope**: Large

### [Shipping] ShipEngine Integration
**Title**: Multi-Carrier Shipping & Label Generation via ShipEngine
**Problem**: Time wasted on manual shipping rate comparison and address typing.
**Design**: "The Manager" AI selects optimal rates; 1-tap label generation.
**Priority**: P2 | **Scope**: Large

### [SMS] MessageBird Integration
**Title**: Global SMS Notifications via MessageBird
**Problem**: Missed orders due to spotty data connectivity and app notification lag.
**Design**: Async SMS dispatch for order alerts and appointment reminders.
**Priority**: P2 | **Scope**: Medium

### [Video Conferencing] Whereby Integration
**Title**: One-Click Branded Video Consultations via Whereby
**Problem**: Friction and unprofessional feel of external video meeting apps.
**Design**: Embedded <iframe> room generation for online services.
**Priority**: P2 | **Scope**: Small
