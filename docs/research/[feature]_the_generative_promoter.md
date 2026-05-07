# [feature] The Generative Promoter

## Title
The Generative Promoter (Autonomous Marketing)

## Problem Statement
Small business owners often lack marketing expertise and struggle to consistently create content for social media, leading to poor customer discovery and brand presence ("Marketing Dread").

## Research Report
*   **Gap:** Most founders aren't designers or copywriters, making consistent social media posting a major pain point.
*   **Differentiation:** The agent automatically creates a 7-day social media calendar whenever a new product is added, including images and captions.
*   **Outcome:** Consistent brand presence with zero effort.
*   **Evidence:** "Marketing Dread" is ranked #3 in the Top 10 SMB Pain Points (55% frequency).

## Design Doc
*   **Entities:** MarketingCampaign, SocialPost, MediaAsset.
*   **Key Relationships:** MarketingCampaign belongs to a Product. SocialPost is part of a MarketingCampaign and contains MediaAssets.
*   **UI/UX (Mobile-First 375px):**
    *   After adding a product, the dashboard shows a "Marketing Calendar Ready" notification.
    *   Tapping the notification opens a swipeable view of the next 7 days' proposed posts.
    *   Each post shows the generated image/video and caption.
    *   A single "Approve Campaign" button schedules all posts.
*   **AI Agent Integration:** A background agent listens for `ProductCreated` events. It generates relevant marketing copy and images/video scripts, assembling them into a `MarketingCampaign`.

## Implementation Prompt
Implement a marketing agent that listens for new product additions to the store. Upon detecting a new product, it should generate a multi-day social media campaign consisting of drafted posts (text and image suggestions) and queue this campaign in the user's dashboard for approval. The user interface should allow for easy review and 1-tap scheduling. Do not focus on the specific database schemas or API integrations; emphasize the event-driven campaign generation and the review UI.

## Priority
P1

## Estimated Scope
Medium
