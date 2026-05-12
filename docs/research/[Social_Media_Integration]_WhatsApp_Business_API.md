# [Social Media Integration] WhatsApp Business API

**Problem Statement**: Small business owners (like retail shop owners or service providers) receive customer inquiries on WhatsApp but struggle to manage them alongside other messages. They need a single place to see and reply to WhatsApp messages so they don't miss customer requests or lose track of conversations, especially when multiple staff members need to respond.

**Research Report**:
- **Target Persona**: Small business owners, retailers, service providers with high WhatsApp usage.
- **Ease of Use**: Direct integration for users might be complex without an interface like OHC. Meta provides the Cloud API which allows sending and receiving messages.
- **Pricing**: WhatsApp charges per conversation (user-initiated vs. business-initiated). OHC could pass these costs or include a tier.
- **Reputation/Reliability**: Meta's official API is reliable but has strict opt-in rules and template approvals for business-initiated messages.
- **Cloud/Standalone**: Works in Cloud via Webhooks. For Standalone, it requires a secure webhook endpoint exposed to the internet (which could be a limitation or require a tunneling solution).

**Design Doc**:
- **Trigger**: Customer sends a WhatsApp message to the business's registered number.
- **Action**: OHC receives the message via Webhook, parses it, and displays it in the unified inbox.
- **User View**: Business owner sees WhatsApp messages in the same inbox as emails and other DMs. They can reply directly from OHC, and the response is sent back via the WhatsApp API.
- **Integration**: Requires OAuth/Meta Business Login to connect the account.

**Implementation Prompt**: Implement a connection flow for business owners to link their WhatsApp Business account. Once linked, incoming WhatsApp messages should appear in the unified inbox, and replies from the inbox should be delivered back to the customer on WhatsApp.

**Priority**: P1
**Estimated Scope**: Large
