# [feature] Unified Omnichannel Inbox for SMBs

## Problem Statement
Small business owners like Maya (baker) and Carlos (handyman) lose revenue because customer inquiries are scattered across Instagram DMs, WhatsApp, SMS, and email. They miss leads when they are actively working because they cannot monitor multiple apps simultaneously. Existing platforms like Shopify either ignore service-based businesses or require expensive third-party integrations for messaging.

## Research Report
- **Validation:** 22% of surveyed SMB pain points involve lost leads due to scattered communication.
- **Competitor Landscape:**
  - *Shopify:* Relies on Shopify Inbox (primarily web chat) or costly app store plugins.
  - *Wix:* Basic built-in chat, but poor WhatsApp/IG integration.
  - *Square:* Good point-of-sale, weak pre-sale communication.
- **Data:** WhatsApp is the primary business communication tool in emerging markets (LATAM, India). Instagram is primary for US-based creative SMBs.

## Design Doc
### Architecture High-Level
- **Entities:** `Conversation`, `Message`, `CustomerProfile`, `ChannelConfig`.
- **Integration Points:** Webhooks for Meta Graph API (IG/WhatsApp), Twilio (SMS), Email (SendGrid/SES).
- **Core Engine:** A central message router that ingests events from all channels, normalizes them into a unified `Message` entity, and associates them with a `CustomerProfile`.

### UX Wireframes (Mobile First - 375px)
- **Bottom Tab:** A prominent "Inbox" tab with an unread badge.
- **List View:** A unified list of threads. Each thread shows a small icon indicating the source (IG, WA, SMS).
- **Thread View:** Standard chat interface.
- **Agent Integration:** A toggle switch at the top of the thread: "AI Auto-Reply [ON/OFF]".

## Implementation Prompt
**User-Facing Outcome:** The SMB owner opens the OHC mobile app and sees a single "Inbox" containing messages from Instagram, WhatsApp, and their website contact form. They can reply directly from OHC, and the message is routed back to the customer's original platform.

**Critical User Journey:**
1. User connects their IG Business account and WhatsApp number in Settings.
2. A customer sends an IG DM asking about pricing.
3. User receives an OHC push notification.
4. User replies within OHC.
5. Customer receives the reply as an IG DM.

**Acceptance Criteria:**
- Support for at least two external channels (e.g., simulated IG/WhatsApp via webhook ingestion).
- Real-time updates to the UI when a new message arrives.
- Ability to toggle an "AI Auto-responder" on a per-channel basis.

## Priority
P0

## Estimated Scope
Large
