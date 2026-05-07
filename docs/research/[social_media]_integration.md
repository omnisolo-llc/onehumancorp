# Social Media Inbox Integration

## Title
Unified Social Media Inbox Integration (Meta API)

## Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook comments, and WhatsApp. They lose potential sales and customer trust when messages fall through the cracks or take too long to answer. They need a single, unified inbox where they can see and reply to all their customer conversations without switching between three different apps.

## Research Report
*   **Target Tools:** Meta Graph API (covering Instagram Direct, Facebook Messenger, WhatsApp Business API).
*   **Pros:** Access to the three largest platforms used by small businesses globally. Consolidates messaging seamlessly.
*   **Cons:** Meta's review process and OAuth setup can be complex for developers. Strict 24-hour reply window for WhatsApp/Messenger before template messages are required.
*   **Ease of Use for Non-Technical Users:** High (once connected). The connection process requires logging into Facebook, which is familiar. The daily usage is just reading and replying to messages in one place.
*   **Pricing:** Meta Graph API is free for Messenger/Instagram. WhatsApp Business API charges per conversation (varies by region, roughly $0.01-$0.08/msg after free tier).
*   **Cloud vs. Standalone:**
    *   *Cloud:* Straightforward OAuth and Webhook configuration.
    *   *Standalone:* Complex because Meta requires public HTTPS webhooks. Standalone would require either a cloud-proxy (via OHC Cloud) or local tunneling (like ngrok), which reduces the "pure offline" capability.

## Design Doc
1.  **Connection Flow:** User navigates to Settings > Integrations > "Connect Facebook/Instagram/WhatsApp". They click a "Connect Meta" button and log into their Facebook account, granting permissions.
2.  **Incoming Messages:** When a customer sends a DM on Instagram or WhatsApp, it appears in the OHC unified inbox UI alongside regular emails.
3.  **Outgoing Messages:** When the business owner replies from OHC, the message is routed back to the correct platform (Instagram/WhatsApp) seamlessly. The UI clearly labels the source platform.

## Implementation Prompt
Create a "Social Media Inbox" feature. The user should be able to connect their Meta account (Instagram, Facebook, WhatsApp) via a simple "Connect" button in the Settings page. Once connected, incoming messages from these platforms should appear in a new unified "Inbox" view. The user must be able to read and reply to these messages directly from OHC. Ensure the UI clearly distinguishes which platform a message came from. Include a visual indicator if a message is older than 24 hours (due to Meta's reply window policies).

## Priority
P0 (critical)

## Estimated Scope
Large
