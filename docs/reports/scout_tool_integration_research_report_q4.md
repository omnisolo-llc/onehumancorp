# Scout: Tool Integration Research Q4

This report evaluates seven critical integrations aimed at solving real-world pain points for small business owners using One Human Corp (OHC) in both Cloud and Standalone environments.

## 1. Social Media: Meta Graph API (Unified Inbox)
Small business owners often miss messages scattered across Instagram, Facebook, and WhatsApp. Integrating the Meta Graph API allows OHC to consolidate these channels into a single unified inbox. Users can reply to all messages from one place, ensuring no lead is missed. This requires a standard OAuth flow and webhook processing, which is fully viable in both Cloud and Standalone modes.

## 2. Calendar & Scheduling: Cal.com API
Service-based businesses waste time playing phone tag to book appointments. Integrating Cal.com allows them to generate a branded booking link connected to their Google/Outlook calendar. The API is robust, open-source, and handles timezone complexities efficiently, offering a seamless booking experience that automatically updates their OHC schedule.

## 3. Email Marketing: Resend API
Traditional email marketing tools are often too complex for simple customer updates (e.g., holiday hours, flash sales). Integrating the Resend API allows users to send simple, plain-text blasts directly from the OHC CRM. It abstracts away DNS complexity (managed by OHC in Cloud mode) and provides an easy-to-use text editor for composing broadcasts.

## 4. Payment Processing: Mercado Pago
To capture markets in Latin America, businesses need local payment methods (Pix, OXXO, local installments) that Stripe does not deeply support. Integrating Mercado Pago directly into OHC invoices unlocks these regions, significantly improving checkout conversion rates for LATAM-based small businesses.

## 5. Shipping & Logistics: Shippo API
E-commerce businesses spend excessive time manually copying addresses to generate shipping labels. Integrating Shippo allows instant rate calculation and label generation directly from an OHC order screen. It provides deep carrier discounts and automates tracking number delivery to the customer.

## 6. SMS & Notifications: Twilio API
Many customers of small businesses prefer text messages over email. Missed emails lead to no-shows. Integrating Twilio allows OHC to send automated SMS reminders for appointments and order pickups. While A2P 10DLC compliance in the US presents a hurdle, the value in reducing no-shows is massive.

## 7. Video Conferencing: Zoom API
Tutors and consultants waste time manually creating and sending Zoom links. Integrating the Zoom API allows OHC to automatically generate a unique meeting room for every virtual booking. The link is automatically added to the OHC schedule and emailed to the client, providing a completely hands-off experience.

## Next Steps
The corresponding issue briefs have been added to the `docs/research/` directory. Implementation should prioritize the P0 issues (Meta Integration, Twilio SMS) followed by P1 (Cal.com, Mercado Pago) to deliver immediate value to the core user personas.
