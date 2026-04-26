# Scout: Tool Integration Research [Q2]

## Overview
This research report explores three core integration categories that solve real problems for non-technical small business owners using OneHumanCorp (OHC):
1. **Social Media Integration:** WhatsApp Business API
2. **Calendar & Scheduling:** Calendly
3. **Email Marketing:** Mailchimp

For each tool, we evaluate its suitability for our target personas (e.g., Maya the Home Baker, Carlos the Handyman), pricing, and how it aligns with OHC's mission of radical simplicity. Following the evaluations are detailed Issue Briefs ready for engineering execution.

---

## Tool Evaluations

### 1. Social Media Integration: WhatsApp Business API
**Problem Solved:** Small business owners, especially in international markets, manage customer inquiries and orders primarily through WhatsApp. Managing these conversations manually across personal and business accounts is chaotic and doesn't scale.
**Benefit to OHC Users:** By integrating WhatsApp, owners like Maya (The Home Baker) and Fatima (The Food Cart Operator) can receive and respond to customer messages directly from their OHC unified inbox. OHC AI agents can auto-reply to common questions (e.g., "What are your hours?").
**Integration Risks:** WhatsApp Cloud API requires Meta Business Manager verification, which can be confusing for non-technical users. OHC must abstract this setup process as much as possible.
**Pricing Estimate:** Meta charges per conversation (business-initiated vs. user-initiated). First 1,000 user-initiated conversations per month are typically free. OHC could absorb this or pass it via a clear tier.
**Mode Support:** Works well in Cloud mode via webhooks. Standalone mode would require an ngrok-like tunnel or local polling wrapper.

### 2. Calendar & Scheduling: Calendly API
**Problem Solved:** Service-based businesses (like Leo the Music Tutor and Carlos the Handyman) spend too much time going back and forth with clients to find meeting times.
**Benefit to OHC Users:** OHC users can embed scheduling directly on their OHC-generated storefronts. Clients can book available slots, and it automatically syncs to the owner's personal calendar (Google/Outlook).
**Integration Risks:** Calendly's API is robust, but OHC users might find configuring event types complex. The integration needs to wrap Calendly's setup into a 1-click template (e.g., "Standard 1-hour service call").
**Pricing Estimate:** Calendly offers a free tier (1 event type). API access typically requires a paid tier (e.g., Standard at $10/mo). OHC might need a partner agreement or require users to bring their own API key via an OAuth flow.
**Mode Support:** Fully supported in both Cloud and Standalone modes via OAuth.

### 3. Email Marketing: Mailchimp API
**Problem Solved:** Business owners want to keep their existing customers engaged (e.g., announcing new stock, holiday promotions) but find full-scale marketing platforms overwhelming.
**Benefit to OHC Users:** Priya (The Boutique Owner) can automatically add customers who buy her clothing to a Mailchimp list. OHC's Marketing Agent can draft monthly newsletters and schedule them via Mailchimp with zero manual layout work.
**Integration Risks:** Mailchimp has strict anti-spam and opt-in rules. OHC must ensure the storefront checkout correctly handles double opt-in consents to protect the owner's domain reputation.
**Pricing Estimate:** Mailchimp has a generous free tier (up to 500 contacts, 1,000 sends/month), perfect for early-stage OHC users.
**Mode Support:** Fully supported in both Cloud and Standalone modes via OAuth.

---

## Issue Briefs

### [Social Media] WhatsApp Business Unified Inbox Integration

**Title:** Integrate WhatsApp Business API into OHC Unified Inbox
**Problem Statement:** Many small business owners, like Fatima the Food Cart Operator, get orders and questions via WhatsApp. Having to switch between OHC for inventory and their phone for WhatsApp is stressful and leads to missed sales. They need all customer messages in one place.
**Research Report:** See evaluation above. The integration must abstract the Meta Business verification process.
**Design Doc:**
- **Trigger:** A customer messages the business's WhatsApp number.
- **Action:** The message appears in the OHC Operations Dashboard under the unified inbox. The AI Customer Success agent drafts a reply. The owner taps "Send" or the AI auto-sends if configured.
- **User Experience:** The owner goes to the OHC app, clicks "Connect WhatsApp", follows a simple OAuth flow, and instantly sees incoming messages in the OHC app.
**Implementation Prompt:**
Build the WhatsApp Business API integration so that incoming WhatsApp messages appear in the OHC tenant's unified inbox. The user should be able to read and reply to messages from the OHC mobile app. The AI agent should be able to read the incoming message and propose a draft reply. Focus on a zero-jargon setup flow.
**Priority:** P1
**Estimated Scope:** Large

### [Scheduling] 1-Click Service Booking via Calendly

**Title:** 1-Click Calendly Sync for Service Businesses
**Problem Statement:** Service providers like Carlos the Handyman lose customers because the booking process requires a phone call. They need a way for customers to instantly book an available slot on their OHC public page without double-booking their personal lives.
**Research Report:** See evaluation above. Calendly handles timezone math and calendar sync perfectly, but OHC needs to simplify the setup.
**Design Doc:**
- **Trigger:** A customer visits Carlos's OHC storefront and clicks "Book Repair".
- **Action:** A native-looking modal appears showing available times (pulled from Calendly). The customer books, and the event appears in the OHC Operations Dashboard.
- **User Experience:** In OHC settings, Carlos clicks "Connect Calendar". OHC walks him through a 2-step process to link his Google Calendar and auto-creates a "Standard Booking" event type behind the scenes.
**Implementation Prompt:**
Integrate the Calendly API to allow OHC tenants to accept bookings on their storefronts. Create an OAuth connection flow. When a customer books on the storefront, the event should register in the OHC backend and trigger an internal notification to the business owner. The storefront UI must remain mobile-first (375px baseline).
**Priority:** P1
**Estimated Scope:** Medium

### [Marketing] Automated Customer Sync to Mailchimp

**Title:** Automated Customer List Sync to Mailchimp
**Problem Statement:** Boutique owners like Priya have a list of past customers but no easy way to email them about new inventory without manually exporting and importing CSVs. They need an automated way to build their mailing list.
**Research Report:** See evaluation above. Mailchimp is the industry standard with a solid free tier for beginners.
**Design Doc:**
- **Trigger:** A customer completes a purchase on an OHC storefront and checks the "Subscribe to updates" box.
- **Action:** The customer's email and name are instantly synced to a designated Audience list in the owner's Mailchimp account.
- **User Experience:** Priya connects her Mailchimp account via OAuth in the OHC Marketing Department settings. OHC automatically creates an "OHC Customers" list in Mailchimp. She doesn't need to configure webhooks or API keys manually.
**Implementation Prompt:**
Implement an OAuth integration with Mailchimp. When a tenant enables this integration, automatically sync any new customer (who opts into marketing during checkout) to a Mailchimp Audience list. Ensure the integration correctly captures opt-in consent to maintain compliance. The AI Marketing Agent should have visibility into this list size for its weekly advisory reports.
**Priority:** P2
**Estimated Scope:** Small
