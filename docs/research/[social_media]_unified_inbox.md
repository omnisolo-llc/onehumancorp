## [Social Media] Issue Brief: Unified Inbox for DMs

**Title**: Scout 🔍: Unified Inbox for Instagram, WhatsApp, and TikTok DMs
**Problem Statement**: Small business owners are struggling to keep track of customer inquiries scattered across Instagram DMs, Facebook comments, WhatsApp messages, and TikTok comments. They need a single, unified inbox within OHC to manage all communications, preventing lost sales and missed messages.
**Research Report**:
- **Tools Evaluated**: Native Meta Graph API (Instagram/Messenger/WhatsApp) and Ayrshare.
- **Evaluation**: Building direct integrations via the Meta Graph API offers the most control, but managing webhooks for multiple platforms (including TikTok) is complex. Tools like Ayrshare provide a unified API. Given our goal for simplicity, direct Meta integration is best for Instagram/WhatsApp, but we should explore a unified aggregator if we add TikTok.
- **Ease of Use**: The business owner simply clicks "Connect [Platform]" and authorizes via OAuth. They then see all messages in one OHC "Inbox".
- **Pricing**: Meta Graph API is mostly free, with WhatsApp having per-conversation pricing.
- **Cloud vs. Standalone**: Works seamlessly in Cloud mode using our centralized webhooks. Standalone is harder because users would need their own developer accounts to receive webhooks; we might need a cloud proxy for standalone users.
**Design Doc**:
- User visits the "Inbox" or "Social Media" section and connects accounts via OAuth buttons.
- OHC registers global webhooks (in Cloud mode) to receive incoming messages.
- Incoming messages are normalized into a unified schema and displayed in the OHC Inbox.
- The business owner (or the AI Customer Success Agent) can reply directly from OHC, and the message is routed back through the appropriate platform API.
**Implementation Prompt**: Create a unified Inbox feature. Implement OAuth flows for Instagram, Messenger, WhatsApp, and TikTok. Set up webhooks to receive incoming messages, normalize them, and store them in the tenant's database. Provide a UI for the business owner to read and reply to these messages, routing outbound replies back to the correct social platform.
**Priority**: P0
**Estimated Scope**: Large
