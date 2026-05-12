**Title**: Social Media Integration via Meta Graph API (Instagram/Facebook/WhatsApp)

**Problem Statement**:
Small business owners (like Priya with her boutique or Fatima running her food cart) receive customer inquiries across multiple platforms: Instagram DMs, Facebook comments, and WhatsApp. Checking these manually on different apps is time-consuming, causes missed orders or leads, and makes it hard to maintain a unified view of customer interactions. They need a single, simple inbox to handle all social communications without dealing with technical setup.

**Research Report**:
The Meta Graph API is the unified gateway for Meta's ecosystem (Facebook, Instagram, and WhatsApp Business).
- **Ease of Use for Non-Technical Users**: The integration flow can be handled via Meta's standard OAuth dialog (Facebook Login for Business). Users just click "Connect Meta", grant permissions for their pages/accounts, and they are done.
- **Features**: Supports reading/replying to Facebook Page comments, Instagram DMs, and WhatsApp messages. Webhooks can be subscribed to push new messages instantly to OHC.
- **Reputation & Reliability**: Meta's API is the industry standard and highly reliable, though it requires businesses to have an Instagram Professional/Business account linked to a Facebook Page.
- **Pricing**: The Graph API itself is free for Facebook/Instagram messaging, but WhatsApp Business API has conversation-based pricing (first 1,000 service conversations per month are free, which covers most small businesses).
- **Cloud vs Standalone**: Works perfectly in Cloud mode via central webhooks. In Standalone mode, users would need to configure their own Meta App ID and webhook endpoints (e.g., via ngrok), which violates the "grandmother test" for simplicity. We may need to route Standalone traffic through a central OHC relay to keep it simple.

**Design Doc**:
- **Trigger**: User navigates to Settings > Integrations and clicks "Connect Meta". An OAuth popup appears for authorization.
- **Action**: Once connected, OHC subscribes to Meta Webhooks for `messages` and `messaging_postbacks`.
- **User View**: A new unified "Inbox" tab appears in OHC. When a customer DMs the business on Instagram, the message appears in this inbox. The business owner replies in OHC, and the response is sent back to the customer's Instagram via the Graph API.
- **Architecture**: OHC backend will need a webhook receiver endpoint that validates Meta's SHA256 signature, parses the payload (differentiating IG, FB, WA), and standardizes it into an internal `Message` entity.

**Implementation Prompt**:
Implement a Meta Graph API integration that allows users to connect their Facebook, Instagram, and WhatsApp Business accounts. Create a unified Inbox UI where business owners can view and reply to incoming messages across all connected channels. The connection process must be a simple 1-click OAuth flow. Ensure incoming messages appear in the OHC inbox within seconds of being sent by the customer.

**Priority**: P1 (high)
**Estimated Scope**: Large
