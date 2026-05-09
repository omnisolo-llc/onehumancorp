# Integration Issue Brief: Social Media Inbox & Management (Hootsuite)

## Title
Social Media Inbox Integration: Hootsuite

## Problem Statement
Small business owners often struggle to manage incoming customer inquiries across multiple platforms. With messages pouring into Instagram DMs, Facebook Messenger, WhatsApp, and TikTok, owners lose track of conversations, miss sales opportunities, and spend hours context-switching between apps. They need a unified view where all social messages are aggregated so they can respond efficiently without being a technical expert.

## Research Report
*   **Tool Evaluated**: Hootsuite
*   **Ease of Use**: Designed for non-technical users, Hootsuite provides a straightforward dashboard that centralizes streams from different networks. While advanced analytics and scheduling exist, the unified inbox is highly accessible for a small business owner.
*   **Market Position & Reputation**: Hootsuite is a legacy leader in social media management, widely trusted, though sometimes viewed as complex due to enterprise features. However, its core inbox functionality is robust.
*   **Pricing**: Paid plans start at $99/month (Standard plan) for 1 user, 10 social accounts, and unlimited scheduling/inbox access. Advanced plans start at $249/month. A 30-day free trial is available.
*   **Cloud vs. Standalone Compatibility**: Hootsuite operates entirely in the cloud via its APIs. For OHC Cloud, we can integrate via their APIs/webhooks. For Standalone mode, integration would still require the local app to reach out to Hootsuite's cloud APIs, which is typical for SaaS integrations.

## Design Doc
*   **Integration Trigger**: The user authenticates their Hootsuite account via OAuth 2.0 in the OHC integrations dashboard.
*   **Action Flow**:
    1.  OHC listens for new messages via Hootsuite's webhooks (or periodically polls for Standalone).
    2.  Messages from connected channels (IG, FB, etc.) are ingested into OHC's unified inbox.
    3.  When the user replies in OHC, the message is pushed back to Hootsuite's API, which dispatches it to the native platform.
*   **User Experience**: The business owner sees a new "Social Messages" tab in OHC. They don't need to log into Hootsuite daily; OHC becomes the single pane of glass.

## Implementation Prompt
Implement a unified social inbox integration using Hootsuite as the backend aggregator. The user should be able to connect their Hootsuite account in the OHC settings. Once connected, incoming messages from all their configured social channels should appear in the OHC inbox. Replying to a message within OHC must successfully send the reply back to the customer on the original social platform. Ensure the UI clearly indicates which platform a message originated from (e.g., an Instagram icon next to an IG DM).

## Priority
P1

## Estimated Scope
Medium
