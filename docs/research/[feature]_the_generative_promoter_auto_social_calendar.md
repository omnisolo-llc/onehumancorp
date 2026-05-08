# [Feature] The Generative Promoter: Auto-Social Calendar

## Title
The Generative Promoter: Auto-Social Calendar

## Problem Statement
Small business owners suffer from "Marketing Dread." Creating consistent content for social media is the #1 reason online stores go "dark" after 3 months. Most founders are not copywriters or designers, and manual creation is too time-consuming.

## Research Report
*   **Competitor Landscape:**
    *   *Shopify/Wix:* Offer basic integrations with social platforms, but the user must still create the content and schedule it manually.
*   **User Pain Points:** 55% of users struggle with marketing dread. It's a massive barrier to sustained sales.
*   **OHC Differentiation:** OHC moves marketing from a manual chore to an automated byproduct of inventory management. When a user does a necessary operational task (adding a product), the marketing happens automatically.

## Design Doc
*   **High-Level Architecture:**
    *   The `Product.Created` event on the Event Mesh triggers the `GenerativePromoterAgent`.
    *   The agent pulls product images, details, and store vibe from the `VectorRepository`.
    *   It uses an LLM to generate 3-5 varied social media captions and potentially overlays text on the images.
    *   The agent creates a `SocialCampaign` entity with scheduled posts.
    *   These appear in the user's Action Feed for 1-tap approval.
*   **UI/UX Flow (Mobile-First 375px):**
    *   After adding a product, a success toast appears: "Product added. Generating marketing plan..."
    *   A new item appears in the Action Feed: "Review 7-Day Social Plan for [Product Name]".
    *   The user taps to see a slick carousel of generated posts (image + caption + scheduled day).
    *   User hits "Approve All" to lock them in.

## Implementation Prompt
Implement the "Generative Promoter" agent. The Critical User Journey (CUJ) is: A user adds a new product to their catalog -> the system autonomously generates a multi-day social media plan (captions and suggested schedule) based on that product -> the plan is presented in the Action Feed for 1-tap approval. Focus on the event trigger and the smooth presentation of the generated content in a mobile-first (375px) UI using OHC design tokens. Do not prescribe specific database schemas or API contracts.

## Priority
P1

## Estimated Scope
Medium
