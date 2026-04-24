# Research Report: Tool Integrations Scout

## Executive Summary
This research phase identified critical tool integrations needed to close feature gaps for non-technical small business owners using OneHumanCorp (OHC). We evaluated tools across seven core functional domains to expand the capabilities of OHC's AI agents. The selected integrations prioritize ease-of-use, white-labeling capabilities (keeping users in the OHC app), and strong developer APIs.

## Areas Evaluated & Recommendations

### 1. Social Media Integration
**Goal**: Unified DM inbox for the "Customer Success" agent.
**Evaluation**: ManyChat vs Buffer.
**Recommendation**: **ManyChat** (P0). ManyChat provides the deep conversational webhook infrastructure needed for AI agents to process and reply to Instagram and Facebook DMs in real-time. Buffer should be reserved for post-scheduling.

### 2. Calendar & Scheduling
**Goal**: Automated appointment booking and calendar sync.
**Evaluation**: Cal.com vs Acuity.
**Recommendation**: **Cal.com** (P0). Open-source, developer-friendly, and easy to white-label within the OHC ecosystem.

### 3. Email Marketing
**Goal**: Simple, beautiful email blasts sent by the "Marketing" agent.
**Evaluation**: Resend vs MailerLite.
**Recommendation**: **Resend** (P1). Resend provides robust API infrastructure. OHC's AI will generate the HTML, keeping the user interface completely within OHC rather than pushing users to a 3rd party drag-and-drop builder.

### 4. Shipping & Logistics
**Goal**: Real-time shipping rates and 1-click label generation.
**Evaluation**: Shippo vs ShipStation.
**Recommendation**: **Shippo** (P1). API-first design allows OHC to build label generation natively into the OHC order fulfillment flow without requiring a separate dashboard login.

### 5. SMS & Notifications
**Goal**: Reliable transactional alerts for users with poor internet connectivity.
**Evaluation**: Twilio vs Vonage.
**Recommendation**: **Twilio** (P0). Gold standard for reliability. Requires a streamlined UI flow to handle A2P 10DLC compliance for US users.

### 6. Video Conferencing
**Goal**: Auto-generated meeting links for online services.
**Evaluation**: Zoom vs Whereby.
**Recommendation**: **Zoom** (P2) via standard OAuth as the expected standard, combined with an embedded OHC-native option via **Whereby** for users lacking a Zoom account.

### 7. Payment Processing (Localization)
**Goal**: Provide alternative payment options beyond Stripe for global markets.
**Evaluation**: Mercado Pago (LATAM) vs Razorpay (India).
**Recommendation**: Build a provider interface in the Go backend to support **Mercado Pago** and **Razorpay** (P2), allowing OHC to adapt to the merchant's country during checkout.

## Next Steps
Detailed issue briefs have been added to the `docs/research/` directory. Implementation should prioritize P0 items (Social Media DMs, Calendar Scheduling, SMS Notifications) to unlock key capabilities for our core user personas.
