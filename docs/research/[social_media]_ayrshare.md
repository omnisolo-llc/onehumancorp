# Scout: Tool Integration Research Q2

## 1. Social Media Integration
**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**: Maya the Baker and Carlos the Handyman spend too much time jumping between Instagram DMs, Facebook Comments, and TikTok. They want a single inbox and a way to post to multiple platforms at once without understanding technical integrations.
**Research Report**:
- Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn).
- Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- Pricing: Free tier available, then scales per user.
- Fits OHC’s "The Promoter" agent to automate posts and "The Ambassador" to draft replies.
- Non-technical users benefit by never leaving the OHC interface.
- Works in Cloud mode well; Standalone mode might require personal Ayrshare API keys or direct OAuth.
**Design Doc**:
- Users link their social accounts via a simple OAuth popup in the "Marketing & Advertising" tab.
- "The Ambassador" AI monitors incoming DMs and drafts replies visible in a unified "Customer Inbox."
- "The Promoter" AI schedules and auto-posts images (e.g., new cake designs) to all linked platforms.
**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
**Priority**: P1
**Estimated Scope**: Large
