## [Social Media] Issue Brief

**Title**: Scout 🔍: Integrate WhatsApp Business API for Unified Inbox
**Problem Statement**:
Many small business owners, especially outside the US, rely heavily on WhatsApp to communicate with customers. Managing these messages manually is tedious and can lead to missed opportunities.
**Research Report**:
- **Tool**: WhatsApp Business API
- **Evaluation**: The WhatsApp Business API allows businesses to automate and manage their interactions. By integrating it, OHC's 'Customer Success' agent can handle common inquiries.
- **Ease of Use**: Business owners simply authenticate with their Facebook/WhatsApp credentials.
- **Pricing**: Priced per conversation. Usually free for the first 1000 conversations.
- **Cloud vs. Standalone**: Works well in Cloud mode. In Standalone, the user would need to configure their own Meta app.
**Design Doc**:
- The user links their WhatsApp account in the 'Social Inbox' tab.
- Webhooks receive incoming messages.
- The AI Agent generates replies based on the business context.
- Replies are sent back via the API.
**Implementation Prompt**:
Implement the WhatsApp Business integration. Create a UI for connecting the account. Set up webhooks to receive and route messages to the AI agent, and send responses back.
**Priority**: P1
**Estimated Scope**: Medium
