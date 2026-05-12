# Title: Unify WhatsApp, IG, and FB Messages into One Inbox
## Problem Statement
Small business owners like Fatima waste hours every day switching between WhatsApp, Instagram DMs, and Facebook Messenger to reply to customers. They lose track of orders, reply late, and miss sales because their messages are scattered.

## Research Report
The Meta Graph API and WhatsApp Cloud API allow connecting all Meta properties to a single inbox. This is critical because Meta owns the majority of SMB communication channels.
- **Ease of Use**: A non-technical user can connect via Facebook Login (OAuth), which is a familiar flow.
- **Pricing**: WhatsApp API has per-conversation pricing after the first 1,000 free tier messages; Graph API for FB/IG is free.
- **Reputation**: It is the official Meta API, so it is reliable but requires business verification.

## Design Doc
- **Trigger**: User clicks "Connect Facebook/Instagram" or "Connect WhatsApp" in the OHC Settings > Integrations tab.
- **Action**: Opens Meta OAuth popup. Upon approval, OHC receives an access token and subscribes to webhooks for incoming messages.
- **User View**: A new "Unified Inbox" screen appears in OHC, showing all incoming messages in a single feed with a small icon indicating the source (IG, FB, WA).

## Implementation Prompt
Implement an OAuth connection flow to Meta Graph API and WhatsApp Cloud API. Create a "Unified Inbox" UI that aggregates incoming messages via webhooks. The user should be able to read and reply to messages from all three platforms directly within the OHC app. Ensure the connection status is clearly visible and errors are handled gracefully (e.g., prompting to re-authenticate).

## Priority
P0

## Estimated Scope
Large

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported via webhooks and OAuth redirects handled by the cloud OHC server.
- **Standalone Mode**: Requires proxying webhooks through a central OHC relay service, or polling if webhooks cannot reach the local machine. OAuth requires an internet connection.
- **Risks**: Webhook delivery failures and token expirations causing silent message drops.
