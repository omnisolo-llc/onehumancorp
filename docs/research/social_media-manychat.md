# Unify Instagram and Facebook Messages with ManyChat

**Problem Statement**
As a small business owner, I spend way too much time hopping between Instagram, Facebook, and WhatsApp to answer customer questions. It's easy to lose track of messages, which means I lose sales. I want one single inbox where I can see and reply to all my customers.

**Research Report**
ManyChat is a leading platform for social media messaging integration. It connects seamlessly with Instagram DMs, Facebook Messenger, and WhatsApp. It is highly reputable and widely used by small businesses. Ease of use for non-technical users is exceptional, offering a visual flow builder. Pricing starts at $15/month for the Pro plan, scaling with contacts. The OAuth flow is robust, and webhooks are reliable for real-time syncing. It works well in both Cloud and Standalone modes as it's a cloud API connection.

**Design Doc**
The business owner will see a 'Connect Social Accounts' button in their settings. Once clicked, they authorize OHC to access their ManyChat account. After connection, incoming messages from connected social platforms will appear in the unified OHC inbox. Replying from the OHC inbox will send the message back to the customer on their original platform.

**Implementation Prompt**
Implement a unified inbox experience where users can connect their ManyChat account. Ensure that incoming messages from Instagram, Facebook, and WhatsApp are displayed in real-time in the main OHC dashboard. The user must be able to reply directly from the dashboard, and the reply should be delivered to the customer via ManyChat. The acceptance criteria include a successful OAuth connection flow and successful end-to-end message sending and receiving.

**Priority:** P1
**Estimated Scope:** Medium
