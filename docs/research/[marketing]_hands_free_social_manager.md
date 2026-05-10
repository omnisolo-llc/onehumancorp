# [Marketing] Hands-Free Social Manager (The Promoter Agent)

## Problem Statement

Consistent social media marketing is critical for SMB survival, but it is the #1 reason businesses go "dark" after three months. Business owners (like Priya the boutique owner) lack the time, design skills, and copywriting expertise to maintain a steady cadence of Instagram and Facebook posts. They treat marketing as an exhausting chore rather than a continuous operational habit.

## Research Report

*   **Evidence:** "Marketing Dread" ranks as the #3 top pain point, with 55% of analyzed user complaints highlighting the burden of content creation.
*   **Competitor Gap:** Legacy platforms offer basic social sharing buttons or paid integrations with tools like Buffer or Hootsuite. These tools still require the user to create the content.
*   **Strategic Advantage:** OHC shifts the paradigm from "content management" to "content generation." The Promoter Agent proactively creates the content based on business events, requiring only approval from the user.

## Design Doc

*   **High-Level Architecture:**
    *   **Event Listener:** Subscribes to the Event Mesh for triggers like `ProductAdded`, `InventoryRestocked`, or `SaleStarted`.
    *   **Generative Engine:** Uses LLMs and image generation APIs (if applicable, or selects from pre-approved brand assets) to create a multi-post campaign.
    *   **Scheduling Service:** Interfaces with social platform APIs (Meta Graph API) to schedule approved posts.
*   **Mobile UX Flow (375px First):**
    *   User adds a new product: "Summer Floral Dress."
    *   A few minutes later, a push notification arrives: "Your launch campaign for Summer Floral Dress is ready to review."
    *   User opens the app to see a generated 3-post schedule (Teaser for Tuesday, Launch for Thursday, Reminder for Saturday) complete with captions, hashtags, and suggested images.
    *   User taps "Approve Campaign." The system handles all scheduling and publishing.
*   **AI Integration Points:** The Promoter Agent must be context-aware of the brand's voice (e.g., professional, quirky, localized) stored in the Tenant profile.

## Implementation Prompt

**Critical User Journey (CUJ):**
A business owner uploads a photo and price for a new service or product. Immediately after upload, the OHC app presents a generated "Week 1 Marketing Plan" consisting of three distinct social media posts with varied captions and hashtags. The user reviews the posts in a swipeable mobile view, edits a typo in one caption, and taps "Approve." The posts are automatically scheduled and published to connected social accounts at optimal times.

**Acceptance Criteria:**
*   Implement the Promoter Agent worker that listens for catalog update events.
*   Develop the generative logic to produce varied, contextually relevant social media captions and suggested posting schedules.
*   Create the mobile-first approval UI in Slint, adhering to the Visual Excellence Mandate (Glassmorphism, touch targets >= 44x44px).
*   Ensure the integration with social platforms is robust, handling token expiration and API rate limits gracefully.

**Priority:** P1
**Estimated Scope:** Medium
