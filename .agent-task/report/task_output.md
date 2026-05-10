# Tool Integration Research Q4

## [Social Media Integration] WhatsApp Business API

**Problem Statement**: Small business owners (like retail shop owners or service providers) receive customer inquiries on WhatsApp but struggle to manage them alongside other messages. They need a single place to see and reply to WhatsApp messages so they don't miss customer requests or lose track of conversations, especially when multiple staff members need to respond.

**Research Report**:
- **Target Persona**: Small business owners, retailers, service providers with high WhatsApp usage.
- **Ease of Use**: Direct integration for users might be complex without an interface like OHC. Meta provides the Cloud API which allows sending and receiving messages.
- **Pricing**: WhatsApp charges per conversation (user-initiated vs. business-initiated). OHC could pass these costs or include a tier.
- **Reputation/Reliability**: Meta's official API is reliable but has strict opt-in rules and template approvals for business-initiated messages.
- **Cloud/Standalone**: Works in Cloud via Webhooks. For Standalone, it requires a secure webhook endpoint exposed to the internet (which could be a limitation or require a tunneling solution).

**Design Doc**:
- **Trigger**: Customer sends a WhatsApp message to the business's registered number.
- **Action**: OHC receives the message via Webhook, parses it, and displays it in the unified inbox.
- **User View**: Business owner sees WhatsApp messages in the same inbox as emails and other DMs. They can reply directly from OHC, and the response is sent back via the WhatsApp API.
- **Integration**: Requires OAuth/Meta Business Login to connect the account.

**Implementation Prompt**: Implement a connection flow for business owners to link their WhatsApp Business account. Once linked, incoming WhatsApp messages should appear in the unified inbox, and replies from the inbox should be delivered back to the customer on WhatsApp.

**Priority**: P1
**Estimated Scope**: Large


---

## [Calendar & Scheduling] Calendly Integration

**Problem Statement**: Service-based businesses (consultants, salons, tutors) spend too much time going back and forth with clients to find a suitable meeting time. They need a way to share a booking link that automatically syncs with their availability.

**Research Report**:
- **Target Persona**: Consultants, coaches, salons, any appointment-based business.
- **Ease of Use**: Calendly is widely known and very user-friendly. Connecting it via OAuth is straightforward.
- **Pricing**: Calendly has a free tier, but team or advanced features require paid plans (~$10-$15/mo).
- **Reputation**: Industry standard, highly reliable.
- **Cloud/Standalone**: Works in both. Cloud can handle webhooks for new bookings. Standalone can poll or use webhooks if exposed.

**Design Doc**:
- **Trigger**: User connects Calendly account. Client books a meeting via Calendly link.
- **Action**: OHC creates a new contact or updates an existing one when a booking is made. A notification is added to the dashboard.
- **User View**: Business owner sees upcoming Calendly appointments on their OHC dashboard and can view client details linked to the booking.

**Implementation Prompt**: Add a Calendly integration where users can connect their account. Display upcoming Calendly events on the OHC dashboard and automatically add/update customer records based on new bookings.

**Priority**: P2
**Estimated Scope**: Medium


---

## [Email Marketing] Mailchimp Integration

**Problem Statement**: Business owners collect customer emails but find it hard to keep their email marketing lists in sync with their actual customer database. They need new contacts added to OHC to automatically flow into their newsletter tool.

**Research Report**:
- **Target Persona**: Retailers, online stores, content creators.
- **Ease of Use**: Mailchimp is very popular. OAuth integration is standard.
- **Pricing**: Free tier available (up to a certain number of contacts/sends).
- **Reputation**: Very established, though pricing changes have frustrated some small users.
- **Cloud/Standalone**: Works in both via standard API calls.

**Design Doc**:
- **Trigger**: A new customer is added to OHC (e.g., via a purchase or contact form).
- **Action**: OHC pushes the contact information to a designated Mailchimp audience.
- **User View**: Business owner selects a Mailchimp list in settings. They don't need to manually export/import CSVs anymore.

**Implementation Prompt**: Create a Mailchimp integration that allows the business owner to authenticate and select an audience. Automatically sync new contacts created in OHC to the selected Mailchimp audience.

**Priority**: P1
**Estimated Scope**: Medium


---

## [Payment Processing] Mercado Pago

