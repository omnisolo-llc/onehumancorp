# 📱 Social Media: Unified Inbox

## Title
Social Media Unified Inbox Integration

## Problem Statement
Small business owners like Maya (The Home Baker) receive customer inquiries across multiple platforms: Instagram DMs, Facebook Messenger, WhatsApp, and TikTok comments. Constantly switching apps to manage these conversations is overwhelming, leading to missed messages, slow response times, and lost sales. They need a single, unified inbox within the OHC app to view and respond to all social media interactions.

## Research Report
- **Goal**: Evaluate tools that provide a unified API for aggregating messages from major social media platforms.
- **Tools Evaluated**:
    - **Meta Graph API (Instagram/Facebook/WhatsApp)**: Direct integration. High reliability but complex OAuth flows and strict review processes. Free, but developer overhead is significant.
    - **Twilio Conversations**: Robust, handles SMS and WhatsApp well, but lacks native Instagram/TikTok comment support. Expensive for high volumes.
    - **MessageBird (Inbox)**: Good omnichannel support, but more geared towards larger enterprises.
    - **Respond.io / Chatwoot (Open Source)**: Chatwoot provides a great open-source core with omnichannel inbox capabilities. Can be self-hosted (Standalone mode) or consumed via API (Cloud mode).
- **Recommendation**: Integrate with **Chatwoot** (or build a similar facade over Meta APIs). It provides a unified data model for conversations, supports webhooks for real-time updates, and handles the complexities of platform-specific message formatting.
- **User Impact**: Maya can see an Instagram DM asking "do you do vegan cakes?" right next to a WhatsApp message, and reply to both from the OHC mobile app. The AI "Customer Success" agent can draft replies for both seamlessly.

## Design Doc
- **Component**: `SocialInboxAgent`
- **Responsibilities**:
    - Manage OAuth connections to social media platforms via the unified provider.
    - Listen for incoming webhooks for new messages/comments.
    - Standardize message formats and store them in the `conversations` and `messages` tables.
    - Route messages to the AI Customer Success agent for auto-drafting replies.
    - Send outbound replies back to the respective platform.
- **Integration Point**: The OHC Frontend will query the `conversations` API to display the unified inbox.

## Implementation Prompt
Implement the Social Inbox integration. Create a service that connects to the chosen provider (e.g., Meta Graph API or Chatwoot API). Implement endpoints to handle incoming webhooks, normalize the message data, and store it in the database. Ensure the service can send outgoing messages. The UI should display a combined list of conversations from all connected platforms. Support both Cloud (multi-tenant webhooks) and Standalone (local proxying if necessary) environments.

## Priority
P0

## Estimated Scope
Large
