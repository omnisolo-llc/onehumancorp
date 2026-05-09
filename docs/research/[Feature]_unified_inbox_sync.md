# [Feature] Unified Inbox Sync

## Problem Statement
Many of our target users (like Maya and Fatima) already have an established customer base on Instagram, WhatsApp, or Facebook. Forcing those customers to move to a new website creates friction. Furthermore, managing communication across multiple apps while trying to fulfill orders causes immense stress and missed sales.

## Research Report
- **Competitive Gap:** Shopify requires third-party apps (like Gorgias) to unify inboxes, which are expensive and designed for larger support teams, not solo founders.
- **User Pain:** "I lost a sale because I missed the DM while I was baking" is a common theme in SMB founder interviews and forums.
- **Data Point:** Omnichannel customers spend more, but omnichannel management is typically reserved for enterprise tools.

## Design Doc
- **Architecture Idea:** A centralized messaging bus that ingests webhooks/API events from connected social accounts (Instagram, WhatsApp, Facebook Messenger) and routes them into a single OHC inbox view.
- **UX Flow (Mobile 375px First):**
  1. User navigates to Inbox.
  2. Clicks "Connect Channels".
  3. OAuth flow for Instagram/FB/WhatsApp.
  4. Inbox now shows a unified feed. Each message has a small platform icon (IG, WA, Site) indicating its origin.
  5. Replying from the OHC inbox sends the message natively back to the source platform.
- **Key Relationships:** External API integrations (Meta Graph API), unified messaging data model, real-time push notifications.

## Implementation Prompt
Build a Unified Inbox feature that aggregates messages from the merchant's OHC website and connected social channels (prioritize Instagram DMs conceptually). The UI must provide a single, seamless chronological feed where merchants can read and reply to messages regardless of origin. Ensure the architecture supports adding future channels easily and handles real-time updates robustly.

## Priority
P1

## Estimated Scope
Large
