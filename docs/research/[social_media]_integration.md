**Title**: Social Media Unified Inbox (WhatsApp & TikTok)
**Problem Statement**: Small business owners like Priya (boutique) or Fatima (food cart) receive customer inquiries across multiple platforms, primarily WhatsApp and TikTok. Checking multiple apps constantly is overwhelming, leading to missed orders, slow responses, and lost revenue. They need a single, simple inbox to see and reply to all messages without juggling apps.
**Research Report**:
- **WhatsApp**: Owned by Meta, it has over 3 billion monthly active users. It's the primary communication channel in many emerging markets. The WhatsApp Business API allows integration, but direct setup is complex for non-technical users.
- **TikTok**: A massive platform for short-form video where trends drive fast sales. Comments and direct messages are crucial for engagement, but the native app is optimized for viewing, not customer service.
- **Ease of Use**: A unified inbox eliminates the need to switch contexts. This is critical for users with limited time or technical proficiency.
- **Pricing**: WhatsApp Business API charges per conversation. TikTok APIs are generally free but require approval. We need to abstract these costs or pass them through transparently.
- **Reputation**: Both are essential, high-trust channels for consumer engagement.
- **Cloud/Standalone**: The unified inbox concept works perfectly in a Cloud environment (webhooks to our servers) and can function in Standalone mode with local network proxies or direct device integrations (though more complex).
**Design Doc**:
- **Trigger**: A customer sends a message on WhatsApp or comments on a TikTok video.
- **Action**: Webhooks receive the payload, standardize the format, and insert it into a unified `messages` table linked to the tenant.
- **UI**: A single 'Inbox' view in the OHC dashboard showing a chronologically sorted list of messages, with icons indicating the source (WhatsApp/TikTok). Users can click a message and reply directly from the OHC interface, which then uses the respective API to send the response back to the platform.
**Implementation Prompt**: Build a unified inbox interface that aggregates messages from WhatsApp and TikTok. The user should be able to authenticate their social accounts with a 1-click OAuth flow (or similar simple setup). Incoming messages should appear in a single feed. The user must be able to reply to any message from this feed, and the response must be delivered to the correct platform natively.
**Priority**: P1
**Estimated Scope**: Large
