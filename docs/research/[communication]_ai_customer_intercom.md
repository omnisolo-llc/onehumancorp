# Unified AI Customer Intercom

## Problem Statement
Small business owners, especially service providers (like Carlos the handyman) and social-first sellers (like Maya the baker), manage communications across too many channels (Instagram DMs, WhatsApp, SMS, Email). They miss inquiries when busy, resulting in lost leads, and waste hours answering repetitive questions ("What are your hours?", "Do you ship to Canada?").

## Research Report
- **Competitor Flaws:** Shopify requires third-party apps for robust multi-channel chat, which are often expensive and complex to set up. Platforms like Squarespace offer basic form fills but no intelligent real-time routing.
- **User Pain Points:** 22% of complaints revolve around juggling multiple tools. A massive pain point is the manual effort required to move a lead from an Instagram DM to an actual booked order or quote.
- **Opportunity:** OHC can provide a single "Unified Inbox" where all channels aggregate, and crucially, an AI agent auto-replies to basic queries based on the store's knowledge base, escalating to the human owner only when necessary.

## Design Doc
### High-Level Architecture
- **Entities:** `Message`, `Conversation`, `ChannelIntegration`, `AgentResponseLog`.
- **Integration Points:** Meta Graph API (Instagram/WhatsApp), Twilio (SMS), Internal LLM routing.
### UI Wireframes / Mobile UX Flow (375px)
1.  **Main Inbox Screen:** A unified list of conversations. Badges indicate source (IG, SMS, Web).
2.  **Conversation View:** Standard chat interface. Messages handled by the AI have a subtle "AI Handled" spark icon.
3.  **Agent Intervention:** A toggle at the top of the chat: "Agent Active / Human Takeover". If the human types a message, the Agent automatically pauses for that thread.
### AI Agent Integration
- The Communication Agent listens to incoming webhook events. It queries the store's RAG (Retrieval-Augmented Generation) memory (policies, inventory, pricing). If it can answer confidently, it replies. If it detects intent to purchase, it drops a direct checkout link into the chat.

## Implementation Prompt
**User-Facing Outcome:** The business owner connects their Instagram account. When a customer DMs "Do you have the red dress in medium?", the OHC Agent checks inventory, replies "Yes! We have 2 left. You can buy it here: [link]," and the owner just sees the notification of a completed sale.
**Critical User Journey (CUJ):**
1. User connects social channels in OHC settings.
2. Customer sends a message via Instagram.
3. System routes the message to the OHC Inbox.
4. AI Agent drafts and sends a contextual reply based on store data.
5. Owner reviews the conversation and can seamlessly take over.
**Acceptance Criteria:**
- The Inbox UI must aggregate messages from at least 2 simulated sources (e.g., Web Chat and SMS).
- The AI Agent must demonstrate the ability to read store data (e.g., business hours or inventory) to formulate a reply.
- The UI must clearly differentiate between AI-generated replies and human replies.
- The interface must adhere to the Visual Excellence Mandate (touch targets >= 44x44px).

## Priority
P1

## Estimated Scope
Large