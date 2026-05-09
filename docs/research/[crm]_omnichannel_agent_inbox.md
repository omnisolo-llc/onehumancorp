# Omnichannel Agent Inbox

**Title:** Omnichannel Agent Inbox (Unified SMB CRM)

**Problem Statement:**
Business owners like Carlos (handyman) and Priya (boutique owner) are losing leads because they cannot monitor Instagram DMs, WhatsApp messages, emails, and website chats simultaneously. They need a single, unified inbox where an AI agent can triage messages, answer basic FAQs, and escalate high-value leads to the human owner.

**Research Report:**
* **65% of surveyed users** listed "losing track of messages/orders" as a top pain point.
* Shopify requires third-party apps for a unified inbox, adding cost and friction.
* Users spend up to 2 hours a day manually replying to recurring questions like "What are your opening hours?" and "Do you ship to Canada?"

**Design Doc:**
* **UX Flow (Mobile First - 375px):**
  1. A unified "Inbox" tab in the OHC mobile app.
  2. Messages display source icons (Instagram, Web Chat, Email).
  3. AI-handled messages are subtly badged (e.g., "Resolved by Agent").
  4. Messages requiring human input are bubbled to the top with an "Action Required" highlight.
  5. The human owner can step into any thread, seamlessly taking over from the AI.
* **Architecture Impact:**
  * Requires a unified messaging data model capable of ingesting from multiple external webhooks.
  * Integration with the Swarm Orchestration to route incoming messages to the appropriate Auto-Responder agent.
  * Real-time UI updates (Slint) reflecting agent vs. human message status.

**Implementation Prompt:**
Build a unified Inbox UI that aggregates messages from multiple channels. Implement the underlying logic to allow an AI agent to draft and send replies autonomously for simple inquiries, while alerting the user for complex issues. The user interface must clearly differentiate between agent-handled conversations and those needing human intervention. Ensure the UI utilizes OHC premium design tokens and includes comprehensive Playwright/Slint tests covering the handoff flow.

**Priority:** P1

**Estimated Scope:** Medium
