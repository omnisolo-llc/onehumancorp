# Social Media Unified Inbox Integration

## Problem Statement
Small business owners lose valuable time and potential sales by constantly switching between Instagram, Facebook, and WhatsApp to respond to customer inquiries. They need a single, simple dashboard to read and reply to all messages.

## Research Report
The Meta Business Suite API and WhatsApp Business API offer the most comprehensive solution, given Meta's dominance in social commerce. While the integration is complex due to strict App Review and OAuth processes, the benefit to the end-user is immense. Alternatives like Twilio offer WhatsApp, but native Meta APIs are required for full IG/FB support.
*   **Ease of use (end user):** Extremely high. One inbox to rule them all.
*   **Pricing:** Free for basic inbound, per-conversation pricing for WhatsApp.
*   **Reputation:** Industry standard.

## Design Doc
OHC will introduce a "Unified Inbox" module.
1.  **Trigger:** User navigates to the "Integrations" page and clicks "Connect Facebook/Instagram/WhatsApp".
2.  **Action:** A secure OAuth flow redirects the user to Meta to grant permissions.
3.  **User Sees:** A new "Inbox" tab appears in the main navigation. All incoming messages from connected channels appear in a threaded view. The user can reply directly from OHC, and the message is routed back to the appropriate platform.

## Implementation Prompt
Implement a unified inbox interface in the OHC application.
*   Create a settings page where users can authenticate and connect their Meta accounts (Facebook Page, Instagram Business, WhatsApp).
*   Build an "Inbox" UI that displays a list of conversations and a detail view for the active conversation.
*   The UI must support receiving incoming messages and sending replies.
*   Ensure the design is mobile-first, as business owners will likely manage messages from their phones.
*   Acceptance Criteria: A user can connect an account, receive a simulated message, and reply to it successfully through the UI.

## Priority
P0

## Estimated Scope
Large
