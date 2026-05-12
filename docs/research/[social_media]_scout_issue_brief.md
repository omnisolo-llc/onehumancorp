# Social Media Integration Research Brief

## Title
Unified Social Media Inbox for Small Business Owners

## Problem Statement
Small business owners, especially those running boutique shops or service-based businesses, receive customer inquiries across multiple platforms: Instagram DMs, Facebook Messenger, WhatsApp, and TikTok comments. Managing these separate channels is chaotic, leading to missed messages, slow response times, and lost sales. They need a single, unified view to read and respond to all customer interactions without switching between apps.

## Research Report
### Market Context
The rise of social commerce means that direct messaging is often the primary channel for customer acquisition and support. Customers expect rapid responses (often within an hour). Tools like ManyChat, Meta Business Suite, and Hootsuite address this but are either too complex, enterprise-focused, or limited to specific ecosystems (e.g., Meta only).

### Tool Evaluations

#### 1. Meta Business Suite
- **Ease of Use:** High for users already embedded in the Meta ecosystem (Facebook + Instagram).
- **Pricing:** Free.
- **Capabilities:** Unified inbox for FB and IG. Does not support WhatsApp natively without API, and lacks TikTok integration.
- **Reputation:** Standard tool, but often buggy and disliked for its cluttered interface.

#### 2. ManyChat
- **Ease of Use:** Moderate. Powerful automation but steep learning curve for non-technical users.
- **Pricing:** Freemium. Pro starts at $15/month (scales with contacts).
- **Capabilities:** Excellent Instagram, Messenger, and WhatsApp automation.
- **Reputation:** Industry leader in chat marketing.

#### 3. Respond.io
- **Ease of Use:** High, designed specifically as a unified inbox.
- **Pricing:** Starts around $79/month, which is expensive for micro-businesses.
- **Capabilities:** Excellent omnichannel support (WhatsApp, IG, FB, Telegram, Viber, Webchat).
- **Reputation:** Reliable, but priced for medium-sized businesses rather than solopreneurs.

### Recommended Direction
Integrate directly with WhatsApp Business API and Meta Graph API to provide a simplified, stripped-down unified inbox within OHC. Avoid third-party aggregators to keep costs low for the business owner.

## Design Doc
### Trigger & Action
1. **Trigger:** A customer sends a message on Instagram, Facebook, or WhatsApp.
2. **Action:** OHC receives a webhook from the respective platform. The message is normalized and stored in the OHC unified communications database.
3. **User View:** The business owner sees a "Messages" tab in OHC. New messages appear in a single feed. They can reply directly from OHC, and the response is routed back to the correct platform via API.

### Environment Support
- **Cloud Mode:** Handles webhooks centrally and routes to the correct tenant.
- **Standalone Mode:** Requires local tunneling (e.g., ngrok) or polling mechanisms if webhooks cannot reach the local network. Alternatively, acts as an OAuth client directly connecting to the APIs from the local machine.

## Implementation Prompt
Create a "Unified Inbox" feature that allows business owners to connect their Meta accounts (Facebook Page, Instagram Business) and WhatsApp Business.
- The user should be able to click "Connect Facebook" and go through an OAuth flow.
- Once connected, all incoming messages from these channels must appear in a single chronologically ordered list.
- The user must be able to type a reply and hit "Send," which routes the message back to the customer on the original platform.
- The UI should clearly indicate the source platform (e.g., a small Instagram icon next to the message).
- Acceptance criteria include successfully receiving an IG DM and replying to it entirely within OHC.

## Priority
P1 (High)

## Estimated Scope
Large

### Extended Social Media Analysis
#### Security & Privacy
Handling customer messages requires strict adherence to privacy regulations (GDPR, CCPA). Small business owners rarely understand these requirements, so the integration must handle data retention and deletion requests transparently. Messages should be encrypted at rest.

#### Reliability & Rate Limiting
Meta's APIs are notoriously strict with rate limits. A sudden influx of comments (e.g., a viral post) could trigger rate limits. The integration must implement robust queueing and backoff strategies. It should also alert the business owner if messages are delayed due to API limits.

#### Media Support
Customers frequently send images (e.g., "Do you have this in stock?"). The unified inbox must support image and video attachments, parsing them correctly from the source platform and displaying them securely in the OHC UI.

#### Future Extensibility
While starting with Meta and WhatsApp, the architecture should be channel-agnostic. Adding TikTok or Google Business Messages later should not require a fundamental rewrite of the inbox UI or database schema.

### User Persona Match
- **Fatima (Boutique Owner):** High value. She relies on IG DMs for custom orders.
- **Carlos (Consultant):** Medium value. He mostly uses email but occasionally gets LinkedIn or Twitter DMs.

### Competitive Benchmarking
Compared to tools like Zendesk or Intercom, our unified inbox should focus strictly on *conversations* rather than *tickets*. Small businesses don't want "Ticket #1234 closed"; they want "Replied to Sarah on IG."

### Conclusion
A unified inbox is a critical differentiator for OHC. By removing the friction of checking 4 different apps, we save the business owner roughly 1-2 hours daily.
