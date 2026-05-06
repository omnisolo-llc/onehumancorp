# Title: Proactive Social Promoter Agent

## Problem Statement
Small business owners experience deep "Marketing Dread." Creating content for social media is seen as a massive burden and is the #1 reason stores go "dark" after 3 months. Many owners like Priya (boutique owner) want to sell online but lack the time and skill to constantly generate marketing materials to drive traffic, resulting in "Invisible Discovery" where they build a store but nobody visits.

## Research Report
- **Competitor Landscape**:
  - Shopify/Wix: Rely on user-driven marketing or third-party apps for automated social posting.
  - GoDaddy Airo: Provides some AI branding, but limited post-launch ongoing marketing automation.
- **User Pain Points Data**:
  - Marketing Dread is the #3 pain point (55% frequency).
  - Invisible Discovery is the #4 pain point (52% frequency).
  - Owners feel SEO and marketing are "black arts."
- **Sources**: Synthesis of Reddit (r/smallbusiness), Trustpilot, App Store reviews.
- **Opportunity**: OHC can transform marketing from an active chore to a passive approval process via "The Promoter" AI agent.

## Design Doc
- **High-Level Architecture**:
  - Content Generation Engine (Image + Copy).
  - Scheduling / Publishing Event Mesh.
  - "The Promoter" (Marketing Agent).
- **Key Relationships & Integration Points**:
  - Connects to Product Catalog / Inventory updates.
  - Integrates with Meta Graph API for posting to Facebook/Instagram.
- **UI/UX Flow (Mobile 375px First)**:
  - Screen 1: "Marketing Feed" showing generated post ideas for the week.
  - Screen 2: Detail view of a generated post (AI image + AI caption).
  - Screen 3: "Approve & Schedule" button prominently displayed.
  - No complex ad-manager dashboards; strictly a 1-tap approval interface.
- **AI Agent Integration Points**:
  - The Promoter Agent notices a new product added or low sales on an item and proactively generates a promotional post to push to social channels.

## Implementation Prompt
**User-Facing Outcome:** The business owner receives a weekly digest of ready-to-publish social media posts created by AI, turning marketing into a 1-tap approval process.
**Critical User Journey (CUJ):**
1. Owner adds a new product: "Handmade Ceramic Mug."
2. The Promoter Agent generates 3 different social media posts (photo context, catchy caption, hashtags).
3. Owner receives a notification: "Your new mug is ready to be promoted!"
4. Owner reviews the posts on mobile and taps "Approve" for Tuesday and Thursday.
5. Posts go live automatically on the scheduled days.
**Acceptance Criteria:**
- Agent automatically generates text and image content based on inventory/business context.
- User interface allows 1-tap approval of generated content.
- Content successfully publishes to integrated social channels (e.g., Meta).

## Priority
P1

## Estimated Scope
Medium