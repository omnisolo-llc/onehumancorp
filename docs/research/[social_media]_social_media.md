# [Social Media Integration] Social Media Unified Inbox via MessageBird

**Title**: Implement Omnichannel Unified Inbox using MessageBird

**Problem Statement**:
Small business owners like Maya (The Home Baker) and Priya (The Boutique Owner) receive customer inquiries, custom orders, and support questions scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. It is overwhelming to constantly switch between 4-5 apps on their phones, and missed messages directly lead to lost sales. They need a single, simple inbox inside the OneHumanCorp (OHC) app where every message appears, allowing them—or their AI "Ambassador"—to read and reply instantly.

**Research Report**:
I evaluated direct Meta Graph API integration, Twilio Conversations, and MessageBird (now Bird) for consolidating social media messages.
- **MessageBird**: Excellent omnichannel API. Standardizes payloads across WhatsApp, Instagram, Facebook, and TikTok. Handles the complex Meta API changes under the hood. High webhook reliability. Pricing: flexible pay-as-you-go or $50/mo base, which is manageable. Best fit for OHC to abstract complexity.
- **Twilio**: Strong for SMS and WhatsApp, but historically slower to fully support native features of Instagram DMs and TikTok compared to MessageBird.
- **Direct Meta API**: Lowest cost (free API), but extremely high OAuth and compliance complexity. It would require OHC to constantly maintain API version bumps for Facebook and Instagram independently, and it doesn't solve TikTok.
- **Conclusion**: MessageBird is the optimal provider. It simplifies the OAuth flow for the end-user (business owner) and standardizes message parsing for our AI agents. It works flawlessly in a Cloud (multi-tenant) environment, while Standalone mode users could provide their own MessageBird API key or rely on a relayed OHC connection.

**Design Doc**:
- **Integration Point**: Resides within the "Customer Success" (The Ambassador) department.
- **Triggers & Flow**:
  1. The user navigates to "Channels" and clicks "Connect Instagram/WhatsApp".
  2. The user completes an OAuth flow (managed by the provider).
  3. Once linked, incoming messages on those platforms trigger a webhook to OHC.
  4. The "Ambassador" AI agent intercepts the webhook, parses the context (and past memory of the customer), and drafts a reply.
  5. The message appears in the user's OHC Mobile Inbox.
- **User View**: A unified, WhatsApp-style chat interface on their 375px mobile screen. Badges indicate the source (e.g., a small Instagram icon next to the message).

**Implementation Prompt**:
Build a unified inbox interface within the OHC Flutter app and integrate it with the selected Omnichannel messaging provider. The user must be able to securely connect their social media accounts via a simple UI flow. Once connected, incoming messages from Instagram, Facebook, and WhatsApp must populate a single chat view in real-time. The "Ambassador" AI agent should automatically draft suggested responses for unread messages, which the user can approve or edit with one tap. Ensure the design relies on OHC Premium Tokens and works flawlessly on a 375px mobile screen.

**Priority**: P1
**Estimated Scope**: Large
