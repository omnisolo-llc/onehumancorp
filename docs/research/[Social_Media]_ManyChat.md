**Title**: Integrate ManyChat for Unified Social Media Inbox
**Problem Statement**: Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook comments, WhatsApp, and TikTok. Missing a message often means losing a sale, but logging into four different apps constantly is overwhelming for a non-technical user.
**Research Report**: ManyChat is a leading conversational marketing tool. It connects easily to Meta's suite (FB, IG, WhatsApp) and provides a visual builder for automations. For non-technical users, its interface is highly intuitive. Pricing starts at a very accessible tier (often free for up to 1,000 contacts, then $15/mo). It is highly reputable and has robust webhook support, which makes it reliable for catching messages in real time.
**Design Doc**:
- **Trigger**: User connects their ManyChat account via OAuth in the OHC dashboard.
- **Action**: Inbound messages from connected social channels are routed into the OHC unified inbox.
- **User Experience**: The business owner sees all social messages in one OHC view, and their replies are routed back through ManyChat to the customer's original platform.
**Implementation Prompt**: Build a unified inbox interface in OHC where users can read and reply to messages from Instagram, Facebook, and WhatsApp. The user must be able to click "Connect Social Media", authorize the integration, and immediately see new messages appear in their OHC dashboard. Replies sent from OHC must reach the customer on the platform they used.
**Priority**: P0
**Estimated Scope**: Large
**Environment**: Works in both Cloud and Standalone modes (assuming webhook exposure).
