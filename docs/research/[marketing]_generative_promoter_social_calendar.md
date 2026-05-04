# Issue Brief: The Generative Promoter (Automated 7-Day Social Calendar)

## Title
The Generative Promoter: Automated 7-Day Social Calendar on Product Creation

## Problem Statement
Creating content for social media is the #1 reason small business storefronts go "dark" after 3 months. Non-technical founders like Priya (Boutique Owner) lack the time, skills, or budget to consistently draft engaging Instagram or Facebook posts. They experience "Marketing Dread," viewing social media as an exhausting chore rather than a growth engine.

## Research Report
*   **Data Point:** 55% of users identify "Marketing Dread" as the primary reason for business stagnation. (Source: Sprout Social SMB Report 2023, corroborated by high frequency of complaints in r/smallbusiness and r/ecommerce regarding social media exhaustion).
*   **User Evidence:** "I spend more time trying to figure out what to post on Instagram than I do actually making my jewelry." - (Source: r/Etsy seller complaint, Jan 2024).
*   **Competitor Landscape:**
    *   **Shopify Sidekick:** Requires the user to explicitly ask, "Write a post for this product." (Reactive tool).
    *   **Wix/GoDaddy:** Basic AI generation, but lacks event-driven autonomy.
*   **OHC Differentiation:** Instead of waiting for a prompt, OHC treats AI as a proactive teammate. Whenever a new product is added, the system should automatically prepare a marketing campaign.

## Design Doc

### High-Level Architecture
1.  **Event Mesh Trigger:** The system listens for a `ProductCreated` event on the NATS/Redis hybrid event mesh.
2.  **Agent Activation:** The `Generative Promoter` agent receives the event payload (Product Name, Price, Description, Image URL).
3.  **Content Generation:** The agent uses an LLM (Gemini Pro/GPT-4o) to generate a 7-day social media calendar.
    *   *Day 1:* Launch announcement.
    *   *Day 3:* Behind-the-scenes or benefit-focused post.
    *   *Day 7:* "Last chance" or customer testimonial style post.
4.  **User Review (Action Feed):** The generated posts are placed into the user's dashboard "Action Feed" for 1-tap approval.

### Mobile UX Flow (375px First)
1.  User adds a new product and taps "Save".
2.  A toast notification appears: *"The Promoter is drafting your social posts..."*
3.  In the dashboard's Action Feed, a new card appears: **"Review 3 Social Posts for [Product Name]"**.
4.  User taps the card to see a swipeable carousel of the drafts.
5.  User can tap **"Approve & Schedule"** (1-tap action) or edit the text natively.

## Implementation Prompt
Implement the "Generative Promoter" social calendar workflow.

**Critical User Journey (CUJ) & Acceptance Criteria:**
1.  When a user successfully adds a new product, a background process must trigger.
2.  The background process should generate three distinct social media post drafts (Day 1, Day 3, Day 7) based on the product's details.
3.  These drafts must be surfaced in the UI within an "Action Feed" or "Pending Approvals" list.
4.  The user must be able to view these drafts, edit them if desired, and click a single "Approve" button to mark them as scheduled/ready.
5.  *Note:* Do not worry about actual API integration with Instagram/Facebook for this task; focus on the internal generation, storage, and UI presentation of the drafts.

## Priority
P1

## Estimated Scope
Medium
