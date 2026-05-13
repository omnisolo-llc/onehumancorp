# [Social Media] Instagram Direct Messages Integration

## Title
Native Instagram DMs Integration for Unified Inbox

## Problem Statement
Maya the Home Baker receives 80% of her cake orders via Instagram DMs. Constantly switching between the Instagram app and her order tracking spreadsheet causes her to miss messages and lose sales. She needs Instagram messages to flow directly into her OHC workspace.

## Research Report
- **Strategy**: Integration with Instagram Messaging API.
- **Advantages**: Captures high-intent buyers directly from social media. Eliminates app-switching.
- **Risks**: Meta's API has strict rate limits and requires business accounts. Rich media (images/videos) handling can be complex.
- **Pricing**: Generally free for standard usage within rate limits.
- **Ease of Use**: Once OAuth is completed, it's seamless for the user.
- **Compatibility**: Cloud (Webhooks). Standalone (Requires proxy for webhooks).

## Design Doc
- User authenticates with Meta/Instagram via an OAuth flow in the "Integrations" tab.
- OHC registers webhooks to receive new DMs.
- Incoming DMs appear in the unified "Customer Inbox" alongside WhatsApp and SMS.
- Responses sent from OHC are delivered back to the customer's Instagram DM via the API.

## Implementation Prompt
Implement the Instagram Messaging API integration. Create the OAuth connection flow, handle incoming webhooks to route messages to the unified inbox, and support outbound replies.

## Priority
P0

## Estimated Scope
Large
