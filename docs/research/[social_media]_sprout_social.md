# Social Media Integration: Sprout Social

**Problem Statement:** Non-technical small business owners struggle to manage customer inquiries across multiple platforms (Instagram DMs, Facebook comments, WhatsApp, TikTok). They need a unified inbox to respond to customers quickly without logging into 5 different apps.

**Research Report:** Sprout Social is a highly regarded social media management platform. While known for enterprise capabilities, its unified smart inbox is extremely powerful for centralizing messages.
- Ease of Use: Excellent interface, very intuitive for non-technical users once set up.
- Pricing: Very expensive (~$249/user/month), which is a major barrier for small businesses compared to ManyChat or Meta Business Suite.
- Reputation: Industry leader.
- Cloud vs. Standalone: Primarily Cloud SaaS. Standalone integration would rely on webhooks and API polling.

**Design Doc:**
- User connects social accounts via OAuth in the OHC integrations dashboard.
- New messages from connected platforms (IG, FB, TikTok) appear in a central OHC "Inbox".
- User replies in OHC; the message is sent back to the native platform via Sprout Social's API.
- UI wireframes or screen flow description (375px first): A mobile-first list view of conversations, similar to iMessage or WhatsApp. Tapping a thread opens a standard chat view.
- Mobile UX flow: Bottom nav bar "Inbox" icon with unread badge. Clean, native-feeling chat interface.

**Implementation Prompt:** Implement the Sprout Social unified inbox integration. The user should be able to authenticate their Sprout Social account and view/reply to messages from within the OHC platform. Ensure the UI matches the mobile-first chat design.
- Acceptance Criteria: Messages sync bi-directionally. Unread counts update correctly. Handles rich media (images/videos) in messages.

**Priority:** P2
**Estimated Scope:** Large
