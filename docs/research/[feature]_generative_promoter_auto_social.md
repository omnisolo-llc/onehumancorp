### Title
[Feature] Generative Promoter (Auto-Social Media Swarm)

**Problem Statement:**
Creating social media content is the #1 reason stores go "dark." Maya the Baker doesn't have time to design Instagram posts or write captions. She needs content to "just happen" whenever she does something in her business.

**Research Report:**
- Marketing Dread (55%) is a top barrier for SMBs (Pain Point #3).
- Shopify Magic (2024) generates product descriptions but not full social calendars.
- Durable provides an "AI Brand Builder" but OHC can automate the *ongoing* promotion.

**Design Doc:**
- **High-Level Architecture:**
    - **Entity Types:** `SocialDraft`, `MediaAsset`, `PromotionCampaign`.
    - **Key Relationships:** `SocialDraft` belongs to a `PromotionCampaign`; `SocialDraft` references one or more `MediaAsset` (generated images/videos).
    - **Integration Points:** Meta Graph API (Instagram/Facebook), TikTok API, OHC Media Storage.
- **Mobile UX Flow (375px First):**
    1. **Trigger:** Push notification: "New cake added! I've drafted 3 Instagram posts for you."
    2. **Review:** Swipeable carousel of drafts (Image suggestion + AI-written caption).
    3. **Action:** Tap "Share Now" or "Schedule for Tuesday."
- **AI Agent Integration Points:** Promoter Agent listens for `ProductAdded` or `JobCompleted` events, calls LLM for captions and Flux/DALL-E for visual ideas.

**Implementation Prompt:**
Implement the "Generative Promoter" agent. This agent should listen for "Business Events" (e.g., new product added, appointment completed) and automatically generate a social media post (image suggestion + caption). These should be queued in the "Agent Activity Feed" for 1-tap approval. Ensure captions match the business "vibe" set during onboarding.

**Priority:** P0
**Estimated Scope:** Large
