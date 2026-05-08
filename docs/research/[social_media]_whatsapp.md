# Scout: Tool Integration Research [Q2]

## [Social Media] Issue Brief: WhatsApp Business Integration

**Title**: Integrate WhatsApp Business Platform for Automated Customer Support

**Problem Statement**:
Small business owners like Fatima (Food Cart) and Maya (Home Baker) receive dozens of WhatsApp messages daily asking for prices, availability, and order status. Manually answering these repetitive questions takes hours and interrupts their work. They need an AI-powered way to handle these inquiries natively within OHC, so they never miss a sale while they are busy cooking or baking.

**Research Report**:
- **Tool**: WhatsApp Business Platform (API).
- **Evaluation**: This is the industry standard for programmatic WhatsApp access. It allows OHC's "Customer Success" agent to read and reply to messages directly.
- **Ease of Use**: High for the owner once connected. Requires a Facebook Business Verification, which OHC can guide the user through.
- **Pricing**: Conversation-based pricing. The first 1,000 service conversations per month are free.
- **Reputation**: Best-in-class reliability and the preferred communication channel in LATAM, India, and parts of Europe.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode. Standalone mode would require the user to provide their own Meta App credentials.

**Design Doc**:
- The user clicks "Connect WhatsApp" in the Operations dashboard.
- OHC guides them through the Meta Embedded Signup flow.
- OHC registers a webhook to receive incoming messages.
- When a message arrives, the "Ambassador" AI agent drafts a reply based on the business's product catalog and FAQs.
- The owner sees the message in their "Unified Inbox" and can choose to auto-reply or approve drafts.

**Implementation Prompt**:
Implement the WhatsApp Business Platform integration. Set up the OAuth/Embedded Signup flow and a secure webhook endpoint to ingest messages. Route messages to the AI agent for response generation and provide a UI in the unified inbox for the owner to manage these conversations.
- **Acceptance Criteria**: User can connect a WhatsApp Business number. Incoming messages appear in the OHC inbox. AI drafts or sends replies based on settings.
- **Priority**: P1
- **Estimated Scope**: Medium
