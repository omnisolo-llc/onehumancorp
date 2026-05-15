# Title: Unified Inbox for Social Media Channels

## Problem Statement
Small business owners struggle to manage customer inquiries scattered across multiple platforms (Instagram DMs, Facebook comments, WhatsApp, TikTok). Checking each app manually is time-consuming and leads to missed sales opportunities or slow response times, which frustrates customers.

## Research Report
*   **Competitors:** Manychat, Hootsuite, Sprout Social.
*   **Ease of Use:** Non-technical users need a single, unified view. The setup must be as simple as "Connect with Facebook/Instagram" without requiring API key management.
*   **Pricing:** Tools range from free tiers (limited messages) to $15-$99/month.
*   **Reputation:** High demand for stable, reliable integrations. Webhook reliability is crucial.

## Design Doc
*   **Trigger:** User navigates to Settings > Integrations and clicks "Connect" for respective platforms.
*   **Actions:** OHC authenticates via OAuth, registers webhooks to receive incoming messages/comments, and pulls historical data.
*   **User View:** A unified "Inbox" tab in OHC where messages from all connected platforms appear in a single threaded view. Users can reply directly from OHC, and the response is routed back to the original platform.

## Implementation Prompt
Implement a unified inbox feature that allows users to connect their social media accounts. The system should aggregate incoming messages and comments into a single interface. Users should be able to read and reply to messages from this unified inbox.

## Priority
P1

## Estimated Scope
Medium
