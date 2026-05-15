# OHC Tool Integration Research Report Q4

## Overview
This report evaluates potential third-party tool integrations for One Human Corp (OHC) across seven key categories to benefit small business owners in both Cloud and Standalone environments. The goal is to identify user-friendly, reliable, and cost-effective tools that solve real problems without requiring technical expertise.

---

## 1. Social Media Integration: ManyChat

**Title**: [Social Media] Integrate ManyChat for Unified Social Inbox
**Problem Statement**: Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook Messenger, and WhatsApp. Missing messages leads to lost sales and frustrated customers. They need one simple inbox to view and respond to all social media messages.
**Research Report**:
- **Tool Evaluated**: ManyChat
- **Reputation**: High. Known for reliable APIs and solid Meta partnerships.
- **Ease of Use**: Excellent visual builder and straightforward OAuth connections.
- **Pricing**: Starts at $15/month for Pro, which covers essential messaging channels.
- **Cloud vs. Standalone**: Works well in Cloud via webhooks. For Standalone, requires polling or a cloud-relay service to receive incoming messages securely.
**Design Doc**:
- **Trigger**: The business owner connects their ManyChat account from the OHC Integrations settings page.
- **Action**: OHC imports new messages into the Unified Inbox tab and syncs outgoing replies back to ManyChat.
- **User View**: A familiar chat interface within OHC where messages from Instagram, WhatsApp, and Facebook appear together with clear channel icons.
**Implementation Prompt**:
- Implement an integration flow allowing users to connect ManyChat.
- Ensure incoming messages from ManyChat appear in real-time or near real-time in the OHC inbox.
- Allow the user to reply from OHC, with the message successfully delivered to the customer on their original platform.
- **Acceptance Criteria**: The user can connect ManyChat in under 3 clicks, receive an Instagram DM in OHC, and reply successfully.
**Priority**: P1
**Estimated Scope**: Medium

---

## 2. Calendar & Scheduling: Calendly

**Title**: [Calendar] Integrate Calendly for Automated Scheduling
**Problem Statement**: Back-and-forth emails to find a meeting time are tedious and look unprofessional. Small business owners need a simple way to let clients book available time slots directly without double-booking their personal calendar.
**Research Report**:
- **Tool Evaluated**: Calendly
- **Reputation**: Industry standard, highly reliable.
- **Ease of Use**: Very intuitive for both the business owner and their clients.
- **Pricing**: Free tier covers basic needs; Essentials is $8/user/month.
- **Cloud vs. Standalone**: API works flawlessly in both environments. Cloud can utilize webhooks for real-time booking updates; Standalone may require periodic sync.
**Design Doc**:
- **Trigger**: User pastes their Calendly API key or connects via OAuth in the OHC setup wizard.
- **Action**: OHC reads scheduled events and displays them in the OHC Dashboard Calendar widget.
- **User View**: A clean "Upcoming Appointments" view on the main dashboard, with a prominent button to copy their booking link to share with clients.
**Implementation Prompt**:
- Provide a connection interface for Calendly.
- Create a dashboard widget that displays upcoming booked appointments fetched from Calendly.
- Add a quick-copy button for the user's booking link.
- **Acceptance Criteria**: User connects Calendly, and upcoming meetings appear on the OHC dashboard accurately reflecting the Calendly schedule.
**Priority**: P1
**Estimated Scope**: Small

---

## 3. Email Marketing: Mailchimp

**Title**: [Marketing] Sync Customer Contacts to Mailchimp
**Problem Statement**: As small businesses grow their customer base in OHC, manually exporting contacts to send newsletters is time-consuming and prone to errors. They need their OHC customer list to automatically sync with their email marketing tool.
**Research Report**:
- **Tool Evaluated**: Mailchimp
- **Reputation**: Household name for small business email marketing.
- **Ease of Use**: Renowned for non-technical user friendliness and great templates.
- **Pricing**: Free up to 500 contacts; paid starts at $13/month.
- **Cloud vs. Standalone**: Strong REST API, fully functional in both modes.
**Design Doc**:
- **Trigger**: A new customer is added or updated in OHC.
- **Action**: OHC automatically pushes the contact details to a designated Mailchimp audience list.
- **User View**: A simple toggle in settings: "Keep Mailchimp Contacts in Sync." No complex mapping required.
**Implementation Prompt**:
- Build a one-way sync from OHC contacts to Mailchimp.
- Add an integration toggle in the UI.
- Handle API rate limits gracefully in the background.
- **Acceptance Criteria**: When a user adds a contact in OHC, it appears in their connected Mailchimp audience within 5 minutes.
**Priority**: P2
**Estimated Scope**: Medium

---

## 4. Payment Processing: Stripe

