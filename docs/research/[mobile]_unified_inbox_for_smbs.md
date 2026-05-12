# Unified Inbox for SMBs

## Problem Statement
Merchants are managing customer communications across Instagram DMs, Facebook Messenger, SMS, and Email. This context-switching leads to lost orders and poor customer experience.

## Research Report
* **Finding:** "Managing messages everywhere" is a top complaint among side-hustlers on Reddit.
* **Competitor Comparison:** High-end CRMs have this, but entry-level platforms (GoDaddy, Wix) lack a truly unified, mobile-first inbox.

## Design Doc
* **Architecture:** Event mesh architecture aggregating webhooks from Meta Graph API, Twilio (SMS), and Email providers into a single 'Conversation' entity.
* **Mobile UX Flow:** Single screen showing all messages sorted by urgency. Badges indicate the source platform.

## Implementation Prompt
**Critical User Journey:** A merchant opens the OHC app and sees messages from Instagram, SMS, and Email in one single list, and can reply directly from the app.
**Acceptance Criteria:**
* System can ingest messages from multiple simulated sources.
* Messages are aggregated into a single chronological feed.
* Merchant can send a reply that routes back to the correct original channel.

## Priority
P1

## Estimated Scope
Large
