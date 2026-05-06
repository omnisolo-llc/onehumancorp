# [Social Media] ManyChat Integration

**Title**: Integrate ManyChat to unify social media DMs for OHC users

**Problem Statement**: Small business owners like Fatima struggle to manage customer inquiries scattered across Instagram DMs, Facebook comments, WhatsApp messages, and SMS. They lack a single place to respond, risking missed sales and poor customer service. They need a simple, unified inbox without technical complexity.

**Research Report**: ManyChat is a leading chat marketing and automation platform. It directly integrates with Meta platforms (Facebook Messenger, Instagram DMs, WhatsApp) and SMS.
- **Ease of use**: High for non-technical users. It has a visual drag-and-drop builder for basic automation, though we only need its unified inbox and messaging capabilities.
- **Pricing**: Freemium model. The Pro plan starts at $15/month for up to 500 contacts, making it very affordable for small business owners.
- **Reputation**: Widely trusted in the e-commerce and small business space for Meta integration.
- **Cloud/Standalone**: The API requires internet access, so it functions natively in Cloud mode but would require an active internet connection if running in Standalone mode.

**Design Doc**:
- **Trigger**: A business owner connects their Meta accounts via OAuth in the OHC settings (under a "Social Media" section).
- **Action**: ManyChat webhooks forward incoming messages from Instagram, WhatsApp, and Facebook to the OHC backend. OHC displays these in a unified "Inbox" view.
- **User Experience**: The user simply clicks "Connect Instagram/Facebook", authorizes the app, and begins seeing their DMs inside OHC's Inbox screen. They can type replies directly in OHC, which are sent back through ManyChat's API.

**Implementation Prompt**: Create a unified inbox interface that allows business owners to read and reply to messages from Instagram, Facebook, and WhatsApp. The integration should be a simple 1-click connect button for the user. Messages should appear in real-time.

**Priority**: P1
**Estimated Scope**: Medium