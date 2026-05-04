# Issue Brief: Auto-Social Content Promoter

## Problem Statement
"Marketing Dread" ranks #3 in the Top 10 SMB Pain Points (55% frequency). Small business owners often launch a storefront but abandon their business after 3 months because creating consistent social media content is overwhelming and time-consuming. Competitors offer tools to "schedule" posts, but still require the owner to be the copywriter and designer.

## Research Report
- **Competitor Landscape:**
  - **Shopify & Wix:** Rely on third-party app stores for social scheduling (cost creep). They don't generate the content autonomously based on inventory.
  - **GoDaddy (Airo):** Basic logo generation, but weak ongoing social media support.
- **User Pain Point (Evidence):** Small business owners say creating content for social media is the #1 reason stores go "dark" after 3 months.
- **The Leapfrog Opportunity:** Treat social media generation not as a tool, but as a teammate ("The Promoter"). When a new product is added or a specific business event occurs, "The Promoter" should automatically generate and schedule a week's worth of multi-platform posts.

## Design Doc
### High-Level Architecture
- **Trigger Mechanisms:**
  - Event-driven trigger: `ProductAdded`, `Restock`, `NewPromotion`.
  - Cron-based trigger: Weekly run on Sunday evening.
- **Agent Interaction:** "The Promoter" subscribes to these events via the OHC Event Mesh.
- **Content Generation Pipeline:**
  - Pull product images, descriptions, and business "vibe" (system prompt).
  - Generate 3-5 distinct captions/hashtags adapted for Instagram, Facebook, and TikTok.
  - Format images into optimal aspect ratios using a background image processing worker.
- **Approval Flow:** Generate a single UI notification: "Your social media calendar for the week is ready." The user taps to approve or edit the pending queue.

### Implementation Prompt
Implement the "Auto-Social Content Promoter" feature within the "Marketing & Advertising" department. The system should listen for inventory additions or weekly cron events, generate a 7-day social media calendar (images + captions for IG/FB), and place it in a "Pending Approval" queue visible on the mobile 375px dashboard. Ensure the LLM integration uses the `generate_marketing_copy` tool and stores the scheduled posts in the database.

## Priority
P1

## Estimated Scope
Medium
