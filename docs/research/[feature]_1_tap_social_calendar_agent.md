# [feature] The Generative Promoter: 1-Tap Social Calendar Agent

**Priority:** P0
**Estimated Scope:** Large

## Problem Statement
Small business owners (like Maya, a 28-year-old baker) often fail to maintain a consistent social media presence. Creating content, writing captions, and scheduling posts is time-consuming and feels like a chore. As a result, businesses go "dark" and lose out on potential customers and engagement.

## Research Report
Based on an analysis of Reddit (r/smallbusiness, r/ecommerce) and App Store reviews for platforms like Shopify and Wix, "Marketing Dread" is the #3 most common pain point (55% frequency). Users want an easy way to promote their products without needing to be professional marketers or designers. Existing tools treat social media management as a separate, complex task requiring manual effort.

## Design Doc
*   **High-level architecture:**
    *   **Trigger:** A new product is added to the catalog or a specific milestone is reached (e.g., restock, sale).
    *   **Agent (The Generative Promoter):** An autonomous background agent listens for the trigger.
    *   **Generation:** The agent uses the product image, description, and business memory to generate a 7-day social media calendar with suggested images (or image edits) and captions.
    *   **Delivery:** The generated calendar is presented to the user in the OHC mobile app's "Action Feed".
*   **Mobile UX Flow (375px first):**
    *   User opens the OHC app.
    *   "Action Feed" shows a new item: "Review your 7-day social calendar for [Product Name]".
    *   User taps to view a carousel of 7 proposed posts (image + caption).
    *   User can swipe to approve or edit a specific post.
    *   A single "Approve All & Schedule" button allows for 1-tap confirmation.
*   **AI Agent Integration:** The agent needs access to product catalog data, business tone/voice settings, and a scheduling API.

## Implementation Prompt
Implement the "Generative Promoter" agent. The agent should automatically generate a 7-day social media calendar (images and captions) whenever a new product is added. Present this calendar in the user's action feed for a simple, 1-tap approval. The Critical User Journey involves a non-technical user adding a product and immediately receiving a ready-to-publish marketing plan that they can approve with one tap on their mobile device.
