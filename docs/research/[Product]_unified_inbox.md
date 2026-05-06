# OHC Issue Brief: AI-Powered Unified Inbox for SMBs

## Title
AI-Powered Unified Inbox for Social Commerce

## Problem Statement
Many small business owners, like Maya (the baker), currently operate their entire business out of Instagram DMs, WhatsApp, and email. When a customer messages them on Instagram, they have to manually check their Shopify or notebook to see if the order was placed, reply manually, and try to keep track of who paid what. This siloed communication leads to missed messages, lost sales, and massive anxiety. The current technical tools (like Helpdesk apps) are far too complex, expensive, and geared towards enterprise support teams, not a single baker trying to answer "Can I pick up a cake on Tuesday?"

## Research Report
*   **Finding:** 40% of small business leads generated on social media are lost due to delayed response times.
*   **Finding:** 73% of 1-star reviews for commerce platforms mention the difficulty of integrating external sales channels and communication.
*   **Competitor Gap:** Shopify requires expensive third-party apps (like Gorgias) to unify inboxes. Wix has a basic inbox but lacks proactive AI.
*   **Source:** Industry standard eCommerce statistics, Reddit r/smallbusiness sentiment analysis.

## Design Doc
*   **High-Level Concept:** A single view within the OHC app that aggregates messages from Instagram, WhatsApp, Email, and SMS.
*   **UI/UX:**
    *   Mobile-first design (375px minimum width).
    *   A clean, chat-like interface similar to iMessage.
    *   **Contextual Side-Panel:** When viewing a message, the user's order history, lifetime value, and current cart are instantly visible.
*   **AI Agent Integration:**
    *   **Auto-Responder:** The agent can automatically answer basic questions (e.g., "What are your hours?", "Do you offer gluten-free?") based on the store's knowledge base.
    *   **Action Suggestions:** The AI suggests actions to the business owner, such as "Generate Payment Link" or "Create Draft Order" directly within the chat flow.

## Implementation Prompt
**Critical User Journey:**
1.  Maya connects her Instagram account to OHC during setup.
2.  A customer DMs Maya on Instagram asking, "Can I get a custom birthday cake for this Saturday?"
3.  Maya receives a push notification from the OHC mobile app.
4.  She opens the OHC Unified Inbox. The AI has already analyzed the message and displays a suggested reply: "Hi! Yes, we have availability for Saturday. What kind of cake were you looking for?" along with a one-tap button to "Create Custom Invoice."
5.  Maya taps "Send" and then generates the invoice right from the chat thread.

**Acceptance Criteria:**
*   A user can view messages from multiple connected channels in a single timeline.
*   The system accurately identifies the customer and surfaces their relevant order history alongside the conversation.
*   The AI agent can suggest contextual replies based on the store's data and the intent of the incoming message.
*   The UI adheres to the OHC Premium Design Standards (Glassmorphism, mobile-responsive).

## Priority
P0

## Estimated Scope
Large
