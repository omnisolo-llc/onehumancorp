# Social Media Unified Inbox Integration

## Title
Social Media Unified Inbox Integration

## Problem Statement
Small business owners often struggle to manage customer inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp, TikTok). Switching between apps is time-consuming, and messages frequently fall through the cracks, leading to lost sales and poor customer service. They need a single, unified inbox to view and respond to all social media interactions.

## Research Report
*   **Tool:** Meta API (Instagram, Facebook, WhatsApp) and TikTok For Business API.
*   **Market Analysis:** The vast majority of B2C small business interactions happen on these platforms. Consolidating them is a highly requested feature.
*   **Competitor Analysis:** Tools like Hootsuite and Sprout Social offer this, but they are often too expensive or complex for micro-businesses. Integrating this directly into OHC provides immense value.
*   **Ease of Use:** For the business owner, the experience should be seamless after a one-time OAuth connection.
*   **Pricing:** The APIs are generally free for the core messaging functionality, though WhatsApp Business API has per-conversation pricing that would need to be managed or passed on.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Works well. Requires managing OAuth tokens securely per tenant.
    *   *Standalone:* Can work, but requires careful handling of OAuth redirect URIs (e.g., using a local proxy or deep links) and ensuring the local app can receive webhooks securely (or polling, if webhooks aren't feasible locally without a tunnel).

## Design Doc
*   **User Journey:** The user navigates to an "Integrations" or "Inbox" settings page in OHC. They click "Connect Instagram/Facebook/WhatsApp/TikTok". An OAuth popup appears for them to authenticate. Once connected, new messages from these platforms appear in the unified OHC inbox. They can reply directly from OHC, and the message is routed back to the original platform.
*   **Triggers:** Incoming webhooks from Meta/TikTok for new messages.
*   **Actions:**
    *   Display new messages in the OHC UI.
    *   Send replies typed in OHC back to the respective platform API.
*   **Visuals:** A clear list of connected accounts with status indicators. A unified chat interface with icons indicating the source platform for each message.

## Implementation Prompt
Implement a unified inbox experience where a small business owner can connect their social media accounts (Instagram, Facebook, WhatsApp, TikTok) and manage all customer interactions from a single screen within OHC. The integration must handle the OAuth flow smoothly and reliably synchronize messages in both directions. The user should not need to worry about API keys or webhooks; the setup should be intuitive and "just work." Ensure the solution is viable for both Cloud (multi-tenant) and Standalone (local) deployments.

## Priority
P0

## Estimated Scope
Large
