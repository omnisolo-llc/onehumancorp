# AI-Generated 7-Day Social Media Calendar

## Problem Statement
Small business owners know they need to post on social media to drive sales, but they lack the time, copywriting skills, and design expertise to do so consistently. Staring at a blank screen is a major barrier. Traditional platforms offer marketing tools, but they still require the user to generate the content.

## Research Report
- **Pain Point Rank:** Top 2 (Source: Reddit r/ecommerce, YouTube "how to market my business").
- **Competitor Landscape:**
  - **Shopify:** Integrates with social channels but doesn't write the posts for you. "Sidekick" can draft text if prompted, but it's reactive.
  - **GoDaddy Airo:** Creates initial branding, but limited ongoing content generation.
  - **Canva/Buffer:** Good tools, but disconnected from the actual product catalog and require manual effort.
- **Evidence:** "I spend more time trying to figure out what to post on Instagram than I do making my products" (Reddit r/smallbusiness).
- **AI Differentiation:** The "Generative Promoter". When a new product is added, the system automatically creates a week's worth of marketing content.

## Design Doc
- **High-Level Architecture:**
  - **Event Trigger:** A `ProductCreated` event fires.
  - **Agent Action:** The Marketing Agent retrieves the product details, images, and business persona. It generates a 7-day calendar with varied content types (e.g., educational, promotional, behind-the-scenes).
  - **Action Feed:** The calendar is presented in the Dashboard for review.
- **UI Flow (Mobile First - 375px):**
  - Notification: "Your marketing plan for [Product Name] is ready."
  - Carousel View: User swipes through 7 proposed posts (image + caption + optimal posting time).
  - 1-Tap Action: User taps "Approve All" to schedule the posts via integrated social APIs (e.g., Meta Graph API).
- **Integration Points:**
  - LLM Gateway (for caption generation).
  - Image generation/processing service (if altering existing images).
  - Social Media Integration (Meta Graph API for Instagram/Facebook).

## Implementation Prompt
Create an autonomous agent flow that triggers when a new product is added to the catalog. The agent must generate a 7-day social media calendar, including captions and suggested posting times, based on the product description and business profile. Present this calendar in the user's dashboard for 1-tap approval and scheduling.

**Acceptance Criteria:**
- The creation of a new product automatically triggers the generation of a 7-day content calendar.
- The calendar includes 7 distinct posts with unique captions.
- The user can review the posts in a mobile-optimized UI.
- The user can approve the entire calendar with a single action, which simulates scheduling the posts.

## Priority
P1

## Estimated Scope
Large
