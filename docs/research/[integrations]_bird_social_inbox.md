# Issue Brief: Unified Social Inbox via Bird.com

## Title
Unified Customer Inbox Integration (Instagram, WhatsApp, TikTok)

## Problem Statement
"I'm missing sales because I didn't see a message on Instagram while I was checking my email." Small business owners like Maya (the baker) are overwhelmed by "notification popcorn"—messages scattered across 5 different apps. They need a single, "Radically Simple" place to see and reply to every customer, regardless of where the customer reached out.

## Research Report
- **Tool**: Bird.com (formerly MessageBird) Omnichannel API.
- **Ease of Use**: High for end-users; provides a unified widget and API for 20+ channels.
- **Persona Fit**:
    - **Maya (Baker)**: Instagram DMs and WhatsApp messages flow into her OHC dashboard.
    - **Fatima (Food Cart)**: Receives pre-order inquiries via SMS or WhatsApp in one list.
- **Cloud vs. Standalone**:
    - **Cloud**: Full webhook integration for real-time sync.
    - **Standalone**: Can pull messages via polling or authenticated local webhooks if a tunnel is present.
- **Pricing**: Pay-as-you-go. WhatsApp has conversation-based pricing (approx. $0.01 - $0.05 per conversation). Bird.com offers a generous free tier for low-volume starters.
- **Competitive Analysis**: Shopify Inbox is great but limited to Shopify. Bird.com allows OHC to own the relationship across any channel.

## Design Doc
- **Integration**: "The Ambassador" (Customer Success Agent) monitors the Bird.com webhook.
- **User Experience**:
    - User connects their social accounts in the OHC "Setup Wizard".
    - A "Unified Inbox" card appears on the mobile dashboard (375px).
    - AI Agent drafts replies to common questions (e.g., "Do you ship to Berlin?") for the user to approve with one tap.

## Implementation Prompt
Implement a connector for the Bird.com Omnichannel API. The system should receive webhooks for incoming messages from Instagram DMs, WhatsApp, and SMS, and normalize them into the OHC `messages` table. Ensure the "The Ambassador" AI agent is notified to generate draft responses. Support multi-tenant isolation using `tenant_id`.

## Priority
P0 (Critical for Maya and Fatima)

## Estimated Scope
Medium
