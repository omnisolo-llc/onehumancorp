## [Social Media] Issue Brief: Automated Direct Message Integration

**Title**: Scout 🔍: Integrate Meta API for Automated Instagram & Messenger DMs
**Problem Statement**:
Small business owners like Maya (Home Baker) and Priya (Boutique) are overwhelmed by repetitive direct messages on Instagram and Facebook (e.g., "Do you do vegan?", "Is this in stock?"). Replying manually takes away from their actual work, and missing DMs means losing sales. They need an automated way to handle these inquiries without touching any code or configuring complex webhook flows.
**Research Report**:
- **Tool**: Meta Graph API (Instagram Direct & Messenger) or a managed wrapper like ManyChat.
- **Evaluation**: The Meta API allows full programmatic access to read and reply to DMs. By integrating this, OHC's "Customer Success" AI agent can draft and send replies based on the business's existing catalog, FAQs, and business hours.
- **Ease of Use**: Very easy for the user. They simply click "Log in with Facebook/Instagram" to grant permissions. No API keys to manage.
- **Pricing**: Free to use the Meta API, though WhatsApp integration has per-conversation pricing.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode (OHC manages the Meta App and Webhooks). In Standalone mode, it would be complex as the user would need to create their own Meta App.
**Design Doc**:
- The user navigates to a "Social Inbox" tab and clicks "Connect Instagram".
- Uses OAuth to grant OHC permission to read/write messages.
- OHC registers a centralized webhook for the tenant.
- Incoming messages are routed to the AI Agent (Customer Success).
- The agent formulates a response based on the tenant's context (products, availability) and sends it back via the Meta API.
**Implementation Prompt**:
Implement the Instagram/Messenger integration. Provide a UI for the user to connect their Meta account. Set up a secure webhook endpoint to receive incoming DMs, route them to the LLM with the user's business context, and send the generated reply back to the customer. Ensure the user can toggle the AI on/off or set it to "draft only" mode.
**Priority**: P1
**Estimated Scope**: Medium
