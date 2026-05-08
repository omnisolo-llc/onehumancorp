# [Social Media Integration] Omnichannel Unified Inbox

## Title
Implement Omnichannel Unified Inbox via Twilio Conversations API

## Problem Statement
Small business owners, like boutique owners or local service providers, are overwhelmed by customers contacting them across multiple platforms (Instagram DMs, Facebook Messenger, WhatsApp, and TikTok). Switching between 4 different apps on their phone to answer basic pricing or availability questions leads to missed messages, slow response times, and lost sales. They need a single, unified view where all customer messages appear in one place, so they or their AI agent can respond instantly without juggling apps.

## Research Report
### Tools Evaluated
1. **Meta Graph API (Direct Integration)**
   - **What it solves for the persona:** Connects Facebook, Instagram, and WhatsApp natively.
   - **How it appears to the business owner:** Very frustrating setup requiring Facebook Developer App reviews, business verification, and technical token management.
   - **Key Advantages & Risks:** Direct source, no middleman. Risk is extremely high friction for non-technical users.
   - **Pricing:** Free for basic APIs, WhatsApp has per-conversation pricing (approx $0.08/msg).
   - **Cloud/Standalone:** Cloud is fine. Standalone requires individual user developer accounts, which is unfeasible.

2. **MessageBird (Inbox)**
   - **What it solves for the persona:** Aggregates multiple channels including WhatsApp, SMS, Instagram, and Google Business Messages.
   - **How it appears to the business owner:** Easy onboarding, but often pushes users to their own dashboard rather than staying in OHC.
   - **Key Advantages & Risks:** Great omnichannel support. Risk is that it acts more like a standalone SaaS rather than an API-first tool for white-labeling inside OHC.
   - **Pricing:** Starts at $50/mo + channel fees.
   - **Cloud/Standalone:** Works via API for both modes.

3. **Twilio Conversations API**
   - **What it solves for the persona:** Aggregates SMS, WhatsApp, and chat into a single thread per customer. Handles the webhook routing and channel specific constraints invisibly.
   - **How it appears to the business owner:** Invisible. OHC handles the OAuth flow via a simple "Connect Facebook/WhatsApp" button, and everything just works in the OHC Inbox.
   - **Key Advantages & Risks:** Robust developer ecosystem, abstraction of channel-specific APIs. Risk is dependency on Twilio's roadmap.
   - **Pricing:** $0.05 per active user/month + per-message channel fees (e.g., standard WhatsApp rates). Highly cost-effective for small volumes.
   - **Cloud/Standalone:** Fully supported via REST API in Cloud. In Standalone, OHC can proxy the webhook events to the local instance or run a lightweight polling mechanism, making it highly flexible.

### Recommendation
**Twilio Conversations API** is the recommended choice due to its robust developer ecosystem, abstraction of channel-specific APIs, and reasonable pricing structure. It seamlessly integrates into both Cloud and Standalone modes and hides complexity from the business owner.

## Design Doc
**Trigger:**
The business owner visits the "Channels" tab in the OHC dashboard and clicks "Connect Social Media".

**Actions:**
1. User clicks the connect button, triggering a standard OAuth popup for their social account (e.g., Facebook login).
2. OHC receives the token and registers the channel on the unified inbox backend (using Twilio Conversations).
3. When a customer sends a DM on Instagram, it flows into the OHC unified inbox.
4. The business owner (or their assigned Sales AI Agent) sees the message in the OHC "Inbox" view and types a reply.
5. The reply is routed back out to the customer's native Instagram app.

**What the user sees:**
A simple setup screen with toggle switches for "Instagram", "WhatsApp", etc. Once connected, they see an iMessage-like interface where messages from all platforms are funneled into a single chat list. A small icon indicates whether the message came from IG, WhatsApp, or Facebook.

## Implementation Prompt
**User-Facing Outcome:**
Create a unified inbox UI within the OHC platform. Add an integration screen where users can authenticate their social media accounts. All incoming messages from connected channels should appear in this inbox. Users must be able to reply to these messages directly from the OHC inbox, and the messages must be delivered to the customer's original platform.

**Acceptance Criteria:**
- The integration screen allows connecting at least two channels (e.g., WhatsApp and Instagram).
- Incoming messages from these channels appear in a single, unified conversation list in real-time or near real-time.
- The UI clearly indicates the source channel for each message.
- The user can reply from the OHC UI, and the message successfully reaches the customer's native app.
- Ensure the user experience gracefully handles connection errors without exposing raw API error codes.
- Provide a simple toggle to enable/disable specific channels without losing message history.

## Priority
P1

## Estimated Scope
Large
