# AI Social Media Auto-Pilot

## Problem Statement

For non-technical small business owners (like Maya the Baker or Priya the Boutique Owner), maintaining an active social media presence is the #1 driver of new business, but it's also the #1 time-sink. They struggle with "blank page syndrome," forgetting to post, or lacking the design skills to create engaging content. Legacy platforms (Shopify/Wix) treat social media as an afterthought, requiring users to install third-party apps or manually write and schedule posts. OHC needs to make social media marketing an invisible, automated process.

## Research Report

Based on our market audit and user pain point analysis:
- **Pain Point Validation:** 65% of micro-business owners cite "marketing and social media" as their biggest operational challenge (Reddit r/smallbusiness survey data).
- **Competitor Gap:** Shopify offers a Sidekick AI chatbot, but it does not autonomously schedule posts based on inventory changes. Wix requires manual intervention via their marketing dashboard.
- **The OHC Advantage:** By leveraging the "Marketing & Advertising" AI Department, OHC can monitor the user's inventory (e.g., a new cake flavor added) and automatically draft, design, and schedule a promotional post across connected platforms without user prompting.

## Design Doc

### High-Level Architecture
The feature sits within the **Marketing & Advertising** AI Department (The Promoter).

- **Entity Types:** `SocialPostDraft`, `SocialIntegration` (Instagram, Facebook), `ContentSchedule`.
- **Key Relationships:** The agent monitors the `Inventory` and `Order` tables. When a trigger event occurs (e.g., `NewProductAdded`, `HighSalesVolume`), a `SocialPostDraft` is generated.
- **Mobile UX Flow (375px first):**
    1. **Notification:** "The Promoter drafted a new Instagram post for your new Lemon Cake. Tap to review."
    2. **Review Screen:** A simple card showing the AI-generated image (or user's product photo) and AI-written caption.
    3. **Action Buttons:** [Approve & Schedule] / [Regenerate Caption] / [Edit Manually].
    4. **Confirmation:** "Scheduled for Tuesday at 10 AM (your busiest engagement time)."
- **AI Integration Points:** Uses the LLM Provider interface (Gemini Pro/GPT-4o) with a specific system prompt for the Marketing Department to ensure the tone matches the business profile.

## Implementation Prompt

**User-Facing Outcome:** The business owner receives a push notification on their phone when the AI has drafted a new social media post based on recent business activity (e.g., adding a new product). They can approve it with one tap.

**Critical User Journey:**
1. A new product is added to the catalog via the OHC app.
2. The Marketing Agent (Promoter) is triggered in the background.
3. The Agent generates an engaging caption and selects the product image.
4. The Agent creates a pending `SocialPostDraft` and notifies the user.
5. The user opens the app, reviews the draft, and taps "Approve".
6. The post is queued for publishing via the Social Integration layer.

**Acceptance Criteria:**
- The AI Agent correctly triggers on the `NewProductAdded` event.
- A draft is generated with an appropriate caption and image.
- The mobile-first UI allows one-tap approval.
- Approved posts are placed in a scheduled state.
- Full E2E test coverage for the drafting and approval flow.

## Priority
P0

## Estimated Scope
Medium