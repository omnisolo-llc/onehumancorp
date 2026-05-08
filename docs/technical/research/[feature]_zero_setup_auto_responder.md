# [feature] Zero-Setup Auto-Responder AI

**Problem Statement:** Solopreneurs (like Maya selling via IG DMs) lose sales because they cannot reply to customer inquiries instantly while working.

**Research Report:** Current platforms offer chatbots that require complex dialogue tree setup. Users want it to "just work."

**Design Doc:**
- **Entities:** Conversation, Message, Intent, Action.
- **UX:** A single toggle switch: "Let AI handle incoming inquiries". No dialogue tree setup required.

**Implementation Prompt:** Create a background worker that intercepts incoming messages (web chat/social integration), parses intent against the business context, and replies autonomously based on business rules.

**Priority:** P0

**Estimated Scope:** Large
