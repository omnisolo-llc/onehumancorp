# Title: Implement Unified Inbox for Social Media Channels

## Problem Statement
Small business owners often miss customer inquiries or sales opportunities because messages are scattered across multiple platforms—Instagram DMs, Facebook comments, WhatsApp chats, and TikTok comments. Managing these separate apps is overwhelming, time-consuming, and prone to error, especially when the owner is busy running the actual business. They need a single, easy-to-use inbox where all customer interactions flow seamlessly, so they can reply from one place without constantly switching apps.

## Research Report
We evaluated multiple Unified Inbox providers to determine the best integration strategy for OHC:
- **Twilio (Conversations API):** Extensive support for WhatsApp and Facebook Messenger. Highly reliable but requires significant technical configuration for channel linking. Pricing is based on active user interactions (e.g., $0.05 per active user/month + message costs). Good reputation, but complex for a non-technical user to configure directly.
- **MessageBird (Inbox / Omnichannel Chat Widget):** Good coverage of Instagram, Facebook, and WhatsApp. Offers a more complete "inbox" experience out of the box. Easy-to-use APIs. Pricing is slightly higher but abstracts a lot of the OAuth and Webhook complexity.
- **Direct Meta Platform APIs (Instagram/Facebook/WhatsApp):** Bypasses middleware providers. Completely free (except WhatsApp business conversation fees). However, OAuth complexity is extremely high for the end user, requiring Facebook Business Manager setup, which fails the "Grandmother Test."
- **Cloud vs. Standalone Compatibility:** Both Twilio and MessageBird require webhooks to deliver messages in real-time. In a **Cloud (multi-tenant)** environment, webhooks map easily to tenant IDs. In a **Standalone (local, private)** environment without public IPs, a local proxy or polling fallback mechanism is necessary, which limits out-of-the-box reliability without additional infrastructure like Ngrok or OHC relay services. Direct Meta APIs also have strict webhook requirements that would need relaying for Standalone mode.

**Recommendation:** A hybrid approach using an abstraction layer (like Twilio or a simplified Meta proxy) that OHC manages. We must abstract the OAuth flow into a simple "Connect Instagram" button.

## Design Doc
When the business owner navigates to their "App Settings" in OHC, they will see a "Connect Instagram/WhatsApp" button. Clicking this initiates a simplified OAuth flow. Once connected, incoming messages from these platforms trigger an event in the OHC system, which updates the Unified Inbox in real-time. The user sees a standard chat interface with an icon indicating the source platform. When the owner replies in OHC, the message is routed back to the customer on their native platform (e.g., Instagram DM). Standalone mode will require a relay service to securely forward incoming webhooks to the local instance.

## Implementation Prompt
Implement a unified inbox experience that aggregates incoming messages from connected social media platforms. The user should be able to connect their Instagram and WhatsApp accounts with a single click from the settings page. Once connected, all new messages must appear in a centralized "Inbox" view inside the OHC application. The user must be able to read and reply to messages directly from this inbox, and the replies must reach the customer on the original platform. Ensure the UI clearly shows which platform the message originated from. The connection flow must avoid technical jargon like "API Keys" or "Webhooks."

## Priority
P1

## Estimated Scope
Large
