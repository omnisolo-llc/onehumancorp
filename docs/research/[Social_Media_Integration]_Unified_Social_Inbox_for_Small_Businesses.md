# Issue Brief: Unified Social Inbox for Small Businesses

**Category**: Social Media Integration

## Problem Statement
Small business owners struggle to keep up with customer inquiries scattered across Instagram DMs, Facebook comments, WhatsApp, and TikTok. They miss leads and upset customers due to slow response times.

## Research Report

### Tool Evaluations

**1. Meta Graph API (Instagram & Facebook)**
The Meta Graph API is the official way to integrate with Facebook Pages and Instagram Professional accounts.
- **Ease of Use for User**: Users must go through a complex Facebook Login flow, granting specific permissions (pages_messaging, instagram_manage_messages). This is often a point of friction where users get confused about selecting the correct Facebook Page linked to their Instagram account.
- **Pricing**: Free for receiving and responding within the 24-hour standard messaging window.
- **Webhook Reliability**: Meta's webhooks are highly reliable but require the OHC Cloud endpoint to verify tokens and handle strict security requirements (HTTPS, specific response times).
- **Mode Compatibility**: In Cloud mode, we can host the webhook endpoint centrally. In Standalone mode, we cannot route webhooks directly to a user's local machine without a proxy service (like the OHC Hybrid Event Mesh).

**2. WhatsApp Business API (via Meta)**
- **Ease of Use for User**: Requires setting up a WhatsApp Business Account (WABA) and verifying the business, which takes days.
- **Pricing**: Meta charges per conversation (user-initiated vs. business-initiated) after the first 1,000 free tier conversations per month.
- **Webhook Reliability**: Same robust infrastructure as the Graph API.
- **Mode Compatibility**: Same challenges for Standalone mode as Facebook/Instagram.

**3. ManyChat (Third Party)**
- **Ease of Use for User**: Extremely high. ManyChat abstracts the Meta API complexity into a visual builder.
- **Pricing**: Starts at $15/month for basic features.
- **Webhook Reliability**: Very good, but introduces a third-party dependency.
- **Mode Compatibility**: They offer their own integrations, but it would fragment our OHC unified experience. We should integrate directly with Meta instead.

**Summary Recommendation**: Build direct integrations with Meta Graph API to avoid third-party subscription costs for the user.


## Design Doc
Integrate Meta Graph API (Instagram/FB/WhatsApp) and TikTok for Business API to pull messages into a unified inbox in the OHC UI. A background worker periodically syncs messages. Users reply from OHC, and we route the message back via the respective API. Cloud mode uses scalable webhook endpoints; Standalone mode uses secure local polling/webhooks where possible or relies on Cloud proxy.

## Implementation Prompt
Build a unified inbox view that displays messages from Instagram, Facebook, and WhatsApp. It should allow the user to read and reply to messages. Do not worry about the exact database schema or API endpoints; focus on the user experience of authenticating these services and managing the messages.

## Priority
P0

## Estimated Scope
Large
