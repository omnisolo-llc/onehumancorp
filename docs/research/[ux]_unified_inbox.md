# SMB Pain Point: Fragmented Tooling (The Unified Inbox)

## Problem Statement
Small business owners—especially service providers like Carlos (handyman) and Maya (baker)—suffer from "tool fragmentation." They manage leads in Instagram DMs, text messages, email, and website contact forms. This fragmentation leads to missed opportunities, slow response times, and an overwhelming administrative burden that prevents them from actually doing their work.

## Research Report
Based on an analysis of r/smallbusiness and app store reviews for incumbent platforms:
*   **The Problem:** Owners report losing track of customer inquiries because they span 3-4 different apps. A lead might DM on Instagram, follow up via SMS, and finally send an email.
*   **Competitor Failures:** Platforms like Shopify focus heavily on order management but treat communication as an afterthought (often requiring third-party apps like Gorgias). Wix has an inbox, but it often struggles to seamlessly merge social DMs and SMS natively in a mobile-friendly way.
*   **The Opportunity:** OHC must provide a single, unified "Feed" or "Inbox" on the mobile app where every customer interaction (Web chat, SMS, IG DM, Email) is threaded by the customer profile, not the channel.

## Design Doc
*   **Core Entities:** `UnifiedMessage`, `CustomerProfile`, `CommunicationChannel`
*   **Integration Points:**
    *   Event Mesh (NATS/Redis) for real-time message ingestion from various webhooks (Meta Graph API, Twilio, SendGrid).
    *   PostgreSQL for persistent storage of threaded conversations.
*   **UX Flow (Mobile First):**
    1.  The primary screen of the OHC app is the "Inbox/Feed".
    2.  Messages appear chronologically, regardless of source.
    3.  A small icon indicates the source (Instagram, SMS, Web).
    4.  Tapping a message opens a thread where the owner can reply. The platform automatically routes the reply back through the correct channel.
    5.  The AI agent drafts suggested replies based on the context of the conversation and the business's data.

## Implementation Prompt
Implement the backend architecture for a Unified Inbox. Create the data models necessary to ingest, thread, and store messages from disparate sources (Web, SMS, Social) and associate them with a single `CustomerProfile`. Develop the API endpoints required for the mobile app to fetch these threaded conversations and to send replies back through the appropriate channel webhook. The system must support real-time updates via the existing event mesh. *Do not prescribe the specific table schemas or function signatures.*

## Priority
P1

## Estimated Scope
Medium
