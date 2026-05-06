# [Product Gap] Unified Social Media Inbox (The Ambassador)

## Title
Implement Unified Social Media Inbox for Seamless Customer Communication

## Problem Statement
Small business owners (like Maya the baker) are overwhelmed managing customer inquiries across Instagram DMs, Facebook Messenger, and Email. They miss messages, lose sales, and spend hours checking multiple apps. They need a single place to see all messages and reply easily, preferably with AI drafting the responses.

## Research Report
*   **Competitor Landscape:** Shopify offers basic unified messaging, but it's often a paid add-on or requires third-party apps (e.g., Gorgias). Wix has a unified inbox, but it lacks strong autonomous AI features.
*   **User Pain Point Data:** Reddit `r/smallbusiness` frequently cites "managing DMs" as a massive time sink. App store reviews for business tools often request "all messages in one place."
*   **OHC Advantage:** By integrating the "Ambassador" AI agent, OHC can not only aggregate messages but *draft replies* autonomously based on the business's knowledge base (hours, pricing, FAQs).

## Design Doc
*   **Entities:** `Message`, `Conversation`, `Channel` (Instagram, Facebook, Email).
*   **Architecture:**
    *   Integration with Meta Graph API (Instagram/Facebook) and Resend (Email).
    *   Webhook listeners to receive incoming messages.
    *   Routing layer to connect incoming messages to the correct tenant and conversation thread.
    *   AI Agent (Ambassador) hook to process new messages and generate suggested replies.
*   **UI Wireframe/Flow (375px first):**
    *   **Screen 1: Inbox List.** A unified list of all recent conversations, badged by channel icon (IG, FB, Mail). Unread messages bolded.
    *   **Screen 2: Conversation View.** Standard chat interface. Incoming messages on the left, outgoing on the right.
    *   **AI Integration:** Above the text input area, an "AI Suggestion" chip. Tapping it populates the text area with a drafted reply (e.g., "Hi! Yes, we have that cake available for Saturday. Would you like to reserve it?").

## Implementation Prompt
Implement a unified inbox feature that allows users to connect their social media accounts (starting with Instagram) and view/reply to messages directly within the OHC platform. The system should support receiving messages via webhooks and sending replies back via the respective APIs. Crucially, integrate the 'Ambassador' AI agent to automatically draft suggested replies for incoming messages based on the business's context.

## Priority
P0

## Estimated Scope
Large
