## [Social Media Integration] Unified Social Inbox
**Title**: Integrate ManyChat / Chatwoot for Unified Social Media Inbox

**Problem Statement**: Small business owners are overwhelmed by managing customer messages across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Constantly switching apps leads to missed messages, slow response times, and lost sales. They need a single place to see and reply to all customer inquiries.

**Research Report**:
- **Persona Context**: Small retailers and service providers who rely heavily on social media for lead generation but lack dedicated customer support staff.
- **Solution Evaluated**: ManyChat and Chatwoot. Chatwoot provides a unified inbox that aggregates conversations from major platforms and has strong WhatsApp/Facebook integration. ManyChat is better for automated flows but more complex.
- **Ease of Use**: Chatwoot is very straightforward for a non-technical user. It looks like a standard email or chat inbox.
- **Advantages**: Solves the fragmentation problem perfectly. Can be self-hosted (Standalone) or consumed as SaaS (Cloud).
- **Risks**: Relying on Meta's official API limits (WhatsApp requires business accounts). Rate limits and account blocks for small users.
- **Pricing Estimate**: Chatwoot ranges from free (self-hosted or basic cloud) to $19/user/month.
- **Cloud/Standalone Support**: Works in both Cloud (via SaaS APIs) and Standalone (can self-host or integrate via API tokens locally).

**Design Doc**:
- **Triggers**: Incoming messages on connected social platforms trigger an event in OHC.
- **Actions**: OHC displays the incoming message in a unified inbox tab within the Slint UI.
- **User Interface**: The user sees a "Social Messages" section in OHC where they can link their social accounts via a simple OAuth flow (or API key for Standalone). Once linked, messages appear in a chat-like interface. Replying in OHC sends the message back to the native platform.

**Implementation Prompt**:
Build a unified inbox interface in OHC where users can read and reply to messages from Instagram, Facebook, and WhatsApp. The user must be able to authenticate their social accounts easily from the settings page. Ensure replies sent from OHC appear natively on the customer's social media app.

**Priority**: P0
**Estimated Scope**: Large
