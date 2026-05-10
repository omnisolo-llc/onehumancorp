# Integration Research Report: Expanding OHC Capabilities

## Overview
To fulfill the OneHumanCorp (OHC) promise of zero-technical-friction business management, we have identified and researched 7 key tool integrations. These tools address critical pain points across all core business personas (Maya the baker, Carlos the handyman, Priya the boutique owner, Leo the tutor, and Fatima the food cart operator).

Each tool was evaluated based on its ease of use for non-technical users, pricing viability for small businesses, and ability to seamlessly integrate into the OHC "Department" architecture.

Detailed issue briefs have been generated for each tool and stored in `docs/research/`.

---

## Evaluated Tools & Findings

### 1. Social Media Integration: Meta Graph API
- **Target Persona**: Maya (Baker), Priya (Boutique)
- **Problem**: Business owners are overwhelmed by managing DMs across Instagram, Facebook, and WhatsApp.
- **Solution**: A unified inbox within the OHC "Customer Success" dashboard, allowing the AI Ambassador to draft replies.
- **Finding**: Meta's Graph API is the most robust way to achieve this. OAuth flow is standard and easy for users to complete.
- **Priority**: P0 | **Scope**: Large

### 2. Calendar & Scheduling: Google Calendar API
- **Target Persona**: Carlos (Handyman), Leo (Tutor)
- **Problem**: OHC bookings conflict with personal calendar events.
- **Solution**: Two-way sync to read free/busy times and push new OHC bookings directly to the owner's Google Calendar.
- **Finding**: Google Calendar API provides reliable `freebusy` endpoints. Generous free tier makes it highly viable.
- **Priority**: P0 | **Scope**: Medium

### 3. Email Marketing: Resend
- **Target Persona**: Priya (Boutique), Leo (Tutor)
- **Problem**: Existing tools like Mailchimp are too complex for non-technical owners wanting to send simple updates.
- **Solution**: A native, simplified "Send Campaign" button inside OHC powered by Resend.
- **Finding**: Resend offers excellent developer experience, high deliverability, and a generous free tier (3,000 emails/month) perfectly suited for OHC's target market.
- **Priority**: P1 | **Scope**: Medium

### 4. Payment Processing: Mercado Pago
- **Target Persona**: All LATAM-based personas
- **Problem**: Stripe lacks penetration and support for local payment methods (e.g., PIX, OXXO) in Latin America.
- **Solution**: Integrate Mercado Pago as a secondary, region-specific payment provider in the Finance department.
- **Finding**: Mercado Pago is the undisputed leader in LATAM. Its web tokenized checkout and webhook architecture map well to our existing payment flow.
- **Priority**: P1 | **Scope**: Large

### 5. Shipping & Logistics: Shippo
- **Target Persona**: Priya (Boutique), Maya (Baker)
- **Problem**: Manually buying shipping labels and copying tracking numbers is extremely tedious.
- **Solution**: In-app rate calculation and label purchasing (PDF generation) directly from the Order Details page.
- **Finding**: Shippo API aggregates all major carriers and handles rate/label logic efficiently for just 5¢ per label.
- **Priority**: P1 | **Scope**: Large

### 6. SMS & Notifications: Twilio
- **Target Persona**: Fatima (Food Cart)
- **Problem**: Customers often miss email notifications when picking up local orders. Immediate notification is required.
- **Solution**: Automated SMS dispatch when an order is marked "Ready for Pickup".
- **Finding**: Twilio Programmable SMS is the industry standard, offering high reliability and low per-message cost, seamlessly integrating into the Customer Success agent's workflow.
- **Priority**: P0 | **Scope**: Medium

### 7. Video Conferencing: Zoom API
- **Target Persona**: Leo (Tutor)
- **Problem**: Manually generating and distributing Zoom links for online bookings leads to errors and delays.
- **Solution**: Auto-generate a Zoom meeting via the user's connected Zoom account upon booking confirmation.
- **Finding**: The Zoom API supports OAuth and allows meeting creation even on basic/free accounts, making it highly accessible for our users.
- **Priority**: P2 | **Scope**: Medium

---

## Next Steps
1. **Approval**: Review and prioritize these integration issue briefs.
2. **Implementation Planning**: Assign P0 issues (Meta Inbox, Google Calendar, Twilio SMS) to the implementation queue for technical design and DB schema updates.
3. **Execution**: Implement following the atomic PR and E2E testing standards outlined in the OHC engineering guidelines.
