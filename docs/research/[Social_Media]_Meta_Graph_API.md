# [Social Media Integration] Meta Graph API

**Title**: Connect Instagram, Facebook, and WhatsApp to OHC Unified Inbox via Meta Graph API

**Problem Statement**:
Business owners like Maya the Home Baker receive inquiries and custom order requests across Instagram DMs, Facebook comments, and WhatsApp messages. Checking multiple apps constantly is overwhelming and leads to missed sales. They need all customer messages routed into a single, unified inbox where their "Customer Success" AI agent can draft replies automatically.

**Research Report**:
The Meta Graph API is the official way to integrate with Facebook, Instagram, and WhatsApp.
- **Ease of Use for Non-Technical Users**: Meta's OAuth flow allows business owners to connect their pages with a few clicks. However, the initial setup can sometimes be confusing due to Meta's strict business account requirements. OHC will need to provide clear, plain-language guidance.
- **Pricing**: Connecting pages and reading/replying to messages via the standard Graph API is generally free, though WhatsApp Business API has per-conversation pricing that needs to be abstracted or passed through transparently.
- **Reputation**: It is the industry standard (and only official way) to access Meta platforms, though its documentation and approval processes can be complex.

**Design Doc**:
- **Trigger**: The user clicks "Connect Social Accounts" in the OHC dashboard and completes the Meta OAuth flow.
- **Action**: OHC subscribes to webhooks for new messages and comments. Incoming messages are routed to the OHC unified inbox. The AI "Ambassador" drafts suggested replies.
- **User View**: The business owner sees a single feed of messages in their OHC app, regardless of whether the message came from Instagram, Facebook, or WhatsApp, and can approve or edit AI-suggested replies with one tap.

**Implementation Prompt**:
Implement Meta Graph API integration to pull Instagram DMs, Facebook comments, and WhatsApp messages into the OHC unified inbox. Setup the OAuth connection flow so it's simple for a non-technical user. Ensure reliable webhook handling to receive real-time messages. The integration must allow the user to reply directly from the OHC app, sending the message back out through the respective platform. This integration should be focused on Cloud mode due to OAuth callback and webhook requirements.

**Priority**: P0
**Estimated Scope**: Large
