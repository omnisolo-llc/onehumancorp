# [Social Media] Unified Inbox with Chatwoot

**Title**: Implement Chatwoot Unified Inbox for Social Media Channels

**Problem Statement**:
Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook comments, WhatsApp, and TikTok. Constantly switching between apps leads to missed messages, slow response times, and lost sales. They need a single, simple place to read and reply to all customer inquiries without needing to understand APIs or complex routing rules.

**Research Report**:
- **Evaluated Tools**: Chatwoot, Intercom, ManyChat.
- **Findings**: Chatwoot stands out as an excellent option because it provides a truly unified inbox designed for multiple social media channels (WhatsApp, Facebook, Instagram, Twitter, etc.). Intercom is too expensive and enterprise-focused for our target persona. ManyChat is powerful for automation but less focused on a simple unified human inbox.
- **Ease of Use**: Chatwoot's interface is intuitive and resembles standard email/chat clients, making it easy for non-technical users to adopt. Setting up channels typically involves a standard OAuth flow ("Connect Facebook") which is manageable for small business owners.
- **Pricing**: Chatwoot offers a free self-hosted version (which aligns with OHC Standalone mode) and affordable cloud pricing (starting around $19/user/month for cloud), making it highly accessible.
- **Cloud vs Standalone**: Works perfectly in both. In Cloud mode, we can provision Chatwoot accounts per tenant. In Standalone mode, Chatwoot can be packaged as a local container or connect to a centralized Chatwoot instance.

**Design Doc**:
- **Trigger**: The user navigates to a "Connect Channels" settings page in the OHC desktop or web app and clicks "Connect Facebook/Instagram".
- **Action**: The system initiates an OAuth flow with the respective social network. Once authorized, OHC configures the webhook/channel in the underlying Chatwoot engine.
- **User View**: A new "Inbox" tab appears in the OHC UI. When a customer sends an Instagram DM, it appears in this Inbox. The business owner can type a reply directly in OHC, which is routed back to the customer's Instagram.

**Implementation Prompt**:
Build a unified "Inbox" interface in the OHC app that allows users to connect their social media accounts (starting with Facebook and Instagram). The user should be able to authenticate with their social accounts via a simple click-through flow. Once connected, incoming messages from those platforms must appear in the OHC Inbox, and any replies sent from the OHC Inbox must be delivered back to the customer on the original platform. Acceptance criteria include successful message receipt and successful reply delivery without the user ever leaving the OHC application.

**Priority**: P1
**Estimated Scope**: Large
