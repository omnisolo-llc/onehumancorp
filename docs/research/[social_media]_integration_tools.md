# Social Media Integration Tools

**Title**: Integrate Social Media Direct Messaging Tools (ManyChat, Buffer)

**Problem Statement**:
Small business owners (like Priya the Boutique Owner and Maya the Home Baker) receive a high volume of customer inquiries across multiple platforms (Instagram DMs, Facebook Comments, TikTok). Managing these separately is overwhelming, prone to missed messages, and requires constant monitoring. They need a unified inbox where their "Customer Success - The Ambassador" AI agent can automatically read, draft, and send responses.

**Research Report**:
We evaluated two main approaches/tools for social media management: ManyChat (for conversational automation) and Buffer (for social media publishing and engagement).
- **ManyChat**: Excellent for conversational flows on Instagram, Facebook Messenger, and WhatsApp. It's built for DMs. It has a robust API for webhooks, allowing us to pipe incoming DMs directly into the OHC AI Agent queue.
  - *Ease of Use*: High. The initial OAuth connection is standard. Once connected, the OHC user never needs to log into ManyChat; OHC handles the API behind the scenes.
  - *Pricing*: Free tier available (limited to 1,000 contacts), Pro is ~$15/mo. Suitable for our user base.
  - *Reputation*: Industry standard for SMB conversational commerce.
- **Buffer**: Excellent for scheduling posts (handled by "Marketing & Advertising - The Promoter"). The engagement API (for replying to comments) is solid, but it's less focused on real-time DM flows compared to ManyChat.
  - *Ease of Use*: Very high. Standard OAuth.
  - *Pricing*: Free tier available for up to 3 channels.
- **Recommendation**: Integrate ManyChat as the primary engine for the Unified Inbox and DM automation, and Buffer for post scheduling.

**Design Doc**:
- **Trigger**: User connects their Instagram/Facebook business account via an OHC UI wizard (OAuth flow).
- **Action**:
  - Incoming messages trigger a webhook to OHC's backend.
  - The message is routed to the AI Job Queue for the "Customer Success" department.
  - The AI reads the message, accesses the business's context (e.g., product availability), and drafts a response.
  - OHC sends the response back via the ManyChat/Buffer API.
- **User Experience**: The user sees all conversations in the OHC app's Unified Inbox. They can see AI-drafted replies and approve them, or set the AI to auto-reply for certain topics (like business hours or vegan options).

**Implementation Prompt**:
Create a UI flow where users can connect their social media accounts. Implement a backend webhook receiver that normalizes incoming messages from ManyChat/Buffer into a standard OHC `Message` object. Create a unified inbox view in the Flutter app where users can read messages, see AI drafts, and manually reply. Ensure the OAuth flow securely stores tokens per tenant.

**Priority**: P0
**Estimated Scope**: Large
