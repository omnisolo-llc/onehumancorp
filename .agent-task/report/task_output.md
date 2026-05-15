# Scout: Tool Integration Research Q3

## 1. Social Media Integration
**Title**: Integrate ManyChat / Meta Graph API for Unified Multi-Channel Inbox
**Problem Statement**: Fatima the Baker misses cake orders because she gets messages scattered across WhatsApp, Instagram DMs, and Facebook Messenger.
**Research Report**: Small businesses heavily rely on WhatsApp, Instagram, and Facebook Messenger. Meta's Graph API provides unified endpoints.
**Design Doc**: The user navigates to the 'Communications' tab and clicks 'Connect Social Accounts'. A standard OAuth flow redirects them to Meta.
**Implementation Prompt**: Build a unified inbox feature that allows users to connect their Meta accounts via OAuth.
**Priority**: P0
**Estimated Scope**: Large
**Key advantages and risks**: Advantage: Deep native integration. Risk: Meta API approval process is strict.
**Rough pricing estimate**: Meta APIs are mostly free. WhatsApp Business API charges ~$0.01 to ~$0.08 per conversation.
**Cloud and Standalone mode viability**: Yes. Cloud handles OAuth callbacks directly; Standalone requires a proxy or dynamic DNS setup.

## 2. Calendar & Scheduling
**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Leo the Music Tutor and Carlos the Handyman lose customers due to back-and-forth scheduling via text.
**Research Report**: Cal.com is an open-source, robust scheduling infrastructure. It handles timezone math and conflict resolution.
**Design Doc**: The user connects their Google/Outlook calendar via a one-click OAuth button.
**Implementation Prompt**: Embed Cal.com's infrastructure to allow users to sync their personal calendars.
**Priority**: P0
**Estimated Scope**: Medium
**Key advantages and risks**: Advantage: Open source, highly customizable. Risk: Self-hosting Cal.com infrastructure can be complex if not wrapped correctly.
**Rough pricing estimate**: Free for individuals. Self-hosted is free (just compute costs).
**Cloud and Standalone mode viability**: Yes. Perfect for Standalone as it can be entirely self-hosted alongside OHC.

## 3. Email Marketing
**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when new summer stock arrives.
**Research Report**: Listmonk is an open-source, self-hosted newsletter and mailing list manager.
**Design Doc**: Customer Success agent automatically tags customers. AI generates the HTML/text email content.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine for OHC.
**Priority**: P2
**Estimated Scope**: Medium
**Key advantages and risks**: Advantage: Very lightweight Go binary. Risk: Deliverability depends entirely on the chosen SMTP provider.
**Rough pricing estimate**: Listmonk is free. Users pay their own SMTP provider (e.g., SendGrid $19.95/mo or AWS SES $0.10/1k emails).
**Cloud and Standalone mode viability**: Yes. Native Go binary compiles easily for Standalone deployments.

## 4. Payment Processing
**Title**: Integrate Mercado Pago & Razorpay for Global Payment Accessibility
**Problem Statement**: Users like Diego in Argentina or Ananya in India face high failure rates or cannot use Stripe at all.
**Research Report**: Global small businesses rely heavily on regional payment gateways (e.g., Pix in Brazil, UPI in India).
**Design Doc**: Users see options based on their business country. The OHC backend normalizes the payment flow.
**Implementation Prompt**: Abstract the payment processing logic to support multiple providers. Implement Mercado Pago and Razorpay.
**Priority**: P1
**Estimated Scope**: Large
**Key advantages and risks**: Advantage: Unlocks massive new geographic markets. Risk: Handling multiple webhook formats increases surface area for bugs.
**Rough pricing estimate**: Providers charge transaction fees (typically 1.5% to 3.5% + fixed fee). OHC takes 0% cut.
**Cloud and Standalone mode viability**: Yes. Both Cloud and Standalone support standard payment integrations.

## 5. Shipping & Logistics
**Title**: Integrate Shippo/EasyPost for Automated Label Generation
**Problem Statement**: Emma the Potter spends 2 hours manually copying customer addresses into the USPS website.
**Research Report**: Shippo and EasyPost are API-first shipping aggregators. They provide real-time rates.
**Design Doc**: An unfulfilled order has a 'Create Shipping Label' button. OHC calls the API.
**Implementation Prompt**: Integrate a shipping aggregator API. Add a workflow to generate, purchase, and print shipping labels.
**Priority**: P1
**Estimated Scope**: Medium
**Key advantages and risks**: Advantage: Immediate access to hundreds of carriers. Risk: API latency during holiday spikes can break checkout flows.
**Rough pricing estimate**: Typically $0.05 per label or a $10-$20 monthly fee + carrier postage costs.
**Cloud and Standalone mode viability**: Yes. API integrations work seamlessly across both architectures.

## 6. SMS & Notifications
**Title**: Integrate Twilio for Global SMS Notifications
**Problem Statement**: Fatima the Baker has customers who don't use email and rely entirely on SMS.
**Research Report**: SMS remains the most reliable notification method globally. Twilio is the industry standard.
**Design Doc**: A 'Notifications' section allows users to toggle SMS on/off. OHC sends via Twilio API.
**Implementation Prompt**: Integrate Twilio to enable automated SMS notifications for critical order events.
**Priority**: P1
**Estimated Scope**: Medium
**Key advantages and risks**: Advantage: Near 100% open rate. Risk: Strict global telecom compliance laws (e.g., A2P 10DLC in the US).
**Rough pricing estimate**: Twilio charges roughly $0.0079 per message in the US, up to $0.10+ internationally.
**Cloud and Standalone mode viability**: Yes. Standalone users must provide their own Twilio API keys to manage costs.

## 7. Video Conferencing
**Title**: Integrate Zoom/Google Meet for Automated Consultation Links
**Problem Statement**: Sarah the Therapist spends time manually generating Zoom links for every booking.
**Research Report**: Zoom and Google Meet are ubiquitous. Video link generation is natively supported.
**Design Doc**: If a video option is selected, OHC prompts for OAuth. OHC generates a link.
**Implementation Prompt**: Extend the scheduling system to support dynamic video conferencing link generation.
**Priority**: P2
**Estimated Scope**: Small
**Key advantages and risks**: Advantage: Extreme customer familiarity. Risk: Token expiration forces frequent user re-authentication.
**Rough pricing estimate**: Basic usage is free. Paid tiers for longer group meetings start around $15/mo.
**Cloud and Standalone mode viability**: Yes. OAuth flows apply equally to Cloud and Standalone environments.
