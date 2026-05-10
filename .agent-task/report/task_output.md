# Scout: Tool Integration Research Q2

## Overview
This report details the evaluation of tool integrations across seven critical categories to expand OHC's capabilities for small business owners in both Cloud and Standalone environments. The focus is on tools that offer direct, immediate value to non-technical users while preserving OHC's Radical Simplicity rule.

## 1. Social Media Integration
**Category**: Social Media Integration
**Strategy**: Native integration with WhatsApp Business API.
**Target Persona**: Fatima (Food Cart Operator), Priya (Boutique Owner)
**Advantages**: WhatsApp is the default communication method in many emerging markets. A unified inbox within OHC prevents lost orders.
**Risks**: Meta's business verification process can be complex. Strict template rules for outbound messages.
**Pricing**: Per-conversation pricing; requires OHC to manage quotas or pass costs.
**Compatibility**: Cloud (Centralized Webhooks). Standalone (User provided API token).

## 2. Calendar & Scheduling
**Category**: Calendar & Scheduling
**Strategy**: Cal.com integration for zero-config booking.
**Target Persona**: Leo (Music Tutor), Carlos (Handyman)
**Advantages**: Open-source, handles timezone/conflict resolution out of the box, embeddable.
**Risks**: Managing user OAuth tokens for Google/Outlook.
**Pricing**: Free tier available, highly scalable.
**Compatibility**: Cloud (SaaS integration). Standalone (Self-hosted or direct API).

## 3. Email Marketing
**Category**: Email Marketing
**Strategy**: Native Campaign Manager via SendGrid/SES.
**Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
**Advantages**: Keeps users in OHC; AI Marketing agent can fully automate campaigns natively.
**Risks**: Building list management and handling unsubscribes internally is complex.
**Pricing**: Predictable transactional API costs (included in OHC platform costs).
**Compatibility**: Cloud (Centralized account). Standalone (Centralized routing).

## 4. Payment Processing
**Category**: Payment Processing
**Strategy**: Mercado Pago native integration.
**Target Persona**: Global users outside the US/EU (e.g., LATAM).
**Advantages**: Provides trusted local payment methods (Pix, Pago Fácil) natively without third-party routing.
**Risks**: Slower settlement times; less standardized API compared to Stripe.
**Pricing**: Standard transaction fees apply.
**Compatibility**: Cloud (Webhooks). Standalone (API Keys / Webhooks).

## 5. Shipping & Logistics
**Category**: Shipping & Logistics
**Strategy**: Native shipping rates and labels via Shippo.
**Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
**Advantages**: One-click label generation natively in OHC. Extremely high value proposition.
**Risks**: Customs declarations for international shipping can be difficult to automate for non-technical users.
**Pricing**: Free tier, nominal per-label fee.
**Compatibility**: Cloud (OAuth). Standalone (API Key).

## 6. SMS & Notifications
**Category**: SMS & Notifications
**Strategy**: Native SMS order notifications via Twilio.
**Target Persona**: Fatima (Food Cart Operator)
**Advantages**: Reliable delivery even in noisy environments or low-data areas.
**Risks**: US A2P 10DLC compliance requires business registration, a potential barrier.
**Pricing**: Pay-per-message (requires credit system).
**Compatibility**: Cloud (Centralized Twilio account). Standalone (User API key).

## 7. Video Conferencing
**Category**: Video Conferencing
**Strategy**: Native Zoom link generation.
**Target Persona**: Leo (Music Tutor)
**Advantages**: Automated, professional, intuitive.
**Risks**: Annual Zoom app review and compliance.
**Pricing**: API is free, but merchant needs a Zoom account.
**Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).

## Next Steps
Create corresponding issue briefs in `docs/research/` to guide the implementation teams, ensuring they focus on the "what" (user experience) and not the "how" (technical implementation details).
