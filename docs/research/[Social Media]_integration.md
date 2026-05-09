# Social Media Integration: Unified Inbox

**Problem Statement:**
Small business owners lose leads because customer inquiries are scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Managing multiple apps is overwhelming, and late replies mean lost sales.

**Research Report:**
* **Tool Evaluated:** Meta Graph API (Instagram, FB, WhatsApp)
* **Ease of Use:** High for the end-user (unified view), but the initial OAuth connection process requires clear, step-by-step guidance in the UI.
* **Pricing:** Free for basic Graph API messaging; WhatsApp Business charges per conversation.
* **Reputation:** Industry standard for social commerce.
* **Hybrid Context:** Cloud mode can use standard webhooks. Standalone mode may require polling or a cloud-hosted relayer to receive incoming messages.

**Design Doc:**
* **Trigger:** A customer sends a DM on Instagram or Facebook.
* **Action:** The message is routed to the OHC Unified Inbox.
* **User Experience:** The business owner sees a notification in OHC. They open the inbox, see the message context (and which platform it came from), type a reply, and hit send. OHC routes the reply back to the native platform.

**Implementation Prompt:**
Implement a Unified Inbox view that aggregates messages from connected Meta platforms. The user should be able to authenticate their social accounts via a simple "Connect Facebook/Instagram" button. They must be able to read incoming messages and send replies directly from the OHC dashboard without needing to know which API is routing the message.

**Priority:** P1
**Estimated Scope:** Large
