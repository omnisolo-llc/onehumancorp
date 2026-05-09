# [growth] AI Social Post Generator

## Problem Statement
"Marketing Dread" is a top-3 pain point for SMBs. Founders are experts in their craft, not in copywriting or graphic design. Creating consistent content for social media is overwhelming, leading to stores going "dark" and losing organic discovery.

## Research Report
*   **Competitor Baseline:** Platforms like Wix and Squarespace offer basic integration with social channels, but creating the actual content is still 100% on the user. Tools like Canva exist but require context-switching and manual work.
*   **User Data:** Creating social media content is cited as the #1 reason online stores fail to gain traction after their initial launch month.
*   **Opportunity:** OHC can transform marketing from a "chore" to an "approval task" using The Generative Promoter agent to automatically build social media calendars.

## Design Doc
*   **Core Entities:** `MarketingCampaign`, `SocialPost`, `ContentCalendar`.
*   **Mobile UX Flow (375px First):**
    1.  **Trigger Event:** User adds a new product or updates stock.
    2.  **Push Notification:** "Your Marketing Agent has drafted 3 new Instagram posts for your new 'Vegan Brownies'."
    3.  **Review Screen:** User sees a swipeable carousel of generated image variations and captions.
    4.  **Schedule:** 1-tap "Approve & Schedule for Tuesday at 9 AM".
*   **AI Integration Point:** An event-driven agent that listens for `ProductCreated` or `InventoryRestocked` events. It uses the product image and description to generate visually appealing social graphics (via image gen APIs if available, or smart templates) and engaging captions, scheduling them into a content calendar.

## Implementation Prompt
Develop the "Generative Promoter" feature for automated social media marketing. The critical user journey starts when a user adds a new product to their store. The system must automatically trigger a background job that generates a proposed social media post (image + caption) promoting the new product. This proposal should appear in the user's action feed. The user must be able to review the generated post and schedule it with a single tap. Acceptance criteria: The agent must successfully generate at least one valid social post proposal triggered strictly by a product addition event, without requiring the user to open a "marketing" tab or write a prompt.

## Priority
P1

## Estimated Scope
Medium
