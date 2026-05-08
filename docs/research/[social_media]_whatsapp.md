## [Social Media] Issue Brief: WhatsApp Business API Integration

**Title**: Scout 🔍: Integrate WhatsApp Business API for Global Customer Communication
**Problem Statement**:
Many small businesses, especially internationally, communicate with customers primarily via WhatsApp. Managing orders and inquiries manually on a personal or separate business phone is inefficient and silos customer data. Business owners need a unified inbox where WhatsApp messages are seamlessly integrated alongside other channels.
**Research Report**:
- **Tool**: WhatsApp Business API
- **Evaluation**: The official API allows sending and receiving messages programmatically. It's robust and supports rich media.
- **Ease of Use**: Setup requires business verification, which can be a hurdle. Once set up, it's invisible to the user.
- **Pricing**: Conversation-based pricing (user-initiated vs. business-initiated).
- **Cloud vs. Standalone**: Cloud requires a hosted provider (like Twilio or direct Meta API). Standalone might be challenging due to Meta's infrastructure requirements.
**Design Doc**:
- User links their WhatsApp Business account via an onboarding flow.
- Incoming messages hit a webhook and populate the OHC unified inbox.
- AI agents can draft replies to common inquiries (e.g., "What are your hours?").
- Users can manually reply from the OHC interface, which sends back via the API.
**Implementation Prompt**:
Build an integration with the WhatsApp Business API. Create an onboarding UI for users to connect their accounts. Implement webhooks to receive incoming messages and an outgoing service to send replies. Ensure messages are correctly formatted and attributed in the unified inbox.
**Priority**: P1
**Estimated Scope**: Large
