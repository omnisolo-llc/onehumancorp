# [Social Media] Unified Inbox Sync

## Problem Statement
Small business owners like Maya (the home baker) get inquiries via Instagram DMs, WhatsApp, and Facebook comments. Keeping track of all these messages across multiple apps is overwhelming, leading to missed orders and slow response times. They need a single, simple inbox inside the OHC app where all customer messages appear in one place, allowing their AI "Ambassador" to help draft replies.

## Research Report
- **Target Tools**: Meta Graph API (Instagram Messaging, Messenger, WhatsApp Business API).
- **Competitive Analysis**: Tools like ManyChat and Shopify Inbox offer similar integrations, but they often require complex initial setups or separate apps.
- **Ease of Use**: By utilizing Meta's official APIs, we can create a streamlined OAuth flow. The business owner just clicks "Connect Instagram" and logs in. No technical setup is required.
- **Pricing**: Meta Graph APIs are generally free for standard messaging. WhatsApp Business has volume-based pricing, but a free tier for initial conversations exists which fits our target personas.
- **Reputation**: Meta APIs are the industry standard for these integrations, despite occasional strict approval processes for API access.
- **Advantages and Risks**: Advantage is native reach on the platforms users already use; risk is Meta's strict API approval and account suspension policies.
- **Cloud vs Standalone**: Works in Cloud mode (central webhooks). Standalone mode would require the user to configure their own Meta App or routing incoming events through the OHC Cloud proxy (which introduces complexity).

## Design Doc
- **Integration Flow**: The user accesses the "Customer Success" department in the OHC app and clicks to connect their social accounts via a standard Meta OAuth popup.
- **Actions**: Once connected, incoming DMs and comments are fetched and displayed in a unified OHC inbox. The AI Ambassador can read these messages to suggest draft replies. When the user approves a reply, it is sent back through the Meta API to the respective platform.
- **User Experience**: A seamless, mobile-optimized chat interface inside OHC where messages show a small icon indicating their source (e.g., an Instagram logo).

## Implementation Prompt
Create a unified inbox feature that allows users to authenticate with Meta and connect their Instagram, Facebook, and WhatsApp accounts. Incoming messages from these platforms should populate a single chat interface within the OHC app. The feature must include the ability to read messages, see which platform they came from, and reply directly from the OHC app, with replies routing back to the correct original platform. Ensure the authentication flow is simple enough for a non-technical user on a mobile device.

## Priority
P0

## Estimated Scope
Large
