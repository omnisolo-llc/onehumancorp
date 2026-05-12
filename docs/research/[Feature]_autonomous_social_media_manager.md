# Feature Brief: Autonomous Social Media Manager

## Title
Autonomous Social Media Manager

## Problem Statement
Small business owners, especially solopreneurs like Maya (the baker) or Priya (the boutique owner), struggle with marketing. They know they need to post on social media to generate sales, but creating visually appealing content, writing engaging captions, and remembering to post consistently is overwhelming. As a result, they post inconsistently, losing out on potential revenue and customer engagement. "I don't know what to post on social media to get sales" is a top 3 pain point (35% frequency).

## Research Report
- **Competitor Landscape**: Currently, platforms like Shopify or Wix require users to either manually create and schedule posts or install expensive 3rd-party apps (e.g., Buffer, Hootsuite) which add complexity and cost.
- **User Needs**: Users need a system that does the heavy lifting for them. They want to approve content, not create it from scratch.
- **AI Opportunity**: By leveraging the user's existing OHC product catalog and inventory data, AI can automatically generate relevant, timely content (e.g., "New Item Alert!", "Low Stock Warning!", or "Weekend Sale").

## Design Doc
- **Core Entity**: `SocialCampaignDraft` (linked to `Product` or `Inventory`).
- **Key Relationships**: Integrates directly with the user's connected social media accounts (Instagram, Facebook).
- **Mobile UX Flow (375px First)**:
  1. User receives a push notification: "Your weekly social posts are ready for review!"
  2. User opens the OHC app and sees a carousel of generated posts (image + caption).
  3. User can swipe right to approve/schedule, swipe left to discard, or tap to edit the caption.
  4. Approved posts are automatically scheduled and published at optimal times.
- **AI Agent Integration**: A background agent periodically scans the catalog for triggers (new items, high inventory, stale items) and generates image assets and captions tailored to the specific social platform's best practices.

## Implementation Prompt
Implement an autonomous agent that monitors a user's product catalog and automatically generates draft social media posts. The Critical User Journey (CUJ) starts with the user receiving a notification of drafted posts. The user should be able to review these drafts in a simple mobile-first carousel interface, approve them with a single tap, and have the system handle the scheduling and publishing. The feature should feel like having a dedicated marketing assistant.

## Priority
P1

## Estimated Scope
Medium
