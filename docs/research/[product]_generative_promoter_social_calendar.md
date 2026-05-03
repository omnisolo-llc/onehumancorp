# Issue Brief: Autonomous "Generative Promoter" Agent for Social Media

## Problem Statement
Small business owners frequently experience "Marketing Dread." Creating consistent, engaging content for social media is identified as a primary reason stores go inactive after 3 months. Many owners are not designers or copywriters, and the manual effort required to generate marketing materials for new products is overwhelming.

## Research Report
- **SMB Pain Points:** "Marketing Dread" is ranked #3 among top pain points, affecting an estimated 55% of users.
- **Competitor Gap:** Existing platforms either lack integrated social media automation or require users to manually prompt AI tools to generate content.
- **OHC Opportunity:** By treating AI as a proactive teammate rather than a reactive tool, OHC can automatically generate a comprehensive social media marketing campaign whenever a new product is added to the catalog, removing the marketing burden entirely.

## Design Doc
### High-Level Architecture
- **Event Trigger:** The "Generative Promoter" agent listens to the event mesh for a `ProductAdded` or `ProductUpdated` event.
- **Content Generation:** Upon triggering, the agent uses the product description, images, and target audience metadata to draft a 7-day social media calendar. This includes generating platform-specific captions (Instagram, Facebook, TikTok) and selecting or generating appropriate visual assets.
- **Approval Workflow:** The generated calendar is surfaced in the mobile UI's "Action Feed." The user can review the 7-day plan and approve it with a single tap, after which the system schedules the posts via the respective social media APIs.

### Mobile UX Flow (375px First)
- **Action Feed:** A notification appears: "The Promoter drafted a 7-day social campaign for [New Product Name]."
- **Review Screen:** Tapping the notification opens a swipeable gallery showing each day's planned post (image + caption) for each platform.
- **Action:** A prominent "Approve & Schedule All" button at the bottom. Individual posts can be edited or skipped if desired.

## Implementation Prompt
Implement the "Generative Promoter" background worker. This worker should consume `ProductAdded` events, invoke the LLM provider to generate a structured 7-day social media calendar (captions and asset references), and persist this plan. Build the corresponding Flutter UI to display this generated calendar in the Action Feed, allowing users to review and approve the scheduled posts with a single tap. Ensure the UI is optimized for a 375px display and follows the Premium Token design system.

## Priority
P1

## Estimated Scope
Large
