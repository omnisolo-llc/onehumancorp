# Title: Auto-Social Media Manager Agent

## Problem Statement
Consistency in social media posting is crucial for small business visibility, but many owners struggle to find the time and inspiration to create content regularly. They know they should post, but "what to post" becomes a paralyzing question.

## Research Report
- **Market Sentiment:** Discussions across r/smallbusiness indicate that marketing and social media management are among the most outsourced tasks due to lack of time and expertise. Existing solutions require users to manually create and schedule every post.
- **Value Proposition:** An agent that proactively generates and schedules relevant, on-brand social media content based on the store's inventory and activity would completely remove the marketing bottleneck.

## Design Doc
- **Core Entity Types:** Store Inventory, Sales Data, Generated Post, Social Channel.
- **Key Relationships:** The agent analyzes Inventory and Sales Data to create a Generated Post, which is then scheduled for a Social Channel.
- **Mobile UX Flow (375px first):**
    1. User connects their social accounts (e.g., Instagram, Facebook).
    2. A toggle is flipped: "Enable Auto-Posting".
    3. The agent surfaces a weekly calendar of proposed posts (with images and captions).
    4. User can swipe to approve, edit, or reject posts.

## Implementation Prompt
- **User-Facing Outcome:** The system autonomously creates social media posts highlighting new products, top sellers, or promotions, and schedules them. The user only needs to approve the suggested content.
- **Critical User Journey (CUJ):**
    1. User connects social media accounts and enables the agent.
    2. Agent generates a batch of posts for the week based on current store data.
    3. User receives a notification to review the weekly content plan.
    4. User approves the plan, and the agent handles the publishing schedule.
- **Acceptance Criteria:**
    - Agent generates varied content (e.g., product spotlights, customer reviews, seasonal promotions).
    - Content includes appropriate images and engaging captions.
    - Seamless integration with social media publishing APIs.

## Priority
P1

## Estimated Scope
Large
