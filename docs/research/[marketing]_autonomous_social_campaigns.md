# Issue Brief: Autonomous Social Campaigns

## Title
[Marketing] Autonomous Social Campaigns

## Problem Statement
Marketing Dread: Creating content for social media is a major barrier for small business owners. They often forget or lack the time to promote new products, services, or events.

## Research Report
- **Competitor Landscape**: Platforms like Shopify and Wix require the user to manually craft social media posts or use separate, reactive AI tools.
- **User Needs**: Users need marketing to happen automatically as a byproduct of their regular business operations.
- **AI Differentiation**: OHC shifts from reactive assistance to autonomous, background infrastructure. The Autonomous Social Promoter acts as a dedicated marketing employee, instantly leveraging new business events into promotional content across platforms.

## Design Doc
### High-Level Architecture
- **Trigger**: The Backend Event Mesh detects business events (e.g., `ProductAdded`, holiday approaching).
- **Agent Action**: The Autonomous Social Campaigns Agent is invoked.
  - Drafts multiple engaging social media posts tailored to the business's vibe.
  - Generates composite promotional images or uses product photos.
- **UI Flow**: The drafts are pushed to the user's Activity Feed, requesting a simple 1-tap approval.

### Mobile UX Flow (375px First)
1. **Activity Feed**: "You added a new product. We drafted 3 Instagram posts for it."
2. **Review Screen**: User views the suggested images and captions in a native-feeling, mobile-optimized card layout.
3. **Action**: User taps "Approve" to publish immediately or schedule, or "Edit" to make quick tweaks.

## Implementation Prompt
Implement the "Autonomous Social Campaigns" feature. Create an event listener that triggers the Marketing Agent whenever a user adds a new product or service. The agent should automatically generate corresponding social media drafts and surface them in the user's dashboard for 1-tap approval and publishing.

## Priority
P1

## Estimated Scope
Medium
