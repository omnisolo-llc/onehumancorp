# Tool Integration Research Report Q4

## 1. Cal.com (Calendar & Scheduling)
**Problem Statement:** Small business owners (like tutors, consultants, or therapists) waste hours going back and forth via email to find meeting times. They need a simple, professional way for clients to book time without the complexity of enterprise tools.
**Research Report:**
- **Overview:** Cal.com is an open-source scheduling tool and Calendly alternative. It offers powerful integrations (Zoom, Google Meet) and is highly customizable.
- **Ease of Use:** Extremely user-friendly for non-technical users. The interface is clean, and the free tier covers basic individual needs.
- **Pricing:** Free for individuals. $12/user/month for Teams. Organizations plan at $28/user/month.
- **Compatibility:** Works well in Cloud. Being open-source, it offers self-hosting, which aligns well with OHC's Standalone mode.
- **Key advantages and risks:**
  - **Advantages:** Open-source, generous free tier, highly customizable.
  - **Risks:** Self-hosting in Standalone mode requires maintenance overhead; AI features might confuse non-technical users.
- **Persona:** Service-based business owners.
**Design Doc:**
- **Trigger:** Business owner sets up a booking link in OHC and shares it.
- **Action:** Client clicks the link, sees available times, and books.
- **User View:** A simple embedded calendar interface within OHC.
**Implementation Prompt:**
- Integrate Cal.com so business owners can generate personal booking links directly from OHC, syncing with existing calendars.
**Priority:** P1
**Estimated Scope:** Medium

## 2. Shippo (Shipping & Logistics)
**Problem Statement:** E-commerce small business owners struggle with calculating accurate shipping rates, printing labels, and tracking packages across multiple carriers.
**Research Report:**
- **Overview:** Shippo is a multi-carrier shipping API and web app.
- **Ease of Use:** Very intuitive dashboard for small businesses.
- **Pricing:** Starter plan is free (pay postage only, + $0.05/label for own accounts). Pro plan starts at $17/mo.
- **Compatibility:** API-first approach fits OHC Cloud. Standalone requires API connectivity.
- **Key advantages and risks:**
  - **Advantages:** Aggregates 40+ carriers, deep discounts, easy label printing.
  - **Risks:** Relies on third-party carrier reliability; pricing can scale quickly for high-volume senders.
- **Persona:** E-commerce sellers, artisans.
**Design Doc:**
- **Trigger:** An order is placed or marked 'ready to ship' in OHC.
- **Action:** OHC fetches real-time rates from Shippo, generates a label, and sends tracking info.
- **User View:** A "Shipping" tab in OHC with pending orders and a "Buy Label" button.
**Implementation Prompt:**
- Add a Shippo integration that allows merchants to compare shipping rates across major carriers and purchase labels in OHC.
**Priority:** P1
**Estimated Scope:** Medium

## 3. Twilio (SMS & Notifications)
**Problem Statement:** Small business owners serving customers with lower English proficiency need a reliable way to send order updates directly via SMS.
**Research Report:**
- **Overview:** Twilio is a cloud communications platform providing APIs for SMS.
- **Ease of Use:** Invisible to the end-user once integrated.
- **Pricing:** Pay-as-you-go. SMS starts at $0.0083/message (US).
- **Compatibility:** Suited for OHC Cloud. Standalone requires internet connection.
- **Key advantages and risks:**
  - **Advantages:** Extremely reliable, global reach, developer-friendly.
  - **Risks:** Complex initial setup; strict regulatory compliance rules (like 10DLC in the US) can be burdensome for small business owners.
- **Persona:** Local services, retail shops.
**Design Doc:**
- **Trigger:** An appointment is coming up, or an order is ready for pickup.
- **Action:** OHC triggers a Twilio API call to send an SMS.
- **User View:** A toggle in settings: "Enable SMS Notifications".
**Implementation Prompt:**
- Integrate Twilio SMS API to enable automated text notifications for key customer journey events.
**Priority:** P0
**Estimated Scope:** Large

## 4. Sprout Social (Social Media Integration)
**Problem Statement:** Small business owners struggle to keep up with customer inquiries across multiple platforms, often missing Instagram DMs or Facebook comments because they aren't checking every app.
**Research Report:**
- **Overview:** Sprout Social provides a unified smart inbox that aggregates messages, comments, and mentions from major social networks.
- **Ease of Use:** Highly rated user interface designed specifically to aggregate social communications into a single stream.
- **Pricing:** Standard plan starts at $199 per seat/month.
- **Compatibility:** Cloud-native platform. Standalone would require API integration to sync messages locally.
- **Key advantages and risks:**
  - **Advantages:** Unified inbox, robust reporting, supports all major networks.
  - **Risks:** High starting price point may be prohibitive for very small businesses; feature bloat for users who only need a simple inbox.
