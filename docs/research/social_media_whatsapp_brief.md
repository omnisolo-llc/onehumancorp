### 1. WhatsApp

**Title**: WhatsApp Business API Integration for Unified Inbox

**Problem Statement**:
Small business owners (especially in LATAM, India, and emerging markets like Fatima) use WhatsApp as their primary communication channel for customers. Currently, they have to manually switch between their personal/business WhatsApp app on their phone and the OHC platform, leading to missed messages, slow response times, and disorganized customer records. They need a way to see and respond to WhatsApp messages directly within the OHC unified inbox.

**Research Report**:
- **Tool**: WhatsApp Business API (via Meta Cloud API).
- **Ease of Use**: High for the end user. They link their phone number via OAuth-like flow once, and messages appear in the OHC inbox.
- **Pricing**: Meta shifted to per-message pricing. Inbound service conversations (customer-initiated) are free and unlimited since late 2024. Marketing and utility messages (business-initiated) cost a small fee per message depending on the country. Access to the Cloud API itself is free.
- **Reputation**: It is the global standard for messaging in many countries.
- **Compatibility**: Works well in Cloud mode (Meta Cloud API). In Standalone mode, users would need to configure their own Meta Developer App credentials, which requires a technical setup, so it is best suited for the multi-tenant Cloud version where OHC manages the API keys.

**Design Doc**:
- **Trigger**: Customer sends a WhatsApp message to the business's linked phone number.
- **Action**: Meta webhook sends the message payload to OHC. OHC creates or updates a conversation thread in the Unified Inbox.
- **User Interface**: Business owner sees a "WhatsApp" icon next to the message in their OHC inbox. They can type a reply, and OHC sends it back via the WhatsApp API.
- **Integration Flow**: A new "Connect WhatsApp" button in the Settings -> Integrations page triggers the Facebook Embedded Signup flow to link their number.

**Implementation Prompt**:
Implement the WhatsApp Business integration allowing users to connect their WhatsApp Business account via the Meta embedded signup flow. Incoming WhatsApp messages should appear in the existing OHC Unified Inbox, clearly marked as WhatsApp messages. Users should be able to reply directly from the inbox, and the replies should be sent back to the customer's WhatsApp. Handle webhook ingestion and basic message parsing (text, images).

**Priority**: P0
**Estimated Scope**: Large
