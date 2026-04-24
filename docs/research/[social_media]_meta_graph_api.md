# Meta Graph API Integration

**Title**: Implement Unified Social Media Inbox via Meta Graph API
**Problem Statement**: Small business owners (like Maya the baker) manage customer inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Context switching between apps leads to missed messages, delayed responses, and lost sales. They need a single, unified inbox within OHC, managed by the "Customer Success" AI agent.
**Research Report**:
- **Tool**: Meta Graph API (specifically Messenger API for Instagram, Messenger API, and WhatsApp Business API).
- **Ease of Use (End User)**: Seamless. After an initial OAuth connection, messages flow into the OHC inbox natively.
- **Pricing**: Instagram/Messenger API is generally free for standard usage. WhatsApp Business API charges per conversation (user-initiated vs. business-initiated), which needs to be factored into OHC pricing tiers.
- **Cloud vs. Standalone**: Works in Cloud (webhooks to OHC servers). For Standalone, requires a cloud proxy to receive webhooks and forward them to the local instance, or polling (less ideal).
**Design Doc**:
- **Trigger**: User connects their Facebook/Instagram/WhatsApp Business accounts via an "Integrations" UI.
- **Action**: Webhooks are established. Incoming messages trigger the Customer Success AI agent to draft responses or alert the user. The user can reply directly from the OHC interface.
- **UI**: A unified "Inbox" view aggregating messages from all three channels, visually tagged by source.
**Implementation Prompt**: Implement an integration with the Meta Graph API that allows users to connect their Instagram Business, Facebook Page, and WhatsApp Business accounts. Ensure incoming messages from these channels appear in a unified OHC inbox. The system must support sending replies back to the respective platforms.
**Priority**: P0
**Estimated Scope**: Large
