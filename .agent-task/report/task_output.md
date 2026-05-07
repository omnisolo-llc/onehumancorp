# Scout Tool Integration Research Report Q4

## Executive Summary
This report evaluates seven key integration categories designed to empower small business owners using One Human Corp (OHC). The focus is exclusively on tools that provide immediate, tangible value to non-technical users in both Cloud and Standalone environments.

## Evaluated Categories and Recommended Tools

### 1. Social Media Integration
- **Recommended Tool**: Chatwoot
- **Problem Solved**: Centralizes customer messages from Instagram, Facebook, and WhatsApp into a single inbox.
- **Why**: Chatwoot provides a truly unified inbox experience that mimics standard email/chat clients. It is open-source, affordable in the cloud, and supports self-hosting, aligning perfectly with OHC's dual-mode architecture.
- **Priority**: P1 | **Scope**: Large

### 2. Calendar & Scheduling
- **Recommended Tool**: Cal.com
- **Problem Solved**: Eliminates back-and-forth emails for booking appointments by providing a public, real-time availability link.
- **Why**: Cal.com is an open-source, highly customizable alternative to Calendly. It is free for individuals, integrates smoothly with Google/Outlook calendars, and can be embedded directly into OHC.
- **Priority**: P1 | **Scope**: Medium

### 3. Email Marketing
- **Recommended Tool**: Brevo
- **Problem Solved**: Allows business owners to send simple promotional blasts to their customer list without the complexity of traditional marketing platforms.
- **Why**: Brevo offers a generous free tier and an easy-to-use drag-and-drop builder. It avoids the bloat of Mailchimp while still providing essential open/click metrics.
- **Priority**: P2 | **Scope**: Medium

### 4. Payment Processing
- **Recommended Tools**: Mercado Pago & Alipay (Alternative Gateways)
- **Problem Solved**: Provides local payment options for international businesses where Stripe is unavailable or too expensive.
- **Why**: Expanding beyond Stripe is critical for global adoption. Integrating regional leaders like Mercado Pago (LATAM) and Alipay (Asia) ensures business owners can get paid in ways their customers trust.
- **Priority**: P1 | **Scope**: Large

### 5. Shipping & Logistics
- **Recommended Tool**: EasyPost
- **Problem Solved**: Automates the calculation of shipping rates and generation of carrier labels directly from an order.
- **Why**: EasyPost consolidates dozens of carriers behind a single API. This hides immense complexity from the user, allowing them to simply click "Buy Label" and print.
- **Priority**: P2 | **Scope**: Large

### 6. SMS & Notifications
- **Recommended Tool**: Twilio
- **Problem Solved**: Ensures critical messages (like appointment reminders) are seen immediately by sending them via SMS instead of easily ignored emails.
- **Why**: Twilio is the global standard for SMS delivery. Its reliability is unmatched, and setting up automated triggers in OHC will drastically reduce customer no-shows.
- **Priority**: P1 | **Scope**: Medium

### 7. Video Conferencing
- **Recommended Tool**: Zoom
- **Problem Solved**: Automatically generates a video meeting link when an online appointment is booked, saving manual effort and preventing forgotten links.
- **Why**: Zoom is the most universally understood video platform. Automating the link generation process via their API provides a seamless experience for both the business owner and their clients.
- **Priority**: P2 | **Scope**: Medium

## Cloud vs. Standalone Compatibility
All recommended tools were specifically chosen for their viability across OHC's deployment modes:
- **Cloud**: Integrations can be managed per-tenant, typically via OAuth flows or centrally managed API keys.
- **Standalone**: All tools support users supplying their own API keys or authenticating their personal accounts locally, maintaining the privacy and control guarantees of Standalone mode.

## Next Steps
The implementer should review the detailed issue briefs located in `docs/research/` and begin drafting technical architecture designs for the P1 integrations (Chatwoot, Cal.com, Mercado Pago/Alipay, and Twilio).