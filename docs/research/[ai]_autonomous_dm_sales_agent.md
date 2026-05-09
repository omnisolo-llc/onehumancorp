# [ai] Autonomous DM Sales Agent

## Problem Statement
SMB owners like Carlos (Handyman) and Maya (Baker) manage communications across Instagram DMs, WhatsApp, SMS, and email. This "Operational Fatigue" (the #2 SMB pain point) leads to missed messages and lost sales, especially since founders cannot answer queries while actively working or sleeping.

## Research Report
*   **Competitor Baseline:** Competitors rely on third-party App Store plugins (e.g., ManyChat for Shopify), which cost extra and require complex visual flow-builders to set up.
*   **User Data:** 40% of founders complain about "Communication Lag"—losing sales due to slow response times.
*   **Opportunity:** OHC can natively consolidate all messages into a "Unified Inbox" and provide an AI "Ambassador" teammate that proactively drafts contextual responses based on store data (inventory, pricing, policies) rather than just generic auto-replies.

## Design Doc
*   **Core Entities:** `UnifiedMessage`, `ConversationThread`, `DraftReply`.
*   **Mobile UX Flow (375px First):**
    1.  **Unified Inbox View:** A single list showing messages from all connected channels (IG, SMS, Web).
    2.  **Action Required Feed:** Messages with AI-drafted replies are highlighted at the top.
    3.  **Draft Review:** User taps a message; the AI's suggested response is pre-filled in the text box. The response is contextual (e.g., "Yes, we have 3 vegan chocolate cakes left for today. Would you like me to hold one for you?").
    4.  **1-Tap Action:** "Approve & Send" or "Edit".
*   **AI Integration Point:** The Silent Ambassador agent continuously monitors the message event mesh. When a new message arrives, it queries the business memory (inventory, past interactions) and generates a `DraftReply` before the user even opens the app.

## Implementation Prompt
Build the "Unified Inbox" and integrate the "Silent Ambassador" AI agent. The critical user journey involves a customer sending a message to the store. The OHC background agent must detect this event, formulate a context-aware draft reply using the current product catalog and business rules, and surface this draft in the mobile app's Unified Inbox. The user must be able to review, edit, or send the AI-drafted reply with a single tap. Acceptance criteria: The system must successfully generate a relevant draft reply for product availability queries without any manual prompting from the store owner.

## Priority
P0

## Estimated Scope
Medium
