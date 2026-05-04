# AI Customer Success Ambassador (Unified Inbox)

### Title
AI Customer Success Ambassador: Unified Omnichannel Inbox

### Problem Statement
Small business owners (like Maya the baker) lose significant revenue and experience "operational fatigue" because customer communications are scattered across multiple platforms (Instagram DMs, email, SMS, website chat). They often answer the same basic questions (e.g., "Do you do vegan cakes?") repeatedly, usually while trying to fulfill orders or sleeping. Competitors like Shopify or Wix require expensive third-party App Store plugins to centralize messaging, which still lack native, invisible AI automation.

### Research Report
- **Market Gap:** "Scattered Communications" and "Operational Fatigue" rank as the #2 and #8 highest pain points among SMBs (affecting 68% and 40% of users, respectively). Current solutions treat AI as a reactive tool, requiring the user to open a chat window and prompt an AI.
- **Competitor Landscape:**
  - **Shopify:** Relies on third-party apps like Gorgias ($$) or basic Shopify Inbox. Sidekick is conversational and not event-driven.
  - **Wix:** Basic native inbox; AI features are focused on initial site generation, not continuous customer support.
  - **Squarespace:** Very basic email/form routing.
- **OHC Differentiation:** Treat AI as a "Teammate" (Proactive, event-driven). The Ambassador agent watches the event mesh for incoming messages from any channel, correlates them with the customer's profile and order history, and proactively drafts a reply.

### Design Doc

**Architecture:**
- **Entity Types:** `Customer`, `MessageThread`, `Message`, `AgentDraft`.
- **Key Relationships:** A `MessageThread` belongs to a `Customer` and a `Tenant`. A `MessageThread` contains many `Message`s. A `Message` can have an `AgentDraft` attached if it requires a response.
- **Integration Points:**
  - Inbound webhook listeners for external channels (e.g., Instagram Graph API, Email ingest, SMS via Twilio).
  - Event mesh triggers the `Customer Success Ambassador` agent on new inbound `Message`.
  - The Agent uses RAG (Retrieval-Augmented Generation) against the tenant's business memory (FAQs, past orders, product catalog) to generate a response draft.

**UI Flow (Mobile-First 375px):**
1. **Lock Screen / Push Notification:** "New IG DM from Sarah: 'Do you have vegan options?' - Tap to review AI reply."
2. **Unified Inbox Screen:**
   - A clean, WhatsApp-style list of active threads, agnostic of the source channel.
   - Channel icons (IG, Email, Web) are small badges on the avatar.
3. **Thread Screen:**
   - Standard chat interface.
   - At the bottom, just above the keyboard, an "AI Draft" card is prominently displayed with the suggested response: "Hi Sarah! Yes, we have 3 vegan cake options available. Would you like a link to the menu?"
   - Two large buttons: **[Approve & Send]** or **[Edit]**.
4. **Settings:**
   - Simple toggles to connect channels (Instagram, Facebook Page, Support Email).
   - A "Business Knowledge" text area where the owner can type rules (e.g., "Always say we need 48 hours notice for custom cakes.").

### Implementation Prompt
Implement the "Unified Omnichannel Inbox" feature powered by the Customer Success Ambassador agent. The system should present a mobile-first (375px) chat interface that consolidates messages from various channels. When a new message arrives, the backend must trigger an AI agent to draft a contextual reply based on the tenant's product data and past interactions. The UI must display this draft prominently to the user, allowing for 1-tap approval and sending, or manual editing. Focus on radical simplicity: hide the complexity of multi-channel routing from the user.

- **Critical User Journey:** User receives a customer question from a connected channel -> User opens the OHC app -> User sees the AI-drafted reply -> User taps "Approve & Send" -> The message is dispatched back to the original channel.
- **Acceptance Criteria:**
  - UI displays a unified list of message threads.
  - Chat interface shows incoming messages and a clearly separated AI-generated draft response.
  - User can send the draft with one tap or edit it before sending.
  - The feature must be fully functional and visually excellent on a 375px mobile breakpoint using the Glassmorphism design tokens.

### Priority
P0

### Estimated Scope
Medium