**Problem Statement**: Small businesses in Latin America often cannot use Stripe or prefer local payment methods (like Pix in Brazil or local credit cards). They need a payment processor that works seamlessly in their region to accept online payments.

**Research Report**:
- **Target Persona**: LATAM-based small businesses.
- **Ease of Use**: Mercado Pago is widely used in LATAM and offers good merchant tools.
- **Pricing**: Varies by country, typically a percentage + fixed fee per transaction.
- **Reputation**: Dominant player in LATAM e-commerce.
- **Cloud/Standalone**: Works in both, though webhooks for payment confirmation are required.

**Design Doc**:
- **Trigger**: Customer initiates checkout on an OHC-powered storefront.
- **Action**: OHC redirects to Mercado Pago checkout or uses their API to process the payment.
- **User View**: Business owner can select Mercado Pago as an alternative to Stripe in the payment settings.

**Implementation Prompt**: Add Mercado Pago as a payment provider option. Allow merchants to connect their Mercado Pago credentials so customers can select it at checkout.

**Priority**: P2
**Estimated Scope**: Large


---

## [Shipping & Logistics] Shippo Integration

**Problem Statement**: Online sellers spend hours manually copying addresses to carrier websites to buy shipping labels. They need to generate shipping labels directly from their order dashboard to save time.

**Research Report**:
- **Target Persona**: E-commerce and physical product sellers.
- **Ease of Use**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) into one API.
- **Pricing**: Pay-as-you-go per label fee (e.g., 5 cents) + postage cost. Very friendly for small businesses.
- **Reputation**: Highly regarded for easy integration and good rates.
- **Cloud/Standalone**: API-based, works well in both environments.

**Design Doc**:
- **Trigger**: Business owner marks an order as "Ready to Ship".
- **Action**: OHC requests a shipping label from Shippo and provides it for download.
- **User View**: A "Buy Shipping Label" button appears on order details. Owner confirms package weight/size and clicks to generate a printable PDF label.

**Implementation Prompt**: Integrate Shippo to allow merchants to generate and download shipping labels directly from the order management screen in OHC.

**Priority**: P1
**Estimated Scope**: Large


---

## [SMS & Notifications] Twilio SMS

**Problem Statement**: Businesses need to reach customers who don't regularly check email (e.g., for appointment reminders or urgent updates). They need a way to send text messages directly from their management dashboard.

**Research Report**:
- **Target Persona**: Local services, salons, clinics, retail.
- **Ease of Use**: Twilio is a developer tool, so OHC must abstract it completely. The business owner should just type a message and hit send.
- **Pricing**: Pay-per-message (fractions of a cent to a few cents depending on country). OHC might need a billing mechanism or allow users to bring their own Twilio keys (though BYOK is complex for grandmas).
- **Reputation**: Industry standard, extremely reliable global coverage.
- **Cloud/Standalone**: API-based, works in both.

**Design Doc**:
- **Trigger**: Business owner types an SMS in the unified inbox or an automated reminder is triggered.
- **Action**: OHC sends the payload to Twilio API.
- **User View**: SMS appears as just another channel in the unified inbox. Owner can select "SMS" when sending a message to a customer.

**Implementation Prompt**: Enable sending and receiving SMS messages via Twilio. Surface SMS as a communication channel in the unified inbox.

**Priority**: P2
**Estimated Scope**: Medium


---

## [Video Conferencing] Zoom Integration

**Problem Statement**: Tutors, consultants, and online coaches manually create Zoom links and email them to clients for every session. They need this process automated so a link is generated as soon as a session is booked.

**Research Report**:
- **Target Persona**: Tutors, online coaches, remote consultants.
- **Ease of Use**: Zoom OAuth is standard.
- **Pricing**: Free tier has 40-minute limits. Paid tiers required for longer meetings.
- **Reputation**: Ubiquitous.
- **Cloud/Standalone**: Works via standard API calls.

**Design Doc**:
- **Trigger**: A new online meeting is scheduled in OHC.
- **Action**: OHC calls Zoom API to create a meeting and stores the join URL.
- **User View**: When scheduling an appointment, the owner checks "Make it a Zoom meeting" and the join link is automatically added to the calendar invite sent to the customer.

**Implementation Prompt**: Integrate Zoom so that when users schedule a virtual appointment or event, OHC automatically generates a Zoom meeting link and includes it in the invitation details.

**Priority**: P2
**Estimated Scope**: Medium
