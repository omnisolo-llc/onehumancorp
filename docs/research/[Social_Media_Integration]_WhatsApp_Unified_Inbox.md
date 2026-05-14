# Scout: Tool Integration Research Q4

## 1. Social Media Integration
**Title**: Integrate WhatsApp Business API for Unified Inbox
**Problem Statement**: Small business owners (like local bakers or plumbers) receive critical customer orders and questions via WhatsApp. Managing multiple apps is chaotic, leading to missed messages, slow replies, and lost revenue. They need one simple inbox.
**Research Report**:
- **Tool**: WhatsApp Cloud API (Meta)
- **Problem it solves for which persona**: Allows service-based small businesses (plumbers, bakers) to view and respond to customer inquiries from WhatsApp directly inside their OHC dashboard.
- **Ease of Use**: Very easy for the non-technical owner. They just link their Meta account once. After that, messages appear like regular chats in the OHC interface.
- **Pricing**: The first 1000 service conversations per month are free. Afterwards, conversation-based pricing varies by region (e.g., $0.015/message in NA). Highly affordable for SMBs.
- **Key Advantages**: Massive user base globally; high open rates for messages; native rich media support (images of broken pipes or cake designs).
- **Integration Risks**: Meta's review process can be strict. The 24-hour customer service window requires careful handling of outbound replies.
- **Environment**: Works well in Cloud. Standalone mode might require the business owner to register their own Meta App ID or use a proxy service.
**Design Doc**:
- **Trigger**: Customer sends a message to the business's WhatsApp number.
- **Action**: Webhook receives the message, parses text/media, and routes it to the OHC unified inbox as a new chat thread.
- **User Interface**: Business owner sees a "WhatsApp" icon next to messages in their OHC inbox. Replying sends the message back via the API.
**Implementation Prompt**: Implement a webhook endpoint to receive incoming WhatsApp messages via Meta's Cloud API and surface them in the OHC unified inbox. Enable the business owner to reply from the OHC UI, ensuring their message is sent back to the customer's WhatsApp. Handle text and basic image media types.
**Priority**: P0
**Estimated Scope**: Large
