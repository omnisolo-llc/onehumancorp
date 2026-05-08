## [Social Media] Buffer Integration
**Title**: Integrate Buffer Engage for Unified Social Inbox
**Problem Statement**: Small business owners need a single inbox to manage incoming messages and comments from Instagram, Facebook, and TikTok without needing to constantly switch apps.
**Research Report**:
- **Tool**: Buffer (specifically the Engage inbox)
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: User-friendly, well-known brand, supports major platforms.
- **Risks**: Pricing can scale up per channel, which might get expensive for small businesses.
- **Pricing**: Free tier exists, but Inbox features typically start around $6/month per channel.
- **Compatibility**: Cloud (OAuth API). Standalone (API keys proxy).
**Design Doc**:
- User visits the Integrations page and selects "Buffer".
- User authenticates via OAuth.
- Incoming messages from connected Buffer channels are routed to the OHC unified inbox.
- Customer Success agent monitors the inbox and drafts replies.
**Implementation Prompt**: Implement an integration with Buffer's API to fetch incoming social media messages and comments, feeding them into the unified OHC inbox. Send outbound replies back through Buffer.
**Priority**: P1
**Estimated Scope**: Medium
