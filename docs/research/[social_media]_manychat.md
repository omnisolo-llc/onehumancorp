# Integrate ManyChat for Unified Social Media DMs

## Problem Statement
Small business owners get overwhelmed when trying to manage direct messages across Instagram, Facebook, and WhatsApp. Important customer inquiries get lost because owners have to constantly switch between apps. They need a single, unified inbox within OHC where they can view and respond to messages from all major social platforms, allowing them to capture leads and resolve issues without missing a beat.

## Research Report
*   **Tool:** ManyChat (or similar multi-platform inbox tools / official Meta APIs)
*   **Problem Solved:** Aggregates DMs from Instagram, Facebook Messenger, and WhatsApp into a single stream.
*   **Ease of Use:** High. Once connected via OAuth, users don't need to leave OHC to manage their communications. The integration is straightforward for the end user.
*   **Pricing:** ManyChat offers a free tier (sufficient for early-stage testing), with Pro plans starting at $15/month based on contact volume.
*   **Reputation:** Well-established tool used by millions of small businesses.
*   **Environment:** Works well in Cloud mode. Standalone mode might require a cloud-relay or direct API connections if webhooks are blocked by local firewalls.
*   **Advantages:** Centralized communication saves time; reduces missed sales opportunities.
*   **Risks:** OAuth connection drops (common with Meta platforms) requiring user re-authentication; API rate limits.

## Design Doc
1.  **Trigger:** A new "Connect Social Inbox" button in the OHC settings or dashboard.
2.  **Action:** User clicks the button and is guided through a standard OAuth flow to grant OHC access to their Meta/WhatsApp accounts.
3.  **User Interface:** A new "Unified Inbox" tab appears in OHC. It displays a real-time list of incoming messages from all connected platforms, visually indicating the source (e.g., an Instagram icon). The user can click a message thread, type a reply, and hit send. The message is routed back to the appropriate platform seamlessly.
4.  **Error Handling:** If the connection drops, a prominent banner alerts the user to "Reconnect Instagram to keep receiving messages."

## Implementation Prompt
Implement a unified inbox feature for small business owners. The user must be able to securely connect their Instagram, Facebook, and WhatsApp accounts via a simple interface. Once connected, all incoming messages must appear in a centralized feed within OHC. Users must be able to read and reply to these messages directly from OHC, with the replies correctly delivered to the customer on the original platform. Ensure clear visual indicators for message sources and robust alerts if the integration needs to be re-authenticated.

## Priority
P1

## Estimated Scope
Medium
