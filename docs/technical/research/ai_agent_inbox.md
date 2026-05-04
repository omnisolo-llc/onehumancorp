# Issue Brief: AI Agent Inbox Integration (Instagram/WhatsApp) (P0)

## Problem Statement
Small business owners (especially personas like Maya the baker) conduct the majority of their early sales via social media Direct Messages (Instagram, WhatsApp, Facebook Messenger). As they grow, responding to inquiries ("Do you do vegan cakes?", "How much is delivery?") becomes a full-time job. Missed DMs equal lost revenue, but they cannot afford a dedicated customer service representative.

## Research Report
*   **Target Persona:** Solopreneurs heavily reliant on social selling (Maya/Priya).
*   **Pain Point Validation:** "Drowning in DMs" is a near-universal complaint among early-stage product creators.
*   **Competitor Analysis:** Competitors offer basic chatbots that feel robotic and lack context about the business's actual inventory or capabilities.
*   **Opportunity:** OHC can deploy "The Ambassador" AI to autonomously read DMs, understand the context of the business (using the pgvector memory layer), and draft highly accurate, personalized replies for the owner to simply approve and send (or eventually, auto-send).

## Design Doc
*   **High-Level Architecture:**
    *   Unified `InboxMessage` entity aggregating multi-channel communications.
    *   Integration with Meta Graph API (Instagram/WhatsApp).
    *   Pipeline triggering the LLM provider (Gemini/GPT-4o) upon new message receipt, passing relevant business context (FAQs, inventory state).
*   **UI/UX Flow (Mobile-First):**
    *   **Owner View:** A unified "Inbox" screen. Each message thread shows the customer's message and a suggested AI reply highlighted in a distinct color. The owner can tap "Send," "Edit," or "Reject."
*   **AI Integration:** "The Ambassador" agent acts as the first line of defense, drafting responses based on the business's specific data profile.

## Implementation Prompt
Implement a unified inbox interface in the frontend that displays incoming messages from multiple hypothetical channels. Build the backend pipeline to receive a message, fetch relevant context from the vector database (e.g., product availability or business policies), and generate a suggested reply using the configured LLM provider. The UI must clearly differentiate between the customer message and the AI-suggested draft, allowing the user to approve or edit the draft with a single tap.

*   **Priority:** P0
*   **Estimated Scope:** Medium
