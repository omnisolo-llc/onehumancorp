# Scout: Tool Integration Research

## [Social Media] Issue Brief: TikTok Comment Integration
**Title**: Integrate TikTok API for Unified Comment Management
**Problem Statement**:
Maya (Home Baker) uses TikTok to showcase her creations. She often gets inquiries like "How much?" or "Where are you located?" in her video comments but misses them because she isn't constantly checking the TikTok app. She needs these inquiries to surface in her OHC dashboard.

**Research Report**:
- **Tool**: TikTok for Business API (Comment Management).
- **Evaluation**:
  - **Ease of Use**: Easy. Standard OAuth "Login with TikTok" flow.
  - **Pricing**: Free for API access.
  - **Reputation**: Rapidly becoming a primary discovery engine for small businesses.
  - **Cloud vs. Standalone**: Works in both. Requires a registered TikTok Developer App.
- **Key Advantages**: Converts social engagement directly into the OHC sales funnel.
- **Risks**: TikTok API has stricter rate limits than Meta for comment retrieval.

**Design Doc**:
- **User Flow**: User connects TikTok via "Marketing" settings.
- **Integration**: OHC polls or receives webhooks for new comments on the user's videos.
- **User Experience**: Comments appear in the unified "Activity Feed". AI classifies them as "Inquiry" or "Feedback".
- **AI Action**: The AI "Promoter" drafts replies to common questions about price or location, which the user can approve with one tap.

**Implementation Prompt**:
Build an integration with the TikTok for Business API to sync video comments into the OHC unified inbox. Implement an OAuth flow for TikTok and a background worker to fetch new comments. The system should categorize comments and allow the business owner to reply to TikTok comments directly from the OHC UI.

**Priority**: P2
**Estimated Scope**: Medium
