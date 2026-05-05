## [Social Media] Issue Brief: Unified Social Media Inbox and Cross-Posting

**Title**: Scout 🔍: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**:
Maya the Baker and Carlos the Handyman spend too much time jumping between Instagram DMs, Facebook Comments, and TikTok. They want a single inbox and a way to post to multiple platforms at once without understanding technical integrations. They need to manage all social interactions natively within OHC without friction.

**Research Report**:
- **Tool**: Ayrshare API.
- **Evaluation**: Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn). Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- **Ease of Use**: Non-technical users benefit by never leaving the OHC interface. They authenticate once per platform, and the rest is managed transparently.
- **Advantages**: Consolidates multiple disparate APIs into one unified interface for the developer, and one unified inbox for the user. Fits OHC’s "The Promoter" agent to automate posts and "The Ambassador" to draft replies.
- **Risks**: Relying on a third-party aggregator instead of direct APIs. If Ayrshare loses a platform integration, OHC loses it.
- **Pricing**: Free tier available, then scales per user.
- **Compatibility**: Works in Cloud mode well; Standalone mode might require personal Ayrshare API keys or direct OAuth.

**Design Doc**:
- Users link their social accounts via a simple OAuth popup in the "Marketing & Advertising" tab.
- "The Ambassador" AI monitors incoming DMs and drafts replies visible in a unified "Customer Inbox."
- "The Promoter" AI schedules and auto-posts images (e.g., new cake designs) to all linked platforms.

**Implementation Prompt**:
Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts using Ayrshare.
- **Acceptance Criteria**: Users can connect multiple social platforms. Incoming DMs are unified in one inbox. Outbound posts can be scheduled to multiple platforms at once.
**Priority**: P1
**Estimated Scope**: Large
