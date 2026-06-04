# Scout Research Report: Tool Integration Evaluation

## 1. Social Media Integration: Unified Inbox
**Title:** Integrate Unified Inbox for Instagram, Facebook, and WhatsApp
**Problem Statement:** Business owners like Maya receive orders and inquiries across multiple platforms. Managing these separately causes missed opportunities and slow response times.
**Research Report:** Meta's Graph API provides the necessary endpoints to aggregate messages from Instagram, Facebook, and WhatsApp. It is the industry standard for this use case. However, the OAuth process can be intimidating for non-technical users, requiring clear, guided onboarding.
**Next Steps:**
- Establish Meta Graph API dev application and configure webhooks.
- Build OAuth flow UI tailored to non-technical users.

## 2. Calendar & Scheduling: Booking Engine
**Title:** Integrate Cal.com for Service Booking and Calendar Sync
**Problem Statement:** Service providers need a way for customers to book time slots without back-and-forth emails, preventing double-booking with their personal calendars.
**Research Report:** Cal.com is an open-source scheduling tool with a robust API and embeddable booking pages. It is significantly more developer-friendly than building a scheduling engine from scratch and handles timezone complexity.
**Next Steps:**
- Provision Cal.com API tokens securely per tenant.
- Integrate Cal.com scheduling embed into the OHC frontend.

## 3. Email Marketing: Campaign Manager
**Title:** Integrate Resend for Transactional and Marketing Emails
**Problem Statement:** Business owners need to send beautiful, professional emails with high deliverability without managing complex tools like Mailchimp.
**Research Report:** Resend offers a developer-first API for sending emails with excellent deliverability and a React Email templating engine. It abstracts away the complexity of SMTP and domain warming.
**Next Steps:**
- Swap existing SMTP configurations for Resend.
- Develop standard email templates using React Email.

## 4. Payment Processing: Global Expansion
**Title:** Integrate Mercado Pago for LATAM Payments
**Problem Statement:** Users in LATAM require local payment methods (like PIX in Brazil) and local currency settlement that Stripe may not fully optimize.
**Research Report:** Mercado Pago is the dominant payment processor in Latin America, offering extensive support for local payment methods and local regulatory compliance.
**Next Steps:**
- Integrate Mercado Pago checkout flow.
- Ensure proper routing logic based on user geography.

## 5. Shipping & Logistics: Automated Fulfillment
**Title:** Integrate EasyPost for Multi-Carrier Shipping Integration
**Problem Statement:** Sellers of physical goods need to calculate shipping costs at checkout and print labels easily.
**Research Report:** EasyPost aggregates 100+ carriers into a single API. It handles rating, label generation, and tracking, drastically simplifying the logistics stack.
**Next Steps:**
- Implement EasyPost API for real-time checkout rates.
- Add label generation feature to order management dashboard.

## 6. SMS & Notifications: Global Reach
**Title:** Integrate Twilio for Global SMS Notifications
**Problem Statement:** Food cart operators and others need immediate, reliable SMS notifications for new pre-orders without relying on app connectivity.
**Research Report:** Twilio is the industry leader for programmable SMS. It offers global reach and high deliverability, though A2P 10DLC compliance adds complexity.
**Next Steps:**
- Set up Twilio SMS webhook triggers.
- Build onboarding flow to manage 10DLC compliance automatically.

## 7. Video Conferencing: Virtual Services
**Title:** Integrate Google Meet for Automated Virtual Appointments
**Problem Statement:** Tutors and service providers need a seamless way to generate video call links for booked lessons.
**Research Report:** Google Meet (via Google Workspace/Calendar API) is ubiquitous, free for basic use, and highly familiar to most users.
**Next Steps:**
- Integrate Google Calendar OAuth.
- Automatically attach Meet links to newly created booking events.
