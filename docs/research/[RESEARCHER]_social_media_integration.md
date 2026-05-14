# Social Media Integration - Unified Inbox

**Problem Statement**: Small business owners (like Fatima) have their time split across multiple platforms (Instagram, Facebook, WhatsApp, TikTok). They constantly check different apps to respond to customer inquiries, leading to delayed responses and lost sales. They need a single place to see and reply to all messages.

**Research Report**:
*   **Instagram DMs & Facebook Messenger**: Meta provides Graph API/Messenger API. It requires OAuth and business account setup. Reliable but setup can be tricky for non-technical users.
*   **WhatsApp Business**: Meta offers Cloud API. Good for automated replies and customer support, but requires a WhatsApp Business account and explicit opt-in for marketing messages.
*   **TikTok**: TikTok for Business API provides access to comments and messages.
*   **Competitors**: Tools like Hootsuite, Sprout Social, and Buffer offer unified inboxes but are often too expensive and complex for a very small business.
*   **Pricing**: Many platforms charge based on message volume or number of connected channels.
*   **Cloud vs Standalone**: Cloud is easier for OAuth flows. Standalone would require the user to set up their own developer accounts/API keys for each platform, which is too complex.

**Design Doc**:
*   **Trigger**: User connects their social media accounts via an OAuth flow in OHC settings.
*   **Action**: OHC sets up webhooks with the respective platforms to receive new messages/comments in real-time.
*   **User Interface**: A "Unified Inbox" tab in OHC where messages from all platforms are listed chronologically. The user can click a message and type a reply, which OHC sends back via the platform's API.

**Implementation Prompt**:
Create a unified inbox interface that allows users to connect their Meta (Facebook/Instagram) and WhatsApp accounts. The inbox should display incoming messages from these platforms and allow the user to reply directly from OHC. Ensure the connection process is as simple as possible (e.g., standard OAuth popup).

**Priority**: P1
**Estimated Scope**: Large