**Title**: [Payments] Enable Simple Invoice Payments via Stripe
**Problem Statement**: Small business owners need a secure, fast way to get paid for their services or products without setting up complex merchant accounts. They want to send a link and get money in their bank.
**Research Report**:
- **Tool Evaluated**: Stripe (specifically Payment Links / Invoicing)
- **Reputation**: Gold standard for developer experience and reliability.
- **Ease of Use**: Excellent dashboard, though the initial KYC setup can be daunting for some.
- **Pricing**: 2.9% + 30¢ per successful card charge (US). No monthly fees.
- **Cloud vs. Standalone**: Works securely in both. Standalone can redirect to Stripe hosted checkout pages to avoid handling sensitive card data locally.
**Design Doc**:
- **Trigger**: User clicks "Create Invoice" or "Request Payment" in OHC.
- **Action**: OHC generates a Stripe Payment Link and provides it to the user.
- **User View**: A "Request Payment" button that asks for an amount and a description, then immediately outputs a short link to text or email to the client.
**Implementation Prompt**:
- Integrate Stripe API to generate standard Payment Links.
- Create a simple form in OHC for amount and description.
- Display the generated link prominently for the user to copy.
- **Acceptance Criteria**: User enters $50 and "Consultation", clicks generate, and receives a working Stripe checkout link.
**Priority**: P0
**Estimated Scope**: Medium

---

## 5. Shipping & Logistics: Shippo

**Title**: [Shipping] Automated Shipping Label Generation with Shippo
**Problem Statement**: E-commerce and retail small businesses waste hours waiting in line at the post office. They need a way to instantly calculate shipping rates and print labels from home.
**Research Report**:
- **Tool Evaluated**: Shippo
- **Reputation**: Highly regarded multi-carrier shipping API.
- **Ease of Use**: Simple UI for businesses; abstracts away carrier-specific complexities.
- **Pricing**: Pay-as-you-go (5¢ per label) or $10/month for Pro.
- **Cloud vs. Standalone**: REST API functions perfectly in both environments.
**Design Doc**:
- **Trigger**: User selects an order in OHC and clicks "Create Shipping Label."
- **Action**: OHC sends package dimensions and destination to Shippo, retrieves rates, and generates a printable PDF label.
- **User View**: A shipping modal where the user inputs weight/size, sees the cheapest rate, and clicks "Print Label."
**Implementation Prompt**:
- Integrate Shippo's rate and label generation endpoints.
- Build a UI modal for package details and carrier selection.
- Surface the PDF label for easy printing.
- **Acceptance Criteria**: User can input package details, view real rates, and generate a valid dummy shipping label PDF in the UI.
**Priority**: P2
**Estimated Scope**: Large

---

## 6. SMS & Notifications: Twilio

**Title**: [SMS] Automated Appointment Reminders via Twilio
**Problem Statement**: No-shows cost small businesses significant revenue. Clients, especially those with lower technical proficiency or who don't check email often, need simple text message reminders before their appointments.
**Research Report**:
- **Tool Evaluated**: Twilio
- **Reputation**: Global leader in programmable SMS.
- **Ease of Use**: API is robust, but A2P 10DLC compliance (for US numbers) can be a hurdle for non-technical owners. We must abstract this complexity.
- **Pricing**: ~$0.0079 per message (US). Very cost-effective.
- **Cloud vs. Standalone**: Works in both, though Standalone requires securely storing the API credentials locally.
**Design Doc**:
- **Trigger**: An appointment is scheduled for a time 24 hours from now.
- **Action**: OHC triggers an SMS via Twilio to the client's phone number.
- **User View**: A settings toggle: "Send SMS reminders 24 hours before appointments." Users don't see the Twilio complexity.
**Implementation Prompt**:
- Implement a background job that checks for upcoming appointments.
- Integrate Twilio API to dispatch SMS messages.
- Add a settings toggle and a phone number field to the customer profile.
- **Acceptance Criteria**: A scheduled appointment triggers an SMS to the customer's phone number exactly 24 hours prior.
**Priority**: P1
**Estimated Scope**: Medium

---

## 7. Video Conferencing: Zoom

**Title**: [Video] Auto-Generate Zoom Links for Meetings
**Problem Statement**: Virtual consultants and tutors manually create Zoom links and paste them into calendar invites, which is tedious and prone to mistakes (e.g., sending the wrong link to the wrong client).
**Research Report**:
- **Tool Evaluated**: Zoom API
- **Reputation**: Ubiquitous for video conferencing.
- **Ease of Use**: Everyone knows how to join a Zoom call; the OAuth flow is standard.
- **Pricing**: Basic is free (40-min limit); Pro is $14.99/month.
- **Cloud vs. Standalone**: OAuth Server-to-Server is ideal for Cloud. For Standalone, PKCE OAuth flow is required.
**Design Doc**:
- **Trigger**: A new meeting is created in OHC with "Video Call" selected.
- **Action**: OHC calls the Zoom API to generate a unique meeting ID and join URL.
- **User View**: When creating a meeting, a single checkbox "Add Zoom Link." The link is automatically added to the meeting details and calendar invite.
**Implementation Prompt**:
- Implement Zoom OAuth integration.
- Add a checkbox to the meeting creation form.
- Automatically append the generated Zoom join URL to the meeting details view.
- **Acceptance Criteria**: User creates a meeting with the Zoom option checked, and a unique, valid Zoom join link is saved and displayed with the meeting details.
**Priority**: P2
**Estimated Scope**: Medium

---

## Next Steps
1. Prioritize P0 and P1 integrations (Stripe, ManyChat, Calendly, Twilio) for the upcoming sprint planning.
2. Review the Standalone-specific security requirements for storing API keys locally (e.g., using the `OHC_SQLITE_KEY` encrypted local SIPDB).
3. Begin UI mockups for the "Integrations" settings page adhering to the OHC Premium Design Standards (Glassmorphism CSS).