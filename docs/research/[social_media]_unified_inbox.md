# Unified Social Inbox Integration

## Problem Statement
Small business owners miss critical customer inquiries because they are scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok comments. Checking multiple apps constantly is stressful and inefficient, leading to lost sales and poor customer service.

## Research Report
**Competitive Landscape:**
1. **Meta Graph API (Direct):** Direct integration with Facebook/Instagram. Free, but complex OAuth and approval process.
2. **ManyChat:** Popular among SMBs for automation, but can be overwhelming and expensive at scale.
3. **Ayrshare:** Good API for posting, but less focus on unified inboxing.
4. **Twilio / WhatsApp Business API:** Essential for WhatsApp, but requires technical setup.

**Evaluation:**
- **Ease of Use:** Must be 1-click connect. Meta's embedded signup is the gold standard here.
- **Pricing:** Direct API is free, aggregators charge $15-$50/mo.
- **Cloud vs Standalone:** Cloud can use central OAuth apps. Standalone needs a proxy or user-provided credentials (harder for SMBs).

## Design Doc
- **Trigger:** User connects social accounts via OHC Settings > Integrations.
- **Action:** OHC subscribes to webhooks for new messages/comments. Incoming messages are routed to a 'Unified Inbox' UI.
- **User Experience:** A single chat interface in OHC where the owner can reply, and the message is routed back to the correct social platform.

## Implementation Prompt
Create a 'Unified Inbox' feature. The user should see a single list of conversations from all connected social channels. They should be able to click 'Connect Instagram', go through an OAuth flow, and instantly see new DMs appear in OHC. Replies sent from OHC must appear in the customer's Instagram app. Ensure a fallback mechanism if the API is down.

## Priority
P0

## Estimated Scope
Large
