# Scout: Tool Integration Research

## [Social Media] Issue Brief: WhatsApp Business Integration
**Title**: Integrate WhatsApp Business API for Unified Customer Messaging
**Problem Statement**:
Small business owners like Fatima (Food Cart Operator) and users in the Global South rely on WhatsApp as their primary communication tool with customers. Switching between OHC and WhatsApp is a friction point that leads to missed orders. They need a single place to manage all conversations.

**Research Report**:
- **Tool**: WhatsApp Business API (via Meta Graph API).
- **Evaluation**:
  - **Ease of Use**: Medium. Requires a Meta Business Manager account and business verification, which OHC can help streamline via "Embedded Signup".
  - **Pricing**: Conversation-based pricing. Meta provides 1,000 free service conversations per month, which covers most OHC small business users.
  - **Reputation**: The gold standard for business messaging globally.
  - **Cloud vs. Standalone**: Cloud-native (requires OHC to host a Meta App). Standalone mode would require a specialized proxy or the user providing their own WABA credentials.
- **Key Advantages**: 98% open rates compared to 20% for email. Essential for markets in LATAM, India, and SE Asia.
- **Risks**: Strict template requirements for outbound-initiated messages.

**Design Doc**:
- **User Flow**: User clicks "Connect WhatsApp" in the OHC Operations dashboard. They complete the Meta Embedded Signup flow.
- **Integration**: OHC receives messages via Meta webhooks.
- **User Experience**: Messages appear in the OHC unified inbox. "The Ambassador" AI drafts responses based on product catalog and business hours.
- **Triggers**: Incoming WhatsApp message -> Webhook -> AI Draft -> Dashboard Notification.

**Implementation Prompt**:
Implement the WhatsApp Business API integration using the Meta Graph API. Support the OAuth/Embedded Signup flow for merchants. Create a webhook handler to ingest incoming messages into the OHC unified inbox and trigger the "Ambassador" agent to draft replies. Ensure support for media (images) sent by customers.

**Priority**: P0
**Estimated Scope**: Medium
