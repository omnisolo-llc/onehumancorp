# Unified Inbox for Social Media

**Problem Statement**: Small business owners miss customer inquiries because they are scattered across Instagram, Facebook, WhatsApp, and TikTok.

**Research Report**: Meta (IG/FB/WA) has strong APIs but complex OAuth. TikTok is emerging but APIs are less stable for DMs. Competitors like Hootsuite are too pricey. Standalone mode is hard due to API key management; Cloud mode is ideal.

**Design Doc**: OAuth connection flow in settings. Unified inbox UI pulling from webhooks. Replies routed back through respective APIs.

**Implementation Prompt**: Build a unified inbox view where users can connect social accounts and reply to messages from one screen.

**Priority**: P0
**Estimated Scope**: Large
