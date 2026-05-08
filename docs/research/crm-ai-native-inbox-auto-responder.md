# [CRM] AI-Native Inbox Auto-Responder

**Priority:** P0 | **Estimated Scope:** Large

## Problem Statement
Small business owners (like Maya the baker and Carlos the handyman) lose leads because they can't monitor Instagram DMs, emails, and site chats 24/7. Current tools like Shopify Sidekick are focused on the admin, not customer-facing auto-replies.

## Research Report
**Findings:**
- 73% of 1-star reviews for SMB platforms cite poor customer management.
- Shopify's Sidekick helps the owner, but doesn't answer customer questions automatically.
- Wix provides simple automations but lacks conversational AI.
**Evidence:** Trustpilot and Reddit (r/smallbusiness) show owners spend 2+ hours daily on DMs.

## Design Doc
**Architecture:**
- Entity Types: `CustomerMessage`, `AiConversation`, `AutoReplyPolicy`
- Key Relationships: A `CustomerMessage` triggers an `AiConversation` based on the `AutoReplyPolicy`.
- Integration Points: Meta Graph API (Instagram/FB), Email Parsing, Native Site Chat.
- Mobile UX: 375px first inbox view with 'AI Handled' and 'Requires Attention' tabs.
- AI Integration: Background agent processing incoming webhooks.

## Implementation Prompt
Implement an AI-Native Inbox that aggregates customer messages and automatically responds based on store context (hours, inventory, pricing). The UI should allow the owner to toggle AI on/off and take over conversations. Ensure it is mobile-first.
