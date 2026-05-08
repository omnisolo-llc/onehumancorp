# Research Report: Tool Integrations for Small Businesses

## Social Media Integration
- **Tool Evaluated**: ManyChat
- **Problem Solved**: Small business owners often miss inquiries and orders because they are spread across Instagram DMs, Facebook comments, WhatsApp, and TikTok. Manually checking each platform is time-consuming and prone to errors.
- **Findings & Risks**: ManyChat provides a unified inbox and automated responses for Instagram, Facebook, and WhatsApp. It is highly rated for ease of use by non-technical users. Pricing is affordable (starting at $15/mo). It works well for cloud multi-tenant setups, but local standalone support might require webhook relays.
- **User Experience**: The ManyChat integration will add a 'Unified Inbox' tab in the OHC dashboard. When a customer messages the business on Instagram/WhatsApp, the message appears in the Unified Inbox. The business owner can reply directly from OHC. Automated greeting rules can be configured visually.
- **Pricing**: $15/mo base
- **Deployment**: Cloud: Yes. Standalone: Requires webhook proxying.

## Calendar & Scheduling
- **Tool Evaluated**: Calendly
- **Problem Solved**: Booking consultations or service appointments involves endless back-and-forth emails. Small business owners need a simple way to let clients book available times without double-booking.
- **Findings & Risks**: Calendly is the industry standard. It handles Google/Outlook calendar sync, timezone math, and conflict resolution perfectly. It has a robust API and iframe embedding. Non-technical users understand the Calendly interface intuitively. Free tier available, paid starts at $10/mo.
- **User Experience**: Business owners can connect their Calendly account via OAuth. OHC will embed their Calendly booking page directly into their storefront or customer portal. Appointments booked via Calendly will sync back into OHC's internal CRM to track customer interactions.
- **Pricing**: Free tier; $10/mo premium
- **Deployment**: Cloud: Yes. Standalone: Yes (API driven).

## Email Marketing
- **Tool Evaluated**: Mailchimp
- **Problem Solved**: Business owners struggle to keep their customer lists synchronized between their store and their marketing tools, leading to missed opportunities to drive repeat sales.
- **Findings & Risks**: Mailchimp is widely recognized by small businesses. It offers excellent drag-and-drop template builders and robust list management. It handles spam compliance automatically. Pricing scales with contacts (Free up to 500 contacts, then starts at $13/mo).
- **User Experience**: A 'Marketing' tab in OHC where users connect Mailchimp. Customer data (opt-ins, purchases) automatically syncs from OHC to Mailchimp audiences. Business owners can view basic campaign stats (open rates, clicks) directly in OHC.
- **Pricing**: Free tier; $13/mo premium
- **Deployment**: Cloud: Yes. Standalone: Yes (API driven).

## Payment Processing
- **Tool Evaluated**: Mercado Pago
- **Problem Solved**: Stripe isn't always the best or most widely used option in Latin America. Business owners in LATAM need a familiar, trusted payment gateway that supports local payment methods and currencies.
- **Findings & Risks**: Mercado Pago is dominant in LATAM, supporting local cards, bank transfers, and cash payments (like OXXO in Mexico). It has a clear API. Settlement speed and fees vary by country but are competitive locally.
- **User Experience**: Add Mercado Pago as a payment provider option in the 'Settings > Payments' area. When enabled, checkout flows will redirect to Mercado Pago or use their transparent checkout. Payment statuses will be updated via secure webhooks.
- **Pricing**: Varies by country (~3-4% per transaction)
- **Deployment**: Cloud: Yes. Standalone: Yes (Webhook relays needed for local testing).

## Shipping & Logistics
- **Tool Evaluated**: Shippo
- **Problem Solved**: Calculating shipping rates manually and buying labels at the post office is incredibly tedious. Business owners need automated rate calculation and label printing.
- **Findings & Risks**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and provides discounted rates. The API is developer-friendly. Non-technical users benefit from a unified interface to buy and print labels. Pricing is pay-as-you-go ($0.05 per label) or $10/mo.
- **User Experience**: When an order is placed, OHC queries Shippo for shipping rates. In the OHC order management view, the owner sees a 'Buy Shipping Label' button. Clicking it purchases the label via Shippo and provides a printable PDF.
- **Pricing**: $0.05 per label or $10/mo
- **Deployment**: Cloud: Yes. Standalone: Yes (API driven).

## SMS & Notifications
- **Tool Evaluated**: Twilio
- **Problem Solved**: Many customers and business owners (like Fatima) rely on SMS rather than email for urgent updates or order confirmations due to lower tech-savviness or internet access issues.
- **Findings & Risks**: Twilio is the gold standard for programmatic SMS globally. It has excellent reliability and global coverage. The main challenge for non-technical users is registering for A2P 10DLC compliance in the US. Pricing is per message (e.g., $0.0079 in US).
- **User Experience**: OHC will manage a centralized Twilio account for Cloud users (reselling SMS) or allow Standalone users to input their own Twilio credentials. Owners can toggle 'Send SMS on Order Confirmation' in settings.
- **Pricing**: Pay per message (~$0.0079/msg)
- **Deployment**: Cloud: Yes (Central pool). Standalone: Yes (Bring your own keys).

## Video Conferencing
- **Tool Evaluated**: Zoom
- **Problem Solved**: Service providers offering online consultations struggle with manually creating and sending video links for every booking.
- **Findings & Risks**: Zoom is universally understood by consumers. The API allows automatic meeting creation. It pairs well with scheduling tools. Free tier has a 40-minute limit, Pro is $15/mo.
- **User Experience**: When a virtual service is booked, OHC calls the Zoom API to generate a unique meeting link. This link is automatically included in the confirmation email/SMS sent to the customer and displayed in the owner's dashboard.
- **Pricing**: Free tier; $15/mo premium
- **Deployment**: Cloud: Yes. Standalone: Yes (API driven).
