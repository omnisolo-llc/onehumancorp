# OHC Tool Integration Research Report Q4

## Overview
This report evaluates seven tool categories essential for small business operations, identifying the best-in-class solutions to integrate with the OHC platform. The focus is strictly on tools that solve real, daily problems for non-technical business owners, optimizing for ease of use, reliability, and compatibility with both OHC's Cloud and Standalone environments.

## Category 1: Social Media Integration
**Selected Tool**: ManyChat
**Problem Solved**: Small business owners lose time and potential sales checking messages across Instagram, Facebook, and WhatsApp.
**Evaluation**: ManyChat is the industry leader for chat automation. It provides robust webhooks and APIs to aggregate messages. While the setup for a unified inbox requires solid backend engineering, it presents a highly intuitive solution for the business owner.
**Integration Design**: Connect via OAuth. OHC receives webhooks for new messages and provides a unified "Messages" tab where the owner can reply directly.
**Pricing**: Free tier available; Pro starts at $15/month.
**Compatibility**: Cloud (excellent). Standalone (requires polling or webhook tunnels).

## Category 2: Calendar & Scheduling
**Selected Tool**: Calendly
**Problem Solved**: Back-and-forth communication to find meeting times is inefficient and leads to double-booking.
**Evaluation**: Calendly is the industry standard. It's universally understood and offers exceptional reliability.
**Integration Design**: Connect via OAuth. OHC receives webhooks on new bookings, automatically creating "Booking" records in the CRM and optionally triggering notifications.
**Pricing**: Free tier available; Paid plans ~$10-$12/user/month.
**Compatibility**: Cloud (excellent). Standalone (requires polling if webhooks are blocked).

## Category 3: Email Marketing
**Selected Tool**: Mailchimp
**Problem Solved**: Managing email lists manually via CSVs is tedious. Owners need a simple way to email their existing customer base.
**Evaluation**: Mailchimp offers a very user-friendly builder and robust APIs for audience management.
**Integration Design**: Connect via OAuth. OHC automatically syncs the local CRM "Customers" list to a Mailchimp Audience, applying tags based on customer data.
**Pricing**: Free for small lists; scales with contacts.
**Compatibility**: Excellent for both Cloud and Standalone (outbound API calls).

## Category 4: Payment Processing
**Selected Tool**: Stripe
**Problem Solved**: Manual invoicing and tracking payments is stressful and time-consuming.
**Evaluation**: Stripe is the gold standard for developer experience and provides a seamless checkout flow for customers.
**Integration Design**: Cloud uses Stripe Connect; Standalone uses direct API keys. OHC generates payment links via Stripe Checkout and listens for webhooks to mark invoices as 'Paid'.
**Pricing**: 2.9% + 30¢ per transaction.
**Compatibility**: Excellent for both Cloud and Standalone.

## Category 5: Shipping & Logistics
**Selected Tool**: ShipEngine
**Problem Solved**: Calculating rates and typing labels manually for physical goods is slow and error-prone.
**Evaluation**: ShipEngine abstracts multiple carriers into a single reliable API.
**Integration Design**: OHC fetches rates based on package details. The owner clicks "Buy Label", and OHC generates a printable PDF and saves the tracking number.
**Pricing**: Pay-as-you-go (cents per label) + carrier fees.
**Compatibility**: Excellent for both Cloud and Standalone.

## Category 6: SMS & Notifications
**Selected Tool**: Plivo
**Problem Solved**: Many customers (especially those with lower English proficiency) prefer SMS for updates. Owners need a reliable way to send these.
**Evaluation**: Plivo is a strong, cost-effective Twilio alternative with excellent global delivery.
**Integration Design**: Cloud integration managed by OHC platform; Standalone users provide their own Auth ID/Token. System automatically sends SMS for key events (e.g., appointments, shipping).
**Pricing**: Pay-as-you-go.
**Compatibility**: Excellent for both Cloud and Standalone.

## Category 7: Video Conferencing
**Selected Tool**: Zoom
**Problem Solved**: Manually creating and sharing video links for online consultations is tedious.
**Evaluation**: Zoom is ubiquitous and reliable.
**Integration Design**: Connect via OAuth. When an online appointment is booked, OHC calls the Zoom API, generates a link, and attaches it to the appointment record.
**Pricing**: Free tier (40-min limits); Pro starts at $15/month.
**Compatibility**: Excellent for both Cloud and Standalone.

## Next Steps
1. Review the generated issue briefs in `docs/research/`.
2. Prioritize implementations based on user demand (suggesting Stripe and ManyChat as P0/P1).
3. Assign implementer agents to design the specific technical architectures (API endpoints, DDL) for the prioritized briefs.
