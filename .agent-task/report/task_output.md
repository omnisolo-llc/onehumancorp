## [Social Media] Manychat Integration
**Title**: Integrate Manychat for Unified Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.
**Research Report**:
- **Tool**: Manychat
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Excellent Instagram and WhatsApp API integrations. Robust webhook support for routing messages to OHC's backend. Extremely popular among SMBs for basic automation.
- **Risks**: Pricing scales with contacts, which may be expensive for high-volume, low-margin businesses. Requires Meta business verification for some features.
- **Pricing**: Free tier available (up to 1,000 contacts). Pro tier starts at $15/mo.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (would require local reverse proxy for webhooks, possible but complex).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Manychat's API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
**Implementation Prompt**: Implement an OAuth flow to connect a user's Instagram/Facebook account via Manychat. Create a webhook endpoint that receives incoming messages, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large


---


# Scout: Tool Integration Research Q2

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


---


## [Email Marketing] Issue Brief: AI-Generated Customer Broadcasts

**Title**: Scout 🔍: Integrate Resend for AI-Powered Email Marketing
**Problem Statement**:
Business owners like Priya want to notify their existing customers about new stock or holiday sales. Traditional tools like Mailchimp are too complex and require manual template design, list management, and campaign scheduling.
**Research Report**:
- **Tool**: Resend.
- **Evaluation**: Resend provides a developer-friendly, reliable email API. Instead of giving users a complex drag-and-drop builder, OHC can use the "Marketing" AI agent to generate beautiful HTML emails based on a simple text prompt from the user.
- **Ease of Use**: Zero-friction. The user types "Tell my customers about the new summer dress collection," and the AI generates the subject line, body, and inserts product photos automatically.
- **Pricing**: Resend charges around $20/mo for up to 50k emails, very economical to bundle into an OHC premium tier.
- **Cloud vs. Standalone**: Cloud mode uses OHC's centralized Resend account. Standalone mode requires the user to input their own SMTP credentials.
**Design Doc**:
- "Marketing" tab -> "Send a Broadcast".
- User provides a 1-sentence prompt.
- The AI Agent generates a responsive HTML email preview.
- User clicks "Send to all customers".
- The system chunks the customer list and sends via the Resend API.
**Implementation Prompt**:
Create a feature where the user can prompt the AI to draft an email blast. Use the business's product catalog to enrich the email. Provide a preview UI. Once approved, queue the emails to be sent out via the Resend API to all opted-in customers, handling rate limits and basic bounce tracking.
**Priority**: P2
**Estimated Scope**: Medium


---


## [Payment] Mercado Pago Integration
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil.
**Research Report**:
- **Tool**: Mercado Pago
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico). Good developer docs.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Variable by country (e.g., ~4-5% per transaction).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout.
- Webhooks update the order status in OHC when payment succeeds.
**Implementation Prompt**: Add Mercado Pago as a secondary payment provider. Implement the checkout flow to redirect to Mercado Pago and handle the success/failure webhooks to update order status.
**Priority**: P2
**Estimated Scope**: Large


---


## [Shipping] Shippo Integration
**Title**: Integrate Shippo for Automated Label Generation
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button in OHC to buy and print a label.
**Research Report**:
- **Tool**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Aggregates rates from USPS, UPS, FedEx, DHL. Simple API. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get rates.
- The Operations agent shows the cheapest shipping option.
- The user clicks "Buy Label", and OHC downloads the PDF label for printing.
- OHC automatically emails the customer the tracking number.
**Implementation Prompt**: Connect the Shippo API to fetch shipping rates based on order weight/dimensions. Allow the user to purchase a label and automatically email the tracking link to the customer.
**Priority**: P1
**Estimated Scope**: Large


---


## [SMS] Twilio Integration
**Title**: Integrate Twilio for SMS Order Notifications
**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable SMS alerts when a new pre-order arrives so she can start cooking.
**Research Report**:
- **Tool**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Global coverage, incredibly reliable. Programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).
**Design Doc**:
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, the Operations agent triggers a Twilio API call to send an SMS: "New order! 2x Falafel for John. Pickup in 15m."
- (Future: Customers can also receive SMS receipts).
**Implementation Prompt**: Integrate the Twilio SDK to send outbound SMS notifications. Add a setting for the business owner to opt-in to SMS alerts for new orders. Ensure compliance with local messaging regulations.
**Priority**: P2
**Estimated Scope**: Medium


---


# [Video] Whereby Integration

## Title
Zero-Friction Video Consultations with Whereby

## Problem Statement
Leo (Music Tutor) is tired of students struggling to download Zoom or Meet. He needs a "one-click" video room that opens directly in the browser for his lessons, without any software installation or account creation for him or the student. This removes the primary barrier to starting an online session.

## Research Report
- **Strategy**: Embed Whereby video rooms via their API/SDK.
- **Target Persona**: Leo (Music Tutor), Consultants, Online Teachers.
- **Advantages**: Purely browser-based (WebRTC) — no downloads required. Minimalist, high-quality UI that feels like part of the OHC platform. Extremely easy for non-technical users.
- **Risks**: Lower brand recognition than Zoom, but higher "ease of use" for first-time students.
- **Pricing**: Generous free tier for 1:1 rooms. Embedded/API plans available for scaling.
- **Ease of Use**: Highest in class. Click a link and you are in.
- **Compatibility**: Cloud & Standalone (Browser-based).

## Design Doc
- **Integration with OHC**:
    - When a service marked as "Online" is booked, OHC calls the Whereby API to create a unique, temporary room.
    - The room link is automatically sent to the customer and displayed in the merchant's "Meetings" dashboard.
    - Clicking the link opens the video call directly within a browser tab or an iframe in the OHC app.
- **User View**: A "Join Lesson" button in the dashboard that opens the video call instantly.

## Implementation Prompt
Integrate Whereby for native, browser-based video conferencing. Implement the logic to automatically generate unique room URLs for scheduled appointments. Display these links in the OHC "Meetings" dashboard and include them in customer confirmation and reminder notifications.

## Priority
P2

## Estimated Scope
Medium
