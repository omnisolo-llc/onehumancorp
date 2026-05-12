# [Marketing] The Generative Promoter: Autonomous Social Campaigns

## Problem Statement
Consistent marketing is essential for SMB survival, but creating social media content is the #1 reason stores "go dark" after 3 months. The process of taking photos, writing captions, and remembering to post across multiple networks is overwhelming for non-technical founders.

## Research Report
The SMB Pain Point analysis identifies "Marketing Dread" as a critical failure point (affecting 55% of users).
*   **Competitor Failure:** Wix and Shopify offer basic AI text generators, but the user still has to orchestrate the entire campaign manually. Durable generates a site but leaves the owner to figure out marketing.
*   **Opportunity:** Transform content creation from an active chore to a passive approval process triggered by core business events (like adding a new product).

## Design Doc
**High-Level Architecture:**
*   The system listens to the event mesh for a `ProductCreated` or `ProductUpdated` event.
*   The "Generative Promoter" agent is triggered. It retrieves the product details, images, and the overall business "vibe" (brand voice settings).
*   The agent uses multimodal LLMs to generate a multi-day social media campaign:
    *   Day 1: "Just Launched" post (Instagram/Facebook).
    *   Day 3: Highlight a specific feature/benefit.
    *   Day 7: Customer testimonial or usage idea.
*   The entire campaign is grouped into a single "Action Required" item in the user's queue.

**Mobile UX Flow (375px First):**
1.  After adding a new product (e.g., "Lavender Soap"), the owner gets a notification: "Your launch campaign is ready."
2.  The user opens the OHC app and sees a visual carousel of 3 generated social media posts, complete with optimized images (cropped appropriately) and captions with hashtags.
3.  The user can swipe through the preview.
4.  A single large button says `Schedule All`. Tapping it pushes the posts to the OHC social media scheduler (integrating with Meta/Instagram APIs).

## Implementation Prompt
Implement the backend "Generative Promoter" agent that listens for product creation events and autonomously generates a multi-post social media campaign using the product's assets and business context. Build the mobile-first (375px) UI component to preview the entire campaign as a swipeable carousel and approve it with a single tap.
*   **Critical User Journey (CUJ):** The owner adds a new product using their phone camera. Immediately after saving, the dashboard presents a drafted 3-post Instagram campaign for the week. The owner reviews the images and captions, taps "Schedule Launch", and the marketing is handled.
*   **Acceptance Criteria:** The agent must generate distinct, non-repetitive content for each post in the campaign. The mobile UI must allow for easy visual previewing of images and text together. It must successfully queue the approved posts for the social media publisher.

## Priority
P1

## Estimated Scope
Large
