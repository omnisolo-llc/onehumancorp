# ManyChat Unified Social Inbox Integration

## Problem Statement
Small business owners receive messages across multiple platforms (Instagram, Facebook, WhatsApp, TikTok) but struggle to keep track of them all. They miss important inquiries, which leads to lost sales. Managing different apps is overwhelming, and they need a single, simple inbox to see and reply to all customer messages without needing to understand technical details.

## Research Report
ManyChat is a leading platform for social media messaging integration and automation.
- **Ease of Use**: It provides a highly intuitive, visually simple interface that is accessible for non-technical users.
- **Capabilities**: Connects seamlessly with Facebook, Instagram, WhatsApp, and Telegram.
- **Competitors**: Ayrshare, Meta Business Suite. ManyChat is more robust for multi-platform unification compared to using Meta's native tools alone, especially when expanding to other channels.
- **Reputation**: Highly rated by small businesses for reliability and ease of setup.
- **Pricing**: Has a free tier (basic features up to 1,000 contacts) which is great for small businesses starting out, and a Pro tier starting around $15/month based on contacts.
- **Deployment**: The integration works well via cloud webhooks, and its API can be connected securely, supporting both Cloud and Standalone models (with appropriate polling or local tunneling if necessary).

## Design Doc
The integration will connect the OHC unified inbox directly to ManyChat's API. When a business owner links their social accounts through OHC, OHC will authorize with ManyChat. Incoming messages from any connected platform will trigger an event sent to OHC's backend, which will then display the message in a single unified conversation view on the business owner's dashboard. Replies typed in OHC will be pushed back through ManyChat to the original platform (e.g., an Instagram DM).

## Implementation Prompt
Create a "Social Inbox" tab in the dashboard. Allow users to click a "Connect Social Media" button that walks them through a simple ManyChat authorization flow. Once connected, all new messages from Instagram, Facebook, and WhatsApp should appear in this inbox. When the user types a reply and hits send, the message should be delivered back to the customer on their respective platform. Ensure error messages are friendly (e.g., "We couldn't send your message, please try again") and no technical API jargon is shown.

## Priority
P1

## Estimated Scope
Medium