- **Persona:** Retailers, service providers, content creators.
**Design Doc:**
- **Trigger:** A customer sends a DM on Instagram or comments on a Facebook post.
- **Action:** OHC fetches the message via Sprout Social's API and displays it in the owner's unified inbox.
- **User View:** An "Inbox" tab where owners can read and reply to messages from any connected platform without leaving OHC.
**Implementation Prompt:**
- Build an integration with Sprout Social's API to fetch incoming social messages and allow the business owner to reply directly from a unified interface within OHC.
**Priority:** P1
**Estimated Scope:** Large

## 5. Mailchimp (Email Marketing)
**Problem Statement:** Small business owners need an easy way to design, send, and track email campaigns to their customer lists to drive repeat sales.
**Research Report:**
- **Overview:** Mailchimp is a leading email marketing and automation platform.
- **Ease of Use:** Drag-and-drop builder is highly praised and very beginner-friendly.
- **Pricing:** Free tier (up to 250 contacts). Essentials starts at $13/mo.
- **Compatibility:** Cloud-native. Fits well into OHC's Cloud environment via API.
- **Key advantages and risks:**
  - **Advantages:** Excellent template library, strong analytics, built-in CRM features.
  - **Risks:** Pricing scales steeply with list size; some features overlap with OHC's core offerings.
- **Persona:** E-commerce, content creators, service providers.
**Design Doc:**
- **Trigger:** Business owner wants to announce a sale or send a newsletter.
- **Action:** OHC syncs customer segments to Mailchimp; owner sends campaign.
- **User View:** A "Marketing" tab showing synced contacts and recent campaign performance.
**Implementation Prompt:**
- Implement a 2-way sync between OHC's customer list and Mailchimp audiences, allowing owners to trigger automated emails or newsletters seamlessly.
**Priority:** P2
**Estimated Scope:** Medium

## 6. Square (Payment Processing)
**Problem Statement:** Small businesses need an accessible, transparent alternative to traditional payment processors, especially one that bridges online and in-person sales.
**Research Report:**
- **Overview:** Square provides comprehensive payment processing and point-of-sale solutions tailored for small businesses.
- **Ease of Use:** Extremely simple onboarding. Hardware is plug-and-play for in-person sales.
- **Pricing:** Typically 2.9% + 30¢ for online transactions; 2.6% + 15¢ for in-person tap, dip, or swipe. No monthly fees for basic processing.
- **Compatibility:** Strong APIs for Cloud integration. Hardware SDKs available for Standalone POS setups.
- **Key advantages and risks:**
  - **Advantages:** Flat-rate pricing, immediate payouts available, excellent hardware integration for omnichannel businesses.
  - **Risks:** Can be more expensive for high-volume, low-ticket transactions compared to interchange-plus pricing models.
- **Persona:** Coffee shops, boutiques, mobile service providers.
**Design Doc:**
- **Trigger:** Customer is ready to pay online or via a connected terminal.
- **Action:** OHC processes the payment via the Square API or Terminal API.
- **User View:** A "Payments" dashboard showing current balance, recent transactions, and payout schedules.
**Implementation Prompt:**
- Integrate Square's Web Payments SDK for online checkout and the Terminal API to allow omnichannel businesses to process payments seamlessly within OHC.
**Priority:** P0
**Estimated Scope:** Large

## 7. Zoom (Video Conferencing)
**Problem Statement:** Service providers (tutors, therapists) need an automated way to generate and share reliable video meeting links for online consultations.
**Research Report:**
- **Overview:** Zoom is a ubiquitous video communications platform.
- **Ease of Use:** Extremely familiar to most users; easy to join meetings.
- **Pricing:** Basic (Free) limits meetings to 40 mins. Pro/Business plans required for longer meetings and advanced features.
- **Compatibility:** Cloud-native. Integrates well via OAuth and APIs.
- **Key advantages and risks:**
  - **Advantages:** High reliability, brand recognition, excellent video/audio quality.
  - **Risks:** 40-minute limit on free tier; privacy concerns require careful configuration of meeting settings (passcodes/waiting rooms).
- **Persona:** Educators, consultants, telehealth providers.
**Design Doc:**
- **Trigger:** A new online appointment is booked in OHC.
- **Action:** OHC calls the Zoom API to create a meeting and appends the join URL to the appointment details.
- **User View:** The appointment card in OHC displays a prominent "Join Zoom Meeting" button.
**Implementation Prompt:**
- Build an OAuth integration with Zoom to automatically generate unique meeting links and passcodes for newly scheduled online appointments, attaching them to calendar invites.
**Priority:** P2
**Estimated Scope:** Medium
