# OHC Scout: Tool Integration Research Report

## Executive Summary
This report summarizes the research and evaluation of seven key external tools aimed at extending the capabilities of the OneHumanCorp (OHC) platform. The tools evaluated span Social Media, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS Notifications, and Video Conferencing.

The focus of this research is strictly on the *non-technical small business owner* persona (e.g., Maya the Baker, Carlos the Handyman), prioritizing ease of integration, zero-jargon setup, and immediate business value.

## Evaluated Integrations

1. **Social Media: TikTok**
   - **Target Persona**: Creative Portfolios, Digital Products, Boutiques.
   - **Value**: Allows users to manage high-engagement Gen Z/Millennial traffic and auto-reply to comments directly from the OHC unified inbox.
   - **Status**: Brief created (`docs/research/[social_media]_tiktok.md`).

2. **Calendar & Scheduling: Microsoft Outlook**
   - **Target Persona**: Service Providers, Consultants.
   - **Value**: Prevents double-booking by providing two-way synchronization between OHC bookings and the professional's primary MS Outlook calendar.
   - **Status**: Brief created (`docs/research/[calendar]_outlook.md`).

3. **Email Marketing: Mailchimp**
   - **Target Persona**: All personas, especially those with existing customer lists.
   - **Value**: Seamlessly syncs OHC customer data to Mailchimp audiences, enabling powerful, familiar email campaigns without manual exports.
   - **Status**: Brief created (`docs/research/[email_marketing]_mailchimp.md`).

4. **Payment Processing: Razorpay**
   - **Target Persona**: Indian SMBs.
   - **Value**: Critical alternative to Stripe for the Indian market, enabling UPI, local cards, and net banking support.
   - **Status**: Brief created (`docs/research/[payment]_razorpay.md`).

5. **Shipping & Logistics: EasyPost**
   - **Target Persona**: Physical Product Sellers (Bakers, Boutiques).
   - **Value**: Abstracted multi-carrier shipping. Generates live rates and printable labels directly within the OHC fulfillment flow.
   - **Status**: Brief created (`docs/research/[shipping]_easypost.md`).

6. **SMS & Notifications: MessageBird**
   - **Target Persona**: Time-sensitive businesses (Food Carts, Services).
   - **Value**: Provides immediate SMS alerts for new orders to owners and delivery/status updates to customers.
   - **Status**: Brief created (`docs/research/[sms]_messagebird.md`).

7. **Video Conferencing: Zoom**
   - **Target Persona**: Online Service Providers (Tutors, Consultants).
   - **Value**: Automatically generates unique video meeting links upon booking, embedding them in confirmation emails and calendar invites.
   - **Status**: Brief created (`docs/research/[video]_zoom.md`).

## Next Steps
- Engineering teams to review the attached issue briefs for implementation feasibility.
- Prioritize P1 integrations (TikTok, Outlook, Razorpay, EasyPost, Zoom) for the upcoming sprint planning.
- Secure necessary API keys and sandbox environments for the selected tools.
