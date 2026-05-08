# Title: Unified Social Media Inbox via Meta API Integration

## Problem Statement
Small business owners often juggle messages across Instagram DMs, Facebook Messenger, and WhatsApp. Missing a message means losing a sale or frustrating a customer. They need a single, simple inbox where all customer communications land automatically, without needing to switch between apps or manage complex credentials.

## Research Report
- **Tool Evaluated**: Meta Graph API / WhatsApp Business API
- **Benefit to Users**: Consolidates all Meta-owned communication channels into the OHC unified inbox. Reduces missed messages and response time.
- **Ease of Use**: Once connected via a simple OAuth flow ("Log in with Facebook"), the experience is seamless. Users read and reply from OHC just like standard text messages.
- **Pricing**: Free for standard IG/FB messaging. WhatsApp Business has conversation-based pricing, but the first 1,000 service conversations per month are free, which covers most small businesses.
- **Integration Risks**: Meta's review process for API access can be stringent. Webhook delivery can sometimes be delayed during major outages.
- **Environment**: Works seamlessly in Cloud mode. For Standalone mode, webhooks require a tunneling mechanism or polling fallback, which adds slight complexity but is fully viable.

## Design Doc
- **Trigger**: User navigates to the "Integrations" page and clicks "Connect Social Media".
- **Action**: User is redirected to Meta's OAuth consent screen. Upon approval, OHC registers webhooks for DMs and comments.
- **User Interface**: Incoming messages appear in the standard OHC "Inbox" view, tagged with an icon (IG/FB/WA). Replies sent from OHC are routed back to the correct channel automatically.

## Implementation Prompt
Implement a Meta social media integration that allows users to authenticate their Facebook/Instagram accounts via OAuth. Once connected, incoming DMs and comments should appear in the OHC unified inbox, and user replies from the inbox should be successfully delivered to the customer on the original platform. Ensure the connection flow is self-serve and clearly indicates connection status.

## Priority
P0

## Estimated Scope
Large