# [Marketing] Automated 7-Day Social Media Calendar Generator (The Generative Promoter)

## Title
Implement the "Generative Promoter" AI Agent for Automated Social Media Scheduling

## Problem Statement
Small business owners face immense "Marketing Dread" (Ranked #3 in Top 10 SMB Pain Points, 55% frequency). Creating content for social media is the number one reason online stores go "dark" after three months. Founders are not copywriters or graphic designers, yet they are expected to maintain a consistent Instagram, Facebook, or TikTok presence to drive discovery. Manual content creation is a massive time sink that distracts from running the core business.

## Research Report
*   **Competitor Landscape:** Squarespace offers basic social integrations but no generative content. GoDaddy's Airo offers initial AI branding but limited ongoing post generation. Third-party apps on Shopify (like Buffer or Hootsuite integrations) require the user to write the content themselves or pay for expensive standalone AI copywriters.
*   **User Evidence:** Users report that "I built it, but nobody came" is a primary reason for failure. Creating fresh posts for every new product feels like a daunting chore.
*   **OHC Differentiation:** OHC treats AI as a proactive teammate. When a user adds a new product or service, the "Generative Promoter" agent automatically kicks in. It creates a complete 7-day social media content calendar (including images/videos adapted from product photos and AI-written captions) and presents it for 1-tap approval.

## Design Doc
*   **Core Entities:** `Product`, `MarketingCampaign`, `SocialPost`, `AgentTask`.
*   **Key Relationships:** The Marketing Agent listens for `ProductCreated` or `ProductUpdated` events. It uses LLMs to generate `SocialPost` records linked to a `MarketingCampaign`. The campaign is surfaced to the user as an `AgentTask`.
*   **Integration Points:**
    *   **Trigger:** NATS/Redis pub/sub event `ohc.catalog.product_added`.
    *   **Logic:** The Marketing Agent generates 3-5 distinct posts (e.g., "Launch Announcement," "Feature Highlight," "Behind the Scenes/Story") spacing them out over 7 days.
    *   **Output:** Pushes an approval workflow to the Action Feed.
*   **UI/UX Flow (Mobile-First, 375px):**
    1.  User adds a new product (e.g., "Handcrafted Ceramic Mug") and saves.
    2.  An "Action Required" card appears: "Your marketing plan for 'Ceramic Mug' is ready. Review 3 upcoming posts."
    3.  User taps the card to enter a swipeable, full-screen carousel view of the generated posts (mocking Instagram Stories format).
    4.  User can tap "Edit Caption" (opens native keyboard) or simply tap "Approve & Schedule All."
    5.  Posts are marked for publishing via the background worker.

## Implementation Prompt
Build the backend agent generation logic and the frontend approval UI for the Automated Social Media Calendar.
1.  Implement an event listener in the Marketing domain that triggers on product creation.
2.  Create the LLM prompt chain (using Gemini/Claude via the Provider interface) to generate varied social media captions and suggest image cropping/filters based on the product description and main image.
3.  Design the data model to store scheduled `SocialPost` drafts.
4.  Develop the 375px mobile UI for reviewing the generated calendar. The UI should use a swipeable card interface (like TikTok/Reels) to preview how the posts will look. Include a clear, one-tap "Schedule All" call to action.

## Priority
P1

## Estimated Scope
Large
