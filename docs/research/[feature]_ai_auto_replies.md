# [feature] AI Auto-Replies for Unified Inbox

**Title**: Implement AI Auto-Replies for Unified Inbox

**Problem Statement**:
Small business owners (like Maya the baker) spend hours daily answering repetitive questions in Instagram DMs ("What are your hours?", "Do you ship to Texas?"). They miss messages when busy, leading to lost sales. They need an invisible assistant that answers basic questions automatically, so they only have to step in for complex custom orders.

**Research Report**:
- 58% of social-first sellers cite "managing DMs" as their biggest time sink.
- Shopify requires expensive third-party apps (e.g., Gorgias) for this functionality, which is too complex for beginners.
- OHC can leapfrog competitors by making this a native, zero-configuration feature.

**Design Doc**:
- **Architecture**:
  - Webhook ingest from Meta Graph API (Instagram/Messenger).
  - Background task worker processes incoming message.
  - LLM context includes business profile, operating hours, and current inventory.
  - System determines confidence score. If high, auto-reply. If low, draft reply and notify user.
- **UI/UX Flow (Mobile 375px first)**:
  - User opens "Inbox" tab.
  - Messages handled by AI have a small "✨ Auto-replied" badge.
  - User can tap to read the AI's reply and step in if needed.
  - Simple toggle in settings: "Let AI answer basic questions."

**Implementation Prompt**:
Develop the Unified Inbox backend and mobile UI to support AI auto-replies. The system must ingest messages, evaluate them using an LLM against the store's context, and execute auto-replies for high-confidence queries. The UI must clearly indicate which messages were handled by the AI and allow the user to easily take over the conversation. Ensure the entire feature is fully functional on a 375px mobile screen.

**Priority**: P0
**Estimated Scope**: Large
