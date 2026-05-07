# Social Media Integration: Ayrshare

**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting

**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.

**Research Report**:
- Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn). Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).

**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.

**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
- **Priority**: P0
- **Estimated Scope**: Large
- **Acceptance Criteria**:
  - Unified inbox UI allows viewing and replying to messages.
  - Integration with Ayrshare API handles cross-posting functionality.

**Strategy**: Utilize Ayrshare's unified API to handle multiple social media platforms natively within OHC.
