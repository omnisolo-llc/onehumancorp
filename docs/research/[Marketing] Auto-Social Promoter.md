# [Marketing] Auto-Social Promoter: The Generative Promoter Agent

## Title
[Marketing] Auto-Social Promoter: Proactive Social Media Teammate

## Problem Statement
Small business owners (e.g., Maya the Baker) suffer from "Marketing Dread." They add new products but never find the time to post on Instagram or TikTok. This inconsistency leads to a "dead" online presence and lost sales. They need a teammate who automatically handles social media whenever a business event occurs.

## Research Report
*   **Competitor Status**:
    *   **Shopify Magic**: Can generate email copy or product descriptions, but doesn't proactively schedule social posts across platforms.
    *   **Wix AI**: Has an "AI Image Generator" and "Text Creator," but it's a reactive tool used within the editor.
    *   **Canva/Buffer**: Great tools, but require manual work to transfer data and schedule.
*   **User Pain Point**: 55% of SMBs report "Marketing Dread" as a top barrier. 72% of Shopify users complain about needing a laptop to handle marketing effectively.
*   **Opportunity**: OHC can leapfrog by being the first platform where the agent *notices* a new product and *proactively* drafts a 7-day social calendar for approval.

## Design Doc
*   **Architecture**:
    *   **Trigger**: `ProductAdded` or `InventoryRestocked` event on the OHC Event Mesh.
    *   **Agent**: The Promoter Agent (generative).
    *   **Action**: Drafts Instagram/Facebook captions, generates high-fidelity product lifestyle images (using DALL-E/Midjourney integration), and queues them in the Dashboard "Action Required" feed.
*   **Mobile UX Flow (375px)**:
    1.  Owner adds a "Sourdough Loaf" via phone.
    2.  5 minutes later, a notification appears: "Promoter Agent: I've drafted 3 Instagram posts for your new Loaf. Approve?"
    3.  Owner taps notification, sees a "Glassmorphic" carousel of posts.
    4.  Owner taps "Approve All" or "Edit Vibe."
*   **AI Integration**:
    *   Vision API to analyze product photos.
    *   Generative AI for captions and lifestyle background generation.

## Implementation Prompt
Implement the "Auto-Social Promoter" agentic workflow. When a new product is added, the system must trigger a background agent that generates 3-5 social media post drafts (image + caption) based on the product's metadata and uploaded photos. These drafts should be presented in the user's "Ongoing Wizards" or "Action Feed" for 1-tap approval. Acceptance criteria: adding a product triggers the creation of social drafts; drafts are visible and approvable via the mobile dashboard.

## Priority
P0

## Estimated Scope
Medium
