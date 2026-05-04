# [Customer Success] Proactive Omnichannel Inbox Agent (The Silent Ambassador)

## Title
Implement the "Silent Ambassador" Agent for Proactive Omnichannel Message Drafting

## Problem Statement
Small business owners, particularly those running service businesses (like Carlos the Handyman) or micro-retail (like Maya the Baker), suffer from "Communication Lag" and the "Never-ending Inbox" (Ranked #2, 68% frequency). Prospects send messages across Instagram DMs, WhatsApp, SMS, and email. Owners lose sales because they are busy working and cannot reply immediately. They need a unified inbox that doesn't just aggregate messages, but automatically drafts context-aware replies ready for one-tap sending.

## Research Report
*   **Competitor Landscape:** Shopify offers a basic inbox, but advanced automation requires expensive third-party apps like Gorgias. Wix has a unified inbox but lacks proactive, context-aware AI drafting. Most solutions provide "chatbots" that annoy customers rather than assisting the human owner.
*   **User Evidence:** "I lose track of orders in DMs" is a common complaint. Solopreneurs report losing up to 30% of potential custom orders because they replied 8 hours late.
*   **OHC Differentiation:** OHC provides a single, unified inbox natively. More importantly, the "Silent Ambassador" agent watches incoming messages across all channels. It uses RAG (Retrieval-Augmented Generation) against the business's product catalog, store policies, and past conversation history to draft a highly accurate reply. The owner just reviews and taps "Send."

## Design Doc
*   **Core Entities:** `Customer`, `Message`, `ConversationThread`, `DraftReply`.
*   **Key Relationships:** An incoming `Message` triggers the Customer Success Agent. The Agent queries the Vector DB (`AgentMemory`) for context and generates a `DraftReply` attached to the `ConversationThread`.
*   **Integration Points:**
    *   **Trigger:** External webhook (e.g., Meta Graph API for Instagram/WhatsApp) drops a message onto the event mesh (`ohc.inbox.message_received`).
    *   **Logic:** The agent retrieves context (e.g., "Customer is asking about vegan cakes; check product catalog for 'vegan'"). It generates a draft.
    *   **Output:** The mobile UI displays the incoming message with the pre-filled text box.
*   **UI/UX Flow (Mobile-First, 375px):**
    1.  User receives a push notification: "New IG DM from Sarah: Do you do vegan cakes?"
    2.  User taps notification, opening the OHC Unified Inbox thread.
    3.  Sarah's message is visible. The reply input box is already populated with: "Hi Sarah! Yes, we offer a Vegan Chocolate Cake and a Vegan Vanilla Bean option. Would you like to see the pricing?"
    4.  A subtle "Sparkle" icon indicates the draft was AI-generated.
    5.  User can tap the text to edit (opening the native keyboard) or simply tap the "Send" arrow.

## Implementation Prompt
Implement the omnichannel inbox architecture and the AI drafting agent.
1.  Define the `Message` and `ConversationThread` data models to support multiple channels (Email, SMS, IG, etc.).
2.  Implement the event-driven Customer Success Agent that listens for new messages.
3.  Build the LLM prompt pipeline that injects business context (catalog, policies) to generate the `DraftReply`. Ensure the tone matches the business's configured persona.
4.  Develop the Chat UI in Flutter/Slint. It must be a familiar, mobile-native chat interface (like iMessage or WhatsApp). Crucially, implement the pre-filled input box state where the AI draft is waiting for user approval.

## Priority
P0

## Estimated Scope
Large
