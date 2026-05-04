### Title
Integrate WhatsApp via Twilio for a Unified Customer Inbox

### Problem Statement
Small business owners often communicate with customers across multiple platforms: SMS, email, and WhatsApp. For businesses catering to international customers or operating in regions where WhatsApp is the primary communication method, managing orders and answering questions via a personal WhatsApp app is chaotic and mixes personal with business. They need these messages routed to a central business inbox.

### Research Report
**Tool Evaluated:** WhatsApp Business API (via Twilio)
**Overview:** While researching SMS solutions, it was determined that Twilio's Programmable Messaging API also natively supports WhatsApp. Given Twilio's scale ($5 billion revenue, robust API), leveraging their existing WhatsApp integration is the most reliable path.
**Key Features & Advantages:**
- Twilio abstracts the complexity of direct WhatsApp Business API approval and integration.
- Allows sending and receiving WhatsApp messages through the same webhook infrastructure as SMS.
- WhatsApp is globally ubiquitous, crucial for non-technical users in emerging markets.
**Risks:** WhatsApp Business imposes strict rules on outbound messaging (requires pre-approved templates for initiating conversations). The primary use case should focus on inbound customer support and replying within the 24-hour customer service window.
**Ease of Use:** For the business owner, it simplifies life by bringing messages into the OHC inbox. For the customer, they just message a WhatsApp number.
**Pricing:** Per-message pricing via Twilio, with WhatsApp's own conversation-based pricing layered on.
**Deployment:** Cloud-native API integration.

### Design Doc
**Integration Trigger:** A business owner connects their phone number to WhatsApp Business within the OHC "Customer Success" settings.
**Action:** OHC configures the Twilio WhatsApp sandbox/API to route incoming messages for that number to the OHC tenant's unified inbox.
**User Experience:**
- **Business Owner:** Sees a WhatsApp icon next to incoming messages in their OHC inbox. They reply directly from the OHC app, and the response is sent back via WhatsApp. The AI "Ambassador" agent can draft replies for them.
- **Customer:** Clicks a "Chat on WhatsApp" button on the merchant's OHC website and sends a message just like chatting with a friend.

### Implementation Prompt
Implement inbound and outbound WhatsApp messaging capabilities using the Twilio Programmable Messaging API to feed the OHC Unified Inbox.

**Acceptance Criteria:**
1. Extend the previously requested Twilio client to support the `whatsapp:` protocol prefix.
2. Implement webhook handlers to receive inbound WhatsApp messages and map them to the correct tenant's inbox based on the destination number.
3. Ensure the UI distinctly labels messages originating from WhatsApp.
4. Enforce WhatsApp's 24-hour reply window constraint in the UI (disable the reply box if the window has expired and no approved template is used).

### Priority
P1

### Estimated Scope
Medium
