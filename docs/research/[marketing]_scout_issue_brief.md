# Issue Brief: Autonomous Social Media Content Generation Engine

## Problem Statement
Marketing is the most intimidating and time-consuming task for new business owners. They lack the expertise to know what to post, when to post, or how to write engaging copy. As a result, they post low-quality images irregularly, yielding zero engagement and wasted effort.

## Research Report
Marketing agencies are prohibitively expensive for micro-businesses. Existing tools like Buffer or Hootsuite are merely scheduling pipes; they require the user to ideate and create the content. OHC can use AI to generate the content *and* manage the scheduling.

If a baker adds a new cake to their catalog, the OHC system should autonomously propose an Instagram post about it. This transforms the platform from a passive repository into an active growth engine.

## Design Doc
**High-Level Architecture & Entities:**
- `MarketingCampaign` & `SocialPost`: Entities representing planned marketing actions.
- Integrations: Meta Graph API (Instagram/Facebook) for publishing.
- AI Service: Image enhancement and caption generation.

**Mobile UX Flow:**
1. **Trigger Event:** User adds a new product (e.g., "Strawberry Tart").
2. **AI Action:** System works in background.
3. **Notification:** User gets an alert: "I've drafted an Instagram post for your new Strawberry Tart."
4. **Review:** User views the drafted post with generated image framing, caption, and optimized hashtags.
5. **Action:** User taps 'Approve and Post Now' or 'Schedule for Friday'.

**AI Agent Integration Points:**
- Agent drafts compelling copy tailored to the specific social platform (e.g., short for IG, longer for Facebook).
- Agent analyzes historical engagement data to suggest optimal posting times.

## Implementation Prompt
Develop an event-driven marketing automation feature. The system should listen for specific business events (e.g., new product added, inventory restocked) and autonomously generate draft social media posts utilizing AI, presenting them to the user for one-click approval.

**Critical User Journey (CUJ):**
1. Event is triggered (e.g., `ProductCreated` event fired).
2. Listener triggers AI to draft social media copy based on product metadata.
3. Draft post is stored in the database.
4. User reviews draft and clicks approve.
5. System initiates publishing workflow (simulated via mock API).

**Acceptance Criteria:**
- Firing a mock `ProductCreated` event must reliably result in a well-formatted draft social post.
- The AI-generated copy must include relevant hashtags and a call-to-action (CTA).
- The system must provide an interface for the user to review and explicitly approve the post before publishing.

## Priority
P2

## Estimated Scope
Medium
