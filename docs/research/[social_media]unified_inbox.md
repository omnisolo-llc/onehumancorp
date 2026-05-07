# Unified Social Media Inbox Integration

**Problem Statement:**
Small business owners often manage communications across multiple platforms like Instagram DMs, Facebook Comments, WhatsApp, and TikTok. Checking multiple apps constantly is exhausting, leads to missed messages from potential customers, and results in delayed responses that hurt sales. They need one simple, unified place to see and reply to all customer messages without switching apps.

**Research Report:**
- **Evaluated Tools:** Meta Graph API (for Instagram/FB), WhatsApp Business API, TikTok for Business API.
- **Ease of Use:** For the end-user, the experience should be a simple "Connect Facebook" button with standard OAuth consent screens.
- **Pricing:** Meta APIs and TikTok APIs are generally free for standard usage, though WhatsApp Business API charges per conversation.
- **Reputation:** Meta and TikTok are industry standards. Webhook reliability is generally good but requires robust retry logic.
- **Cloud vs Standalone:** Works well in Cloud (webhooks can easily hit our servers). For Standalone, we may need a polling fallback or a relay service to handle inbound webhooks securely without exposing local ports.

**Design Doc:**
- **Trigger:** Business owner connects their social accounts via a simple "Connect Social Media" settings page.
- **Action:** Incoming messages (DMs, comments) trigger webhooks that are ingested by OHC. The system creates a unified notification and adds the message to a central inbox interface.
- **User Interface:** A "Unified Inbox" tab where users can see a consolidated list of conversations, clearly marked with the source platform's icon. Users can type a reply and click send, and OHC routes it back to the correct platform.

**Implementation Prompt:**
Build a unified inbox feature that allows a business owner to connect their Meta (Facebook/Instagram) and WhatsApp accounts. Once connected, incoming messages should appear in a single inbox view in the OHC app. The business owner must be able to read and reply to messages directly from this inbox, with replies successfully delivered back to the customer on their original platform. The flow to connect an account must be a standard, user-friendly OAuth pop-up.

**Priority:** P0
**Estimated Scope:** Large
