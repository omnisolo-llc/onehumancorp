# Social Media Integration: Unified Inbox

## Problem Statement
Small business owners often miss customer inquiries spread across Instagram DMs, Facebook comments, and WhatsApp messages. Checking multiple apps constantly is distracting and leads to lost sales when responses are delayed. They need a single place to see and reply to all customer messages.

## Research Report
**Selected Tools:** Meta Graph API & WhatsApp Business API
We evaluated Meta's official APIs against third-party aggregators. While aggregators simplify integration, they often add significant costs and latency. Direct integration with Meta APIs provides the most robust solution.
- **Ease of use for non-technical users:** Once connected via OAuth, the experience is seamless—messages just appear in OHC. The initial setup requires clear guidance.
- **Pricing:** Meta Graph API is free. WhatsApp Business API uses conversation-based pricing.
- **Reputation:** Meta is the industry standard for these channels.

## Design Doc
**Integration with OHC:**
- **Trigger:** Webhooks from Meta arrive at OHC when a new message/comment is received.
- **Action:** OHC normalizes the payload into a standard `Message` format and routes it to the user's unified inbox interface. Outgoing replies use the respective platform APIs.
- **User Interface:** A simple, unified chat interface within OHC. Users can see the origin platform icon next to the message but reply from the same text box.
- **Environment:** Cloud mode receives webhooks directly. Standalone mode requires a Cloud-hosted proxy to relay webhooks to the local instance securely.

## Implementation Prompt
**User-Facing Outcome:** The business owner can connect their Facebook and Instagram accounts with one click. They see all incoming messages and comments in a single "Inbox" screen within OHC. Replying to a message sends it back to the correct platform automatically.
**Acceptance Criteria:**
- OAuth connection flow is simple and plain-language.
- Incoming messages appear in the OHC inbox in near real-time.
- Replies sent from OHC are delivered to the customer on the original platform.
- Supports text and basic image attachments.

## Priority
P1

## Estimated Scope
Large
